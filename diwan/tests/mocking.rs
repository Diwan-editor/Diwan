// mocking user input
// go_right -> increase the cursor.x by 1 ( rouding in case the max width is reached )
// go_left -> decrease the cursor.x by 1 ( rouding in case the max width is 0 )
use anyhow::Error;
use diwan::core::utils::*;
use diwan::screen::DWidget;
use diwan::screen::Modes;
use std::sync::Arc;
use std::sync::Mutex;
use termwiz::terminal::{buffered::BufferedTerminal, UnixTerminal};

use std::io;
use std::io::Write;

struct MockTerminal();

impl MockTerminal {
    pub fn write_char(ch: char) -> Result<(), Error> {
        let mut stdout = io::stdout().lock();
        write!(&mut stdout, "{}", ch)?;

        Ok(())
    }
}

pub fn bootstrap_diwan() -> Result<
    (
        Arc<Mutex<BufferedTerminal<UnixTerminal>>>,
        Arc<Mutex<DWidget>>,
    ),
    Error,
> {
    // init the a new buffered terminal
    let dnbuffer = new_buffered_term()?;
    // init a mutex string for our poet :)
    // let typed_text = Arc::new(Mutex::new(String::new())); // probably won't need it anymore
    // in simplified lang: combine the initialized bufer and content String
    // and return a mutable buffer and main_screen for displaying everingthing
    let (buffer, main_screen) = new_with_widget(dnbuffer)?;

    let my_buffer = Arc::new(Mutex::new(buffer));
    let my_main_screen = Arc::new(Mutex::new(main_screen));
    // set up the ui
    let _ui = Arc::new(Mutex::new(setup_ui()));

    // how to start the thread and still returns the same datastructure ?
    // let _handler = std::thread::spawn(move || {
    //     // clone the shared ui
    //     main_event_loop(&mut my_buffer.lock().unwrap(), &mut ui.lock().unwrap())
    //         .unwrap();
    // });

    Ok((my_buffer, my_main_screen))
}

#[test]
fn go_right() {
    let (_buffer, widget) = bootstrap_diwan().unwrap();
    assert_eq!(widget.lock().unwrap().mode, Modes::Normal);
    let _ = MockTerminal::write_char('i');
    assert_eq!(widget.lock().unwrap().mode, Modes::Insert);
}
