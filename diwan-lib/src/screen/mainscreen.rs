use termwiz::caps::Capabilities;
use termwiz::input::*;
use termwiz::surface::{Change, Position};
use termwiz::terminal::UnixTerminal;
use termwiz::terminal::{buffered::BufferedTerminal, new_terminal, Terminal};
use termwiz::widgets::*;
use termwiz::Error;

use super::keymap::Modes;
use super::{Keymap, StatusBar};

/// This is a widget for our application
pub struct MainScreen<'a> {
    /// Holds the input text that we wish the widget to display
    pub text: &'a mut String,
    pub mode: Modes,
    pub cursor_pos: usize,
    pub status_bar: StatusBar<'a>,
}

impl<'a> MainScreen<'a> {
    pub fn new_buffered_term() -> Result<BufferedTerminal<UnixTerminal>, Error> {
        let caps = Capabilities::new_from_env()?;
        let term = UnixTerminal::new(caps)?;
        let buffer = BufferedTerminal::new(term)?;
        return Ok(buffer);
    }

    pub fn new_with_widget(
        mut buffer: BufferedTerminal<UnixTerminal>,
        content: &'a mut String,
    ) -> Result<(BufferedTerminal<UnixTerminal>, Self), Error> {
        buffer.terminal().set_raw_mode()?;
        buffer.terminal().enter_alternate_screen()?;
        let status_bar = StatusBar::new("Status: Ready");
        Ok((
            buffer,
            Self {
                text: content,
                mode: Modes::Normal,
                cursor_pos: 0,
                status_bar,
            },
        ))
    }
    pub fn setup_ui(self) -> Ui<'a> {
        let mut ui = Ui::new();
        ui.set_root(self);
        ui
    }

    pub fn main_event_loop(
        mut buf: &mut BufferedTerminal<impl Terminal>,
        mut ui: Ui,
    ) -> Result<(), Error> {
        loop {
            ui.process_event_queue()?;

            // After updating and processing all of the widgets, compose them
            // and render them to the screen.
            if ui.render_to_screen(&mut buf)? {
                // We have more events to process immediately; don't block waiting
                // for input below, but jump to the top of the loop to re-run the
                // updates.
                continue;
            }
            // Compute an optimized delta to apply to the terminal and display it
            buf.flush()?;

            // Wait for user input
            match buf.terminal().poll_input(None) {
                Ok(Some(InputEvent::Resized { rows, cols })) => {
                    // FIXME: this is working around a bug where we don't realize
                    // that we should redraw everything on resize in BufferedTerminal.
                    buf.add_change(Change::ClearScreen(Default::default()));
                    buf.resize(cols, rows);
                }
                Ok(Some(input)) => match input {
                    InputEvent::Key(KeyEvent {
                        key: KeyCode::Char('q'),
                        ..
                    }) => {
                        // Quit the app when escape is pressed
                        Keymap::close_terminal(buf).unwrap();
                    }
                    input => {
                        // Feed input into the Ui
                        ui.queue_event(WidgetEvent::Input(input));
                    }
                },
                Ok(None) => {}
                Err(e) => {
                    print!("{:?}\r\n", e);
                    break;
                }
            }
        }
        Ok(())
    }
}
