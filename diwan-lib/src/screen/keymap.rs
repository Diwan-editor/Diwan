use std::{
    process::ExitCode,
    sync::{Arc, Mutex},
};

use termwiz::{
    input::{InputEvent, KeyCode, KeyEvent},
    surface::Change,
    terminal::{buffered::BufferedTerminal, Terminal},
    widgets::WidgetEvent,
    Error as KeymapError,
};

use super::MainScreen;

/// `keymap` for handling key mappings in Diwan editor.
pub struct Keymap;

/// Represents the modes in which the editor can operate.
#[derive(Debug)]
pub enum Modes {
    /// The normal mode for navigation.
    Normal,
    /// The insert mode for text input.
    Insert,
}

/// Defines the actions that can be performed in the editor.
pub enum Actions {
    /// Exit the application.
    Quit,
    /// Enter insert mode.
    EnterInsertMode,
    /// Enter normal mode.
    EnterNormalMode,
    /// Move the cursor left.
    MoveLeft,
    /// Move the cursor right.
    MoveRight,
    /// Move the cursor up.
    MoveUp,
    /// Move the cursor down.
    MoveDown,
    /// Insert a character at the cursor.
    InsertChar(char),
    /// Delete a character before the cursor.
    DeleteChar,
    /// Insert a newline character.
    NewLine,
}

impl Keymap {
    /// Maps a key event to a corresponding action based on the current mode.
    ///
    /// # Parameters
    ///
    /// - `event`: The input event to map.
    /// - `mode`: The current mode of the editor.
    ///
    /// # Returns
    ///
    /// An `Option<Actions>` indicating the mapped action, or `None` if no action is mapped.
    pub fn map_key_to_action(event: &WidgetEvent, mode: &Modes) -> Option<Actions> {
        if let WidgetEvent::Input(InputEvent::Key(KeyEvent { key, .. })) = event {
            match mode {
                Modes::Normal => match key {
                    // KeyCode::Char('q') => Some(Actions::Quit),
                    KeyCode::Char('h') | KeyCode::LeftArrow => Some(Actions::MoveLeft),
                    KeyCode::Char('j') | KeyCode::DownArrow => Some(Actions::MoveDown),
                    KeyCode::Char('k') | KeyCode::UpArrow => Some(Actions::MoveUp),
                    KeyCode::Char('l') | KeyCode::RightArrow => Some(Actions::MoveRight),
                    KeyCode::Char('i') => Some(Actions::EnterInsertMode),
                    _ => None,
                },
                Modes::Insert => match key {
                    KeyCode::Escape => Some(Actions::EnterNormalMode),
                    KeyCode::LeftArrow => Some(Actions::MoveLeft),
                    KeyCode::DownArrow => Some(Actions::MoveDown),
                    KeyCode::UpArrow => Some(Actions::MoveUp),
                    KeyCode::RightArrow => Some(Actions::MoveRight),
                    KeyCode::Char(c) => Some(Actions::InsertChar(*c)),
                    KeyCode::Backspace => Some(Actions::DeleteChar),
                    KeyCode::Enter => Some(Actions::NewLine),
                    _ => None,
                },
            }
        } else {
            None
        }
    }

    /// Handles the specified action by updating the content and cursor position.
    ///
    /// # Parameters
    ///
    /// - `action`: The action to perform.
    /// - `content`: Shared mutable string content.
    /// - `cursor_pos`: Mutable reference to the cursor position.
    /// - `mode`: Mutable reference to the current mode.
    pub fn handle_action(
        action: Actions,
        content: Arc<Mutex<String>>,
        cursor_pos: &mut usize,
        mode: &mut Modes,
    ) {
        let mut content_guard = content.lock().unwrap();
        match action {
            Actions::Quit => {
                println!("Exiting...");
                std::process::exit(0)
            }
            Actions::EnterInsertMode => *mode = Modes::Insert,
            Actions::EnterNormalMode => *mode = Modes::Normal,
            Actions::MoveLeft => {
                if *cursor_pos > 0 {
                    *cursor_pos -= 1;
                }
            }
            Actions::MoveRight => {
                if *cursor_pos < content_guard.len() {
                    *cursor_pos += 1;
                }
            }
            Actions::MoveUp | Actions::MoveDown => {
                // Implement vertical movement logic if necessary
            }
            Actions::InsertChar(c) => {
                content_guard.insert(*cursor_pos, c);
                *cursor_pos += 1;
            }
            Actions::DeleteChar => {
                if *cursor_pos > 0 {
                    content_guard.remove(*cursor_pos - 1);
                    *cursor_pos -= 1;
                }
            }
            Actions::NewLine => {
                content_guard.insert(*cursor_pos, '\n');
                *cursor_pos += 1;
            }
        }
    }

    /// Cleans up and closes the terminal.
    ///
    /// # Parameters
    ///
    /// - `buffer`: A mutable reference to a buffered terminal.
    ///
    /// # Returns
    ///
    /// A `Result<(), KeymapError>` indicating success or failure.
    pub fn close_terminal(buffer: &mut BufferedTerminal<impl Terminal>) -> Result<(), KeymapError> {
        buffer.terminal().exit_alternate_screen()?;
        buffer.terminal().flush()?;
        Ok(())
    }
}
