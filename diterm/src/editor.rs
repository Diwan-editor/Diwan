use anyhow::{Context, Result};
use std::collections::VecDeque;
use termwiz::{
    caps::Capabilities,
    color::ColorAttribute,
    input::{InputEvent, KeyCode},
    surface::{Change, CursorShape, Position},
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
    Normal,
    Insert,
}

pub struct Editor {
    content: VecDeque<VecDeque<char>>,
    cursor_x: usize,
    cursor_y: usize,
}

impl Editor {
    /// Creates a new instance of the editor and initializes it
    pub fn new() -> Result<Self, anyhow::Error> {
        let mut editor = Self {
            content: VecDeque::from([VecDeque::new()]),
            cursor_x: 0,
            cursor_y: 0,
        };
        editor.init_editor()?;
        Ok(editor)
    }

    /// Maps `KeyCode` to `Actions`
    fn map_key_to_action(key: KeyCode, mode: &Modes) -> Option<Actions> {
        match mode {
            Modes::Normal => match key {
                KeyCode::Char('q') => Some(Actions::Quit),
                KeyCode::Char('h') | KeyCode::LeftArrow => Some(Actions::Left),
                KeyCode::Char('j') | KeyCode::DownArrow => Some(Actions::Down),
                KeyCode::Char('k') | KeyCode::UpArrow => Some(Actions::Up),
                KeyCode::Char('l') | KeyCode::RightArrow => Some(Actions::Right),
                KeyCode::Char('i') => Some(Actions::EnterMode(Modes::Insert)),
                _ => None,
            },
            Modes::Insert => match key {
                KeyCode::Escape => Some(Actions::EnterMode(Modes::Normal)),
                KeyCode::LeftArrow => Some(Actions::Left),
                KeyCode::DownArrow => Some(Actions::Down),
                KeyCode::UpArrow => Some(Actions::Up),
                KeyCode::RightArrow => Some(Actions::Right),
                _ => None,
            },
        }
    }

    /// Handles mode-specific input events
    ///
    /// # Arguments
    ///
    /// * `terminal` - Mutable reference to the terminal
    /// * `mode` - Current mode of the editor
    /// * `ev` - Input event to handle
    ///
    /// # Returns
    ///
    /// * `Option<Actions>` - The action to be performed based on the input event
    fn handle_modes(
        &mut self,
        terminal: &mut dyn Terminal,
        mode: &Modes,
        ev: InputEvent,
    ) -> anyhow::Result<Option<Actions>> {
        match ev {
            InputEvent::Key(key) => {
                if let Some(action) = Self::map_key_to_action(key.key, mode) {
                    return Ok(Some(action));
                }

                match mode {
                    Modes::Normal => Ok(None),
                    Modes::Insert => match key.key {
                        KeyCode::Char(c) => {
                            self.insert_char(c);
                            self.render_content(terminal)?;
                            Ok(None)
                        }
                        KeyCode::Backspace => {
                            self.delete_char();
                            self.render_content(terminal)?;
                            Ok(None)
                        }
                        KeyCode::Enter => {
                            self.insert_newline();
                            self.render_content(terminal)?;
                            Ok(None)
                        }
                        _ => Ok(None),
                    },
                }
            }
            _ => Ok(None),
        }
    }

    /// Inserts a character at the current cursor position
    ///
    /// # Arguments
    ///
    /// * `c` - Character to insert
    fn insert_char(&mut self, c: char) {
        self.content[self.cursor_y].insert(self.cursor_x, c);
        self.cursor_x += 1;
    }

    /// Deletes a character at the current cursor position
    fn delete_char(&mut self) {
        if self.cursor_x > 0 {
            self.cursor_x -= 1;
            self.content[self.cursor_y].remove(self.cursor_x);
        } else if self.cursor_y > 0 {
            let line = self.content.remove(self.cursor_y).unwrap();
            self.cursor_y -= 1;
            self.cursor_x = self.content[self.cursor_y].len();
            self.content[self.cursor_y].extend(line);
        }
    }

