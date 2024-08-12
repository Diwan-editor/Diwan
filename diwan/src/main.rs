mod commands;
mod editor;
mod utils;
mod screen;
use tokio;

use termwiz::Error;
use screen::MainScreen;

#[tokio::main]
async fn main() -> Result<(), Error> {

    let mut typed_text = String::new();
    let main_screen = MainScreen::new_with_widget(&mut typed_text)?;
    let ui = main_screen.setup_ui();
    main_screen.main_event_loop(ui)?;

    println!("The text you entered: {}", typed_text);
    Ok(())
}
