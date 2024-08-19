use std::process::ExitCode;

use termwiz::{
    input::{InputEvent, KeyCode, KeyEvent},
    surface::Change,
    terminal::{buffered::BufferedTerminal, Terminal},
    widgets::WidgetEvent,
    Error as KeymapError,
};

use super::MainScreen;

pub struct Keymap;

pub enum Modes {
    Normal,
    Insert,
}

pub enum Actions {
    Quit,
    EnterInsertMode,
    EnterNormalMode,
    MoveLeft,
    MoveRight,
    MoveUp,
    MoveDown,
    InsertChar(char),
    DeleteChar,
    NewLine,
}

impl Keymap {
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
    pub fn handle_action(
        action: Actions,
        content: &mut String,
        cursor_pos: &mut usize,
        mode: &mut Modes,
    ) {
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
                if *cursor_pos < content.len() {
                    *cursor_pos += 1;
                }
            }
            Actions::MoveUp | Actions::MoveDown => {
                // Implement vertical movement logic if necessary
            }
            Actions::InsertChar(c) => {
                content.insert(*cursor_pos, c);
                *cursor_pos += 1;
            }
            Actions::DeleteChar => {
                if *cursor_pos > 0 {
                    content.remove(*cursor_pos - 1);
                    *cursor_pos -= 1;
                }
            }
            Actions::NewLine => {
                content.insert(*cursor_pos, '\n');
                *cursor_pos += 1;
            }
        }
    }
    //close and clean terminal

    pub fn close_terminal(buffer: &mut BufferedTerminal<impl Terminal>) -> Result<(), KeymapError> {
        buffer.terminal().exit_alternate_screen()?;
        buffer.terminal().set_cooked_mode()?;
        std::process::exit(0);
    }
}
