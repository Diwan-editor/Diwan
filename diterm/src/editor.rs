#![allow(unused_variables)]

use anyhow::{bail, Result};
use termwiz::{
    caps::Capabilities,
    input::{InputEvent, KeyCode},
    terminal::{new_terminal, Terminal},
};

pub struct Editor;

impl Editor {
    pub fn new() -> Result<Self, anyhow::Error> {
        match Self::init_terminal() {
            Ok(_) => {
                println!("Bye from diwan");
                Ok(Self)
            }

            Err(e) => bail!("{}", e),
        }
    }

    // Function that initializes the editor
    fn init_terminal() -> anyhow::Result<()> {
        let caps = Capabilities::new_from_env()?;
        let mut terminal = new_terminal(caps)?;

        // Enter raw mode
        terminal.enter_alternate_screen()?;
        terminal.set_raw_mode()?;

        // Initial cursor position
        let mut x = 1;
        let mut y = 1;

        loop {
            // Poll inputs
            match terminal.poll_input(None)? {
                Some(InputEvent::Key(key_event)) => {
                    // Extract KeyCode from KeyEvent
                    let key_code = key_event.key;

                    // Move cursor
                    match key_code {
                        KeyCode::RightArrow => x += 1,
                        KeyCode::LeftArrow => {
                            if x > 1 {
                                x -= 1;
                            }
                        }
                        KeyCode::DownArrow => y += 1,
                        KeyCode::UpArrow => {
                            if y > 1 {
                                y -= 1;
                            }
                        }
                        KeyCode::Char('q') => {
                            //println!("Bye from Diwan!");
                            break; // Exit on 'q'
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        terminal.flush()?;
        Self::close_terminal(&mut terminal)?;

        Ok(())
    }

    // Function that closes the editor and cleans the session
    fn close_terminal(terminal: &mut dyn Terminal) -> anyhow::Result<()> {
        terminal.exit_alternate_screen()?;
        terminal.set_cooked_mode()?;
        Ok(())
    }
}
