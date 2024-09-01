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
    let mut typed_text2 = String::new();
    let main_screen = MainScreen::new_with_widget(&mut typed_text)?;
    let ui = main_screen.setup_ui();
    let mut arc_ui = Arc::new(Mutex::new(&ui));

    tokio::spawn(move {
        let ui = *arc_ui.lock().unwrap();
        main_screen.main_event_loop(ui)?;
    });

    let main_screen2 = MainScreen::new_with_widget(&mut typed_text2)?;

    tokio::spawn(move {
        let ui = *arc_ui.lock().unwrap();
        let widgetId = ui.add(Some(main_screen2));
    });

    // TODO:
    // 1. Create a new thread that holds WidgetId;
    // 2. Ui must be in a separate thread ;
    // 3. Read a bit about Discovering threads , u may want to read about Observable pattern !

    println!("widgetId: {}", widgetId);
    dbg!(&ui);
    println!("The text you entered: {}", typed_text);

    Ok(())
}
