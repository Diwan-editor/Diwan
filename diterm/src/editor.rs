use anyhow::{anyhow, Context, Result};

use termwiz::{
    caps::Capabilities,
    input::{InputEvent, KeyCode},
    surface::{Change, Position},
    terminal::{new_terminal, Terminal},
};

/// Enum for action cursor
enum Actions {
    Quit,
    Up,
    Down,
    Left,
    Right,
    EnterMode(Modes),
}

/// Enum for mode
enum Modes {
    NORMAL,
    INSERT,
}

pub struct Editor;

impl Editor {
    pub fn new() -> Result<Self, anyhow::Error> {
        match Self::init_editor() {
            Ok(_) => {
                println!("Bye from diwan");
                Ok(Self)
            }

            Err(e) => Err(anyhow!("{}", e)),
        }
    }

    fn handle_modes(
        terminal: &mut dyn Terminal,
        mode: &Modes,
        ev: Option<InputEvent>,
    ) -> anyhow::Result<Option<Actions>> {
        match mode {
            Modes::NORMAL => Editor::normal_mode(ev.unwrap()),
            Modes::INSERT => Editor::insert_mode(terminal, ev.unwrap()),
        }
    }

    fn normal_mode(ev: InputEvent) -> anyhow::Result<Option<Actions>> {
        match ev {
            InputEvent::Key(ev) => match ev.key {
                KeyCode::Char('q') => Ok(Some(Actions::Quit)),
                KeyCode::Char('h') | KeyCode::LeftArrow => Ok(Some(Actions::Left)),
                KeyCode::Char('j') | KeyCode::DownArrow => Ok(Some(Actions::Down)),
                KeyCode::Char('k') | KeyCode::UpArrow => Ok(Some(Actions::Up)),
                KeyCode::Char('l') | KeyCode::RightArrow => Ok(Some(Actions::Right)),
                KeyCode::Char('i') => Ok(Some(Actions::EnterMode(Modes::INSERT))),
                _ => Ok(None),
            },
            _ => Ok(None),
        }
    }

    fn insert_mode(terminal: &mut dyn Terminal, ev: InputEvent) -> anyhow::Result<Option<Actions>> {
        match ev {
            InputEvent::Key(ev) => match ev.key {
                KeyCode::Escape => Ok(Some(Actions::EnterMode(Modes::NORMAL))),
                KeyCode::Char(c) => {
                    terminal.render(&[Change::Text(format!("{}", c))])?;
                    terminal.flush()?;
                    Ok(None)
                }
                _ => Ok(None),
            },
            _ => Ok(None),
        }
    }

    fn init_editor() -> anyhow::Result<()> {
        // Cursor position (cx, cy)
        let mut cx = 0;
        let mut cy = 0;
        let mut mode = Modes::NORMAL;

        // Get terminal capabilities
        let caps = Capabilities::new_from_env()?;
        let mut terminal = new_terminal(caps)?;

        // Enter raw mode
        terminal.enter_alternate_screen()?;
        terminal.set_raw_mode()?;

        // Main loop for handling input
        loop {
            // Move the cursor to the current position
            terminal.render(&[Change::CursorPosition {
                x: Position::Absolute(cx),
                y: Position::Absolute(cy),
            }])?;
            terminal.flush()?;
            terminal.flush()?;

            // Read an event from the terminal
            let input_event = terminal.poll_input(None).context("Failed to poll input")?;

            match Editor::handle_modes(&mut terminal, &mode, input_event)? {
                Some(action) => match action {
                    Actions::Quit => break,
                    Actions::Up => {
                        if cy > 0 {
                            cy -= 1;
                        }
                    }
                    Actions::Down => {
                        cy += 1;
                    }
                    Actions::Left => {
                        if cx > 0 {
                            cx -= 1;
                        }
                    }
                    Actions::Right => {
                        cx += 1;
                    }
                    Actions::EnterMode(new_mode) => mode = new_mode,
                },
                None => {}
            }
        }

        // Leave the alternate screen and disable raw mode
        Self::close_terminal(&mut terminal)?;

        // Print goodbye message
        println!("Bye from Diwan!");
        Ok(())
    }

    // Function that closes the editor and cleans the session
    fn close_terminal(terminal: &mut dyn Terminal) -> anyhow::Result<()> {
        terminal.exit_alternate_screen()?;
        terminal.set_cooked_mode()?;
        Ok(())
    }
}
