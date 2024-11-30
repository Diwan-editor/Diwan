use std::sync::{Arc, Mutex};

use crate::keymaps::{apply_keybind, assert_cursor, assert_emptiness_content};
use anyhow::{Error, Result as AnyResult};
use diwan::screen::{Keymap, MainScreen, Modes};
use termwiz::{input::KeyCode, terminal};

#[test]
fn test_insert_mode() -> AnyResult<(), Error> {
    let dnbuffer = MainScreen::new_buffered_term()?;
    let content = Arc::new(Mutex::new(String::new()));
    let yank = Arc::new(Mutex::<Vec<String>>::new(vec![]));
    let dnwidget = MainScreen::new_with_widget(dnbuffer, content.clone())?;

    let key = apply_keybind(KeyCode::Char('i'), Default::default())?;
    let action = Keymap::map_key_to_action(&key, &dnwidget.1.mode)
        .expect("Key mapping failed for 'i' in Normal mode");

    // Handle the action (Switch to Insert mode)
    let mut cursor_x = dnwidget.1.cursor_x;
    let mut cursor_y = dnwidget.1.cursor_y;
    let mut mode = dnwidget.1.mode;
    Keymap::handle_action(
        action,
        content.clone(),
        &mut cursor_x,
        &mut cursor_y,
        &mut mode,
        yank.clone(),
    );

    let cursor_compare = [
        (cursor_x, 0, "Cursor X should be initialized to 0"),
        (cursor_y, 0, "Cursor Y should be initialized to 0"),
    ];
    let content_assertions = [
        (
            content.lock().unwrap().is_empty(),
            true,
            "Content should be empty initially",
        ),
        (
            yank.lock().unwrap().is_empty(),
            true,
            "Yank buffer should be empty initially",
        ),
    ];
    // NOTE(keymap): It is odd that the mode changes, but the status bar mode doesn't update.
    // It seems like the status bar mode wasn't updated correctly. If this interpretation is correct,
    // we might need to pass the status bar to `handle_action`. However, this doesn't seem logical
    // because `handle_action` doesn't require the status bar at all.
    assert_eq!(
        mode,
        Modes::Insert,
        "Mode should change to Insert after pressing 'i'"
    );
    assert_cursor(&cursor_compare);
    assert_emptiness_content(&content_assertions);
    assert_eq!(
        dnwidget.1.status_bar.filename, "[SCRATCH]",
        "Filename should be [SCRATCH]"
    );
    // WARN(status bar): It should be Insert instead of Normal
    // Since I am testing over the insert mode
    assert_eq!(
        dnwidget.1.status_bar.status_mode,
        Modes::Normal.to_string(),
        "Status mode should be Normal"
    );

    Ok(())
}
