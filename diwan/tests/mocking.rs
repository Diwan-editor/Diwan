// mocking user input
// go_right -> increase the cursor.x by 1 ( rouding in case the max width is reached )
// go_left -> decrease the cursor.x by 1 ( rouding in case the max width is 0 )
use anyhow::Error;
use diwan::screen::{MainScreen, Modes};
use termwiz::terminal::{buffered::BufferedTerminal, UnixTerminal};
use core::task;
use std::os::unix::thread;
use std::sync::Mutex;
use std::sync::Arc;

use std::io;
use std::io::Write;
use std::thread::Thread;

struct MockTerminal();

impl MockTerminal {
    pub fn write_char(ch: char) -> Result<(), Error> {
        let mut stdout = io::stdout().lock();
        write!(&mut stdout, "{}", ch)?;

        Ok(())
    }
}


pub fn bootstrap_diwan() -> Result<(Arc<Mutex<BufferedTerminal<UnixTerminal>>>, Arc<Mutex<MainScreen>>), Error> {
    // init the a new buffered terminal
    let dnbuffer = MainScreen::new_buffered_term()?;
    // init a mutex string for our poet :)
    let typed_text = Arc::new(Mutex::new(String::new()));
    // in simplified lang: combine the initialized bufer and content String
    // and return a mutable buffer and main_screen for displaying everingthing
    let (mut buffer, main_screen) =
        MainScreen::new_with_widget(dnbuffer, Arc::clone(&typed_text))?;

    let my_buffer = Arc::new(Mutex::new(buffer));
    let my_main_screen = Arc::new(Mutex::new(main_screen));
    // set up the ui
    let shared_ui = Arc::new(Mutex::new(main_screen.setup_ui()));

    // clone the shared ui
    // enter the main loop

    let ui = shared_ui.clone();
    let handler = std::thread::spawn(move || {

        MainScreen::main_event_loop(&mut my_buffer.lock().unwrap(), &mut ui.lock().unwrap()).unwrap();
    });

    Ok((my_buffer, my_main_screen))
}


#[test]
fn go_right() {
    let (_buffer, widget) = bootstrap_diwan().unwrap();
    assert_eq!(widget.mode, Modes::Normal);
    let _ = MockTerminal::write_char('i');
    assert_eq!(widget.mode, Modes::Insert);
}
