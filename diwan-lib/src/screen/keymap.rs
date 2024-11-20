use std::{
    process::ExitCode,
    sync::{Arc, Mutex},
};

use termwiz::{
    input::{InputEvent, KeyCode, KeyEvent},
    terminal::{buffered::BufferedTerminal, Terminal},
    widgets::WidgetEvent,
    Error as KeymapError,
};

use super::MainScreen;

/// `keymap` for handling key mappings in the Diwan editor.
pub struct Keymap;

/// Represents the modes in which the editor can operate.
#[derive(Debug, PartialEq)]
pub enum Modes {
    /// The normal mode for navigation.
    Normal,
    /// The insert mode for text input.
    Insert,
}

/// Defines the actions that can be performed in the editor.
pub enum Actions {
    /// Exit the application.
    // Quit,
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
    /// Paste a string from the clipboard
    Paste(String),
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
        } else if let WidgetEvent::Input(InputEvent::Paste(pasted_string)) = event {
            Some(Actions::Paste(pasted_string.to_owned()))
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
    /// - `cursor_x`: Mutable reference to the cursor position along the x-axis.
    /// - `cursor_y`: Mutable reference to the cursor position along the y-axis.
    /// - `mode`: Mutable reference to the current mode.
    pub fn handle_action(
        action: Actions,
        content: Arc<Mutex<String>>,
        cursor_x: &mut usize,
        cursor_y: &mut usize,
        mode: &mut Modes,
    ) {
        let mut content_guard = content.lock().unwrap();
        let lines: Vec<&str> = content_guard.lines().collect();

        match action {
            Actions::MoveLeft => Self::move_cursor_left(cursor_x, cursor_y, &lines),
            Actions::MoveRight => Self::move_cursor_right(cursor_x, cursor_y, &lines),
            Actions::MoveUp => Self::move_cursor_up(cursor_x, cursor_y, &lines),
            Actions::MoveDown => Self::move_cursor_down(cursor_x, cursor_y, &lines),
            Actions::NewLine => Self::insert_newline(cursor_x, cursor_y, &mut content_guard),
            Actions::EnterInsertMode => *mode = Modes::Insert,
            Actions::EnterNormalMode => *mode = Modes::Normal,
            Actions::InsertChar(c) => Self::insert_char(c, cursor_x, cursor_y, &mut content_guard),
            Actions::Paste(pasted_string) => {
                Self::insert_string(pasted_string, cursor_x, cursor_y, &mut content_guard)
            }
            Actions::DeleteChar => Self::delete_char(cursor_x, cursor_y, &mut content_guard),
        }
    }
    /// Inserts a character at the cursor position.
    fn insert_char(c: char, cursor_x: &mut usize, cursor_y: &mut usize, content: &mut String) {
        let lines: Vec<&str> = content.lines().collect();
        let byte_pos = Self::get_byte_position(&lines, (*cursor_x, *cursor_y));

        // If the current line starts with a `~`, remove it before inserting the character
        // if lines[*cursor_y].trim() == "~" {
        //     let line_start = Self::get_byte_position(&lines, (0, *cursor_y));
        //     content.replace_range(line_start..line_start + 1, "EOF");
        // }

        // Insert the new character
        content.insert(byte_pos, c);
        *cursor_x += 1;
    }

    /// Inserts a new line at the cursor position.
    fn insert_newline(cursor_x: &mut usize, cursor_y: &mut usize, content: &mut String) {
        let lines: Vec<&str> = content.lines().collect();
        let byte_pos = Self::get_byte_position(&lines, (*cursor_x, *cursor_y));

        content.insert(byte_pos, '\n');
        *cursor_x = 0;
        *cursor_y += 1; // TODO: how character is bein added !
    }

    /// Deletes a character before the cursor.
    fn delete_char(cursor_x: &mut usize, cursor_y: &mut usize, content: &mut String) {
        if *cursor_x == 0 && *cursor_y == 0 {
            return;
        }

        let lines: Vec<&str> = content.lines().collect();
        let byte_pos = Self::get_byte_position(&lines, (*cursor_x, *cursor_y));

        if *cursor_x > 0 {
            content.remove(byte_pos - 1);
            *cursor_x -= 1;
        } else {
            *cursor_y -= 1;
            *cursor_x = lines[*cursor_y].len();
            content.remove(byte_pos - 1);
        }
    }

    /// Moves the cursor up.
    fn move_cursor_up(cursor_x: &mut usize, cursor_y: &mut usize, lines: &[&str]) {
        if *cursor_y > 0 {
            *cursor_y -= 1;
            *cursor_x = (*cursor_x).min(lines[*cursor_y].len());
        }
    }

    /// Moves the cursor down.
    fn move_cursor_down(cursor_x: &mut usize, cursor_y: &mut usize, lines: &[&str]) {
        if let (Some(current_line), _) = (lines.get(*cursor_y), lines.get(*cursor_y + 1)) {
            if *cursor_y < lines.len() - 1 {
                *cursor_y += 1;
                *cursor_x = (*cursor_x).min(lines[*cursor_y].len());
            }
        }
    }

    /// Moves the cursor left.
    fn move_cursor_left(cursor_x: &mut usize, cursor_y: &mut usize, lines: &[&str]) {
        if *cursor_x > 0 {
            *cursor_x -= 1;
        } else if *cursor_y > 0 {
            *cursor_y -= 1;
            *cursor_x = lines[*cursor_y].len();
        }
    }

    /// Moves the cursor right.
    fn move_cursor_right(cursor_x: &mut usize, cursor_y: &mut usize, lines: &[&str]) {
        if let (Some(current_line), _) = (lines.get(*cursor_y), lines.get(*cursor_y + 1)) {
            if *cursor_x < lines[*cursor_y].len() {
                *cursor_x += 1;
            } else if *cursor_y < lines.len() - 1 {
                *cursor_y += 1;
                *cursor_x = 0;
            }
        }
    }

    /// Returns the byte position in the content string based on the current cursor position.
    fn get_byte_position(lines: &[&str], cursor_pos: (usize, usize)) -> usize {
        let line_start: usize = lines[..cursor_pos.1].iter().map(|l| l.len() + 1).sum(); // Sum of lengths of lines before the current line
        line_start + cursor_pos.0
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

    fn insert_string(
        pasted_string: String,
        cursor_x: &mut usize,
        cursor_y: &mut usize,
        content: &mut std::sync::MutexGuard<'_, String>,
    ) {
        let lines: Vec<&str> = content.lines().collect();
        let byte_pos = Self::get_byte_position(&lines, (*cursor_x, *cursor_y));

        // Insert the new character
        content.insert_str(byte_pos, pasted_string.as_str());
        *cursor_x += pasted_string.len();
    }
}
