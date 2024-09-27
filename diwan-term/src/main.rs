use std::sync::{Arc, Mutex};

use anyhow::{Error, Result};
use clap::{
    builder::{styling::AnsiColor, Styles},
    Parser,
};
use diwan::screen::MainScreen;

/// diwan is a rust based text editor that is fast and secure.
#[derive(Parser, Debug)]
#[command(version = "1.0.0", about, long_about, styles=handle_cli_help_color())]
struct DiwanArgs {
    /// load the user manual
    #[arg(short, long)]
    man: bool,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // parse the args for DiwanArgs structs
    let arg = DiwanArgs::parse();

    // match the args

    if arg.man {
        println!("Loading the manual")
    } else {
        // TODO:
        // 1. Create a new thread that holds WidgetId;
        // 2. Ui must be in a separate thread ;
        // 3. Read a bit about Discovering threads , u may want to read about Observable pattern !
        let mut dnbuffer = MainScreen::new_buffered_term()?;
        let mut dnbuffer2 = MainScreen::new_buffered_term()?;
        let mut typed_text = String::new();
        let mut typed_text2 = String::new();

        let (buffer, main_screen) = MainScreen::new_with_widget(dnbuffer, &mut typed_text)?;
        let ui = main_screen.setup_ui();
        let arc_ui = Arc::new(Mutex::new(ui));

        tokio::spawn(async move {
            let ui = arc_ui.lock().unwrap();
            MainScreen::main_event_loop(&mut buffer, ui).unwrap(); // TODO: MutexGuard needs to be handled
        });

        // another ui
        let (buffer2, main_screen2) = MainScreen::new_with_widget(dnbuffer2, &mut typed_text2)?;
        tokio::spawn(async move {
            let ui = arc_ui.lock().unwrap();
            let widgetId = ui.add(None, main_screen2);
        });

        tokio::signal::ctrl_c().await?;
        println!("Shut Down...")
    }

    Ok(())
}

// style help of diwan cli
fn handle_cli_help_color() -> Styles {
    Styles::styled()
        .usage(AnsiColor::BrightBlue.on_default())
        .header(AnsiColor::BrightYellow.on_default())
        .literal(AnsiColor::BrightMagenta.on_default())
        .invalid(AnsiColor::BrightRed.on_default())
        .error(AnsiColor::BrightRed.on_default())
        .valid(AnsiColor::BrightWhite.on_default())
        .placeholder(AnsiColor::BrightBlue.on_default())
}
// let mut typed_text = String::new();
// let buffer = MainScreen::new_buffered_term()?;
// let (mut buffer, main_screen) = MainScreen::new_with_widget(buffer, &mut typed_text)?;
// let ui = main_screen.setup_ui();
// // create a mutable var that holds typed_text
// MainScreen::main_event_loop(&mut buffer, ui)?;

// println!("The text you entered: {}", typed_text);
