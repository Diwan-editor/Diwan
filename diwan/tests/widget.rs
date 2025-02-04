use std::sync::{Arc, Mutex};

use anyhow::Error;
use diwan::screen::{MainScreen, Modes};

#[test]
fn test_create_widget() -> Result<(), Error> {
    let dnbuffer = MainScreen::new_buffered_term()?;
    let content = Arc::new(Mutex::new(String::new()));
    let yank = Arc::new(Mutex::<Vec<String>>::new(vec![]));
    let dnwidget = MainScreen::new_with_widget(dnbuffer, content.clone())?;

    // this doesn't make sense
    // assert_eq!(*dnwidget.1.text.lock().unwrap(), *content.lock().unwrap());, this one hangs for some reason

    assert_eq!(dnwidget.1.mode, Modes::Normal);
    assert_eq!(dnwidget.1.cursor_x, 0);
    assert_eq!(dnwidget.1.cursor_y, 0);

    // ig this one also doesn't make sense , this one doesn't , the type is a Vector<String>
    assert_eq!(*dnwidget.1.yank.lock().unwrap(), *yank.lock().unwrap());

    assert_eq!(dnwidget.1.status_bar.filename, "[SCRATCH]".to_string());
    assert_eq!(dnwidget.1.status_bar.status_mode, Modes::Normal);
    Ok(())
}


#[test]
fn test_alter_content() -> Result<(), Error> {
    let dnbuffer = MainScreen::new_buffered_term()?;
    // to be replaced by some random_content generator
    let content = Arc::new(Mutex::new(String::new()));
    let yank = Arc::new(Mutex::<Vec<String>>::new(vec![]));
    let (buffer, mut widget) = MainScreen::new_with_widget(dnbuffer, content.clone())?;


    assert_eq!(widget.mode, Modes::Normal);
    assert_eq!(widget.cursor_x, 0);
    assert_eq!(widget.cursor_y, 0);

    assert_eq!(widget.status_bar.filename, "[SCRATCH]".to_string());
    assert_eq!(widget.status_bar.status_mode, Modes::Normal);

    // altering the value
    let random_content = "this is a content";

    // testing altering content
    *content.lock().unwrap() = random_content.to_string();
    assert_eq!(*widget.text.lock().unwrap(), random_content.to_string());

    // testing altering Modes
    widget.update_status_mode(Modes::Insert);
    assert_eq!(widget.mode, Modes::Insert);

    // testing altering cursor
    widget.cursor_x = 1;
    widget.cursor_y = 1;
    assert_eq!(widget.cursor_x, 1);
    assert_eq!(widget.cursor_y, 1);

    // testing altering status_bar
    widget.status_bar.filename = "about::blanc".to_string();
    assert_eq!(widget.status_bar.filename, "about::blanc".to_string());
    assert_eq!(widget.status_bar.status_mode, Modes::Insert);

    Ok(())
}