    /// Inserts a new line at the current cursor position
    fn insert_newline(&mut self) {
        let remainder: VecDeque<char> = self.content[self.cursor_y].split_off(self.cursor_x);
        self.content.insert(self.cursor_y + 1, remainder);
        self.cursor_y += 1;
        self.cursor_x = 0;
    }

    /// Renders the content of the editor to the terminal
    ///
    /// # Arguments
    ///
    /// * `terminal` - Mutable reference to the terminal
    ///
    /// # Returns
    ///
    /// * `Result<()>` - Result indicating success or failure
    fn render_content(&self, terminal: &mut dyn Terminal) -> anyhow::Result<()> {
        terminal.render(&[Change::ClearScreen(ColorAttribute::Default)])?;
        for (y, line) in self.content.iter().enumerate() {
            terminal.render(&[
                Change::CursorPosition {
                    x: Position::Absolute(0),
                    y: Position::Absolute((y as u16).into()),
                },
                Change::Text(line.iter().collect::<String>()),
            ])?;
        }
        terminal.render(&[Change::CursorPosition {
            x: Position::Absolute((self.cursor_x as u16).into()),
            y: Position::Absolute((self.cursor_y as u16).into()),
        }])?;
        terminal.flush()?;
        Ok(())
    }

    /// Initializes the editor and starts the main loop
    ///
    /// # Returns
    ///
    /// * `Result<()>` - Result indicating success or failure
    fn init_editor(&mut self) -> anyhow::Result<()> {
        let mut mode = Modes::Normal;

        // Get terminal capabilities
        let caps = Capabilities::new_from_env()?;
        let mut terminal = new_terminal(caps)?;

        // Enter raw mode
        terminal.enter_alternate_screen()?;
        terminal.set_raw_mode()?;

        self.render_content(&mut terminal)?;

        // Main loop for handling input
        loop {
            // Read an event from the terminal
            let input_event = terminal.poll_input(None).context("Failed to poll input")?;

            if let Some(action) = self.handle_modes(
                &mut terminal,
                &mode,
                input_event.context("can't get context")?,
            )? {
                match action {
                    Actions::Quit => break,
                    Actions::Up => self.move_cursor_up(),
                    Actions::Down => self.move_cursor_down(),
                    Actions::Left => self.move_cursor_left(),
                    Actions::Right => self.move_cursor_right(),
                    Actions::EnterMode(new_mode) => {
                        mode = new_mode;
                        match mode {
                            Modes::Normal => {
                                terminal.render(&[Change::CursorShape(CursorShape::Default)])?
                            }
                            Modes::Insert => {
                                terminal.render(&[Change::CursorShape(CursorShape::BlinkingBar)])?
                            }
                        }
                        terminal.flush()?;
                    }
                }
            }

            self.render_content(&mut terminal)?;
        }

        // Leave the alternate screen and disable raw mode
        self.close_terminal(&mut terminal)?;

        // Print goodbye message
        println!("Bye from Diwan!");
        Ok(())
    }

    fn move_cursor_up(&mut self) {
        if self.cursor_y > 0 {
            self.cursor_y -= 1;
            self.cursor_x = self.cursor_x.min(self.content[self.cursor_y].len());
        }
    }

    fn move_cursor_down(&mut self) {
        if self.cursor_y < self.content.len() - 1 {
            self.cursor_y += 1;
            self.cursor_x = self.cursor_x.min(self.content[self.cursor_y].len());
        }
    }

    fn move_cursor_left(&mut self) {
        if self.cursor_x > 0 {
            self.cursor_x -= 1;
        } else if self.cursor_y > 0 {
            self.cursor_y -= 1;
            self.cursor_x = self.content[self.cursor_y].len();
        }
    }

    fn move_cursor_right(&mut self) {
        if self.cursor_x < self.content[self.cursor_y].len() {
            self.cursor_x += 1;
        } else if self.cursor_y < self.content.len() - 1 {
            self.cursor_y += 1;
            self.cursor_x = 0;
        }
    }

    // Function that closes the editor and cleans the session
    fn close_terminal(&self, terminal: &mut dyn Terminal) -> anyhow::Result<()> {
        terminal.exit_alternate_screen()?;
        terminal.set_cooked_mode()?;
        Ok(())
    }
}
