use termwiz::terminal::Terminal;
use crate::screen::MyTerminal;




impl Terminal for MyTerminal {
    fn set_raw_mode(&mut self) -> Result<()> {
        let mut raw = self.write.get_termios()?;
        cfmakeraw(&mut raw);
        self.write
            .set_termios(&raw, SetAttributeWhen::AfterDrainOutputQueuePurgeInputQueue)
            .context("failed to set raw mode")?;

        macro_rules! decset {
            ($variant:ident) => {
                write!(
                    self.write,
                    "{}",
                    CSI::Mode(Mode::SetDecPrivateMode(DecPrivateMode::Code(
                        DecPrivateModeCode::$variant
                    )))
                )?;
            };
        }

        if self.caps.bracketed_paste() {
            decset!(BracketedPaste);
        }
        if self.caps.mouse_reporting() {
            decset!(AnyEventMouse);
            decset!(SGRMouse);
        }
        self.write.modify_other_keys(2)?;
        self.write.flush()?;

        Ok(())
    }

    fn set_cooked_mode(&mut self) -> Result<()> {
        self.write.modify_other_keys(1)?;
        self.write
            .set_termios(&self.saved_termios, SetAttributeWhen::Now)
    }

    fn enter_alternate_screen(&mut self) -> Result<()> {
        if !self.in_alternate_screen {
            write!(
                self.write,
                "{}",
                CSI::Mode(Mode::SetDecPrivateMode(DecPrivateMode::Code(
                    DecPrivateModeCode::ClearAndEnableAlternateScreen
                )))
            )?;
            self.in_alternate_screen = true;
        }
        Ok(())
    }

    fn exit_alternate_screen(&mut self) -> Result<()> {
        if self.in_alternate_screen {
            write!(
                self.write,
                "{}",
                CSI::Mode(Mode::ResetDecPrivateMode(DecPrivateMode::Code(
                    DecPrivateModeCode::ClearAndEnableAlternateScreen
                )))
            )?;
            self.in_alternate_screen = false;
        }
        Ok(())
    }

    fn get_screen_size(&mut self) -> Result<ScreenSize> {
        let size = self.write.get_size()?;
        Ok(ScreenSize {
            rows: cast(size.ws_row)?,
            cols: cast(size.ws_col)?,
            xpixel: cast(size.ws_xpixel)?,
            ypixel: cast(size.ws_ypixel)?,
        })
    }

    fn probe_capabilities(&mut self) -> Option<ProbeCapabilities> {
        Some(ProbeCapabilities::new(&mut self.read, &mut self.write))
    }

    fn set_screen_size(&mut self, size: ScreenSize) -> Result<()> {
        let size = winsize {
            ws_row: cast(size.rows)?,
            ws_col: cast(size.cols)?,
            ws_xpixel: cast(size.xpixel)?,
            ws_ypixel: cast(size.ypixel)?,
        };

        self.write.set_size(size)
    }
    fn render(&mut self, changes: &[Change]) -> Result<()> {
        self.renderer.render_to(changes, &mut self.write)
    }
    fn flush(&mut self) -> Result<()> {
        self.write.flush().context("flush failed")
    }

    fn poll_input(&mut self, wait: Option<Duration>) -> Result<Option<InputEvent>> {
        if let Some(event) = self.input_queue.pop_front() {
            return Ok(Some(event));
        }

        // Some unfortunately verbose code here.  In order to safely hook and process
        // SIGWINCH we need to use the self-pipe trick to deliver signals to a pipe
        // so that we can use poll(2) to wait for events on both the tty input and
        // the sigwinch pipe at the same time.  In theory we could do away with this
        // and use sigaction to register SIGWINCH without SA_RESTART set; that way
        // we could do a blocking read and have it get EINTR on a resize.
        // Doing such a thing may introduce more problems for other components in
        // the rust crate ecosystem if they're not ready to deal with EINTR, so
        // we opt to take on the complexity here to make things overall easier to
        // integrate.

        let mut pfd = [
            pollfd {
                fd: self.sigwinch_pipe.as_raw_fd(),
                events: POLLIN,
                revents: 0,
            },
            pollfd {
                fd: self.read.fd.as_raw_fd(),
                events: POLLIN,
                revents: 0,
            },
            pollfd {
                fd: self.wake_pipe.as_raw_fd(),
                events: POLLIN,
                revents: 0,
            },
        ];

        if let Err(err) = poll(&mut pfd, wait) {
            return match err
                .source()
                .ok_or_else(|| anyhow::anyhow!("error has no source! {:#}", err))?
                .downcast_ref::<std::io::Error>()
            {
                Some(err) => {
                    if err.kind() == ErrorKind::Interrupted {
                        // SIGWINCH may have been the source of the interrupt.
                        // Check for that now so that we reduce the latency of
                        // processing the resize
                        if let Some(resize) = self.caught_sigwinch()? {
                            Ok(Some(resize))
                        } else {
                            Ok(None)
                        }
                    } else {
                        bail!("poll(2) error: {}", err)
                    }
                }
                None => bail!("poll(2) error: {}", err),
            };
        };

        if pfd[0].revents != 0 {
            // SIGWINCH received via our pipe?
            if let Some(resize) = self.caught_sigwinch()? {
                return Ok(Some(resize));
            }
        }

        if pfd[1].revents != 0 {
            let mut buf = [0u8; 64];
            match self.read.read(&mut buf) {
                Ok(n) => {
                    let input_queue = &mut self.input_queue;
                    self.input_parser.parse(
                        &buf[0..n],
                        |evt| input_queue.push_back(evt),
                        n == buf.len(),
                    );
                    return Ok(self.input_queue.pop_front());
                }
                Err(ref e)
                    if e.kind() == ErrorKind::WouldBlock || e.kind() == ErrorKind::Interrupted => {}
                Err(e) => bail!("failed to read input {}", e),
            }
        }

        if pfd[2].revents != 0 {
            let mut buf = [0u8; 64];
            if let Ok(n) = self.wake_pipe.read(&mut buf) {
                if n > 0 {
                    return Ok(Some(InputEvent::Wake));
                }
            }
        }

        Ok(None)
    }

    fn waker(&self) -> UnixTerminalWaker {
        UnixTerminalWaker {
            pipe: self.wake_pipe_write.clone(),
        }
    }
}
