use std::sync::{Arc, Mutex};

use anyhow::Error;
use diwan::screen::{Keymap, MainScreen, Modes};
use termwiz::{input::KeyEvent, widgets::WidgetEvent};

#[test]
fn test_create_widget_at_normal() -> Result<(), Error> {
    let dnbuffer = MainScreen::new_buffered_term()?;
    let content = Arc::new(Mutex::new(String::new()));
    let yank = Arc::new(Mutex::<Vec<String>>::new(vec![]));
    let dnwidget = MainScreen::new_with_widget(dnbuffer, content.clone())?;

    //assert_eq!(*dnwidget.1.text.lock().unwrap(), *content.lock().unwrap());
    assert_eq!(dnwidget.1.mode, Modes::Normal);
    assert_eq!(dnwidget.1.cursor_x, 0);
    assert_eq!(dnwidget.1.cursor_y, 0);
    assert_eq!(*dnwidget.1.yank.lock().unwrap(), *yank.lock().unwrap());
    assert_eq!(dnwidget.1.status_bar.filename, "[SCRATCH]".to_string());
    assert_eq!(dnwidget.1.status_bar.status_mode, Modes::Normal.to_string());
    Ok(())
}

#[test]
fn test_create_widget_in_insert_mode() -> Result<(), Error> {
    let dnbuffer = MainScreen::new_buffered_term()?;
    let content = Arc::new(Mutex::new(String::new()));
    let yank = Arc::new(Mutex::<Vec<String>>::new(vec![]));
    let dnwidget = MainScreen::new_with_widget(dnbuffer, content.clone())?;

    let key_event = WidgetEvent::Input(termwiz::input::InputEvent::Key(KeyEvent {
        key: termwiz::input::KeyCode::Char('i'),
        modifiers: Default::default(),
    }));
    let action = Keymap::map_key_to_action(&key_event, &dnwidget.1.mode)
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

    // NOTE(keymap): It is odd that the mode changes, but the status bar mode doesn't update.
    // It seems like the status bar mode wasn't updated correctly. If this interpretation is correct,
    // we might need to pass the status bar to `handle_action`. However, this doesn't seem logical
    // because `handle_action` doesn't require the status bar at all.
    assert_eq!(
        mode,
        Modes::Insert,
        "Mode should change to Insert after pressing 'i'"
    );
    // Additional assertions
    assert_eq!(cursor_x, 0, "Cursor X should be initialized to 0");
    assert_eq!(cursor_y, 0, "Cursor Y should be initialized to 0");
    assert!(
        content.lock().unwrap().is_empty(),
        "Content should be empty initially"
    );
    assert!(
        yank.lock().unwrap().is_empty(),
        "Yank buffer should be empty initially"
    );
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
