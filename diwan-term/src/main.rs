use anyhow::{Error, Result};
use clap::{
    builder::{styling::AnsiColor, Styles},
    Parser,
};
use diwan::screen::{MainScreen, SendableUi};
use std::{
    process::exit,
    sync::{Arc, Mutex},
};
use tokio::task;

/// diwan is a rust based text editor that is fast and secure.
#[derive(Parser, Debug)]
#[command(version = "1.0.0", about, long_about, styles = handle_cli_help_color())]
struct DiwanArgs {
    /// load the user manual
    #[arg(short, long)]
    man: bool,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let arg = DiwanArgs::parse();

    if arg.man {
        println!("Loading the manual");
    } else {
        let shared_ui = Arc::new(Mutex::new(SendableUi::new(termwiz::widgets::Ui::new())));

        // First UI
        let dnbuffer = MainScreen::new_buffered_term()?;
        let typed_text = Arc::new(Mutex::new(String::new()));
        let (mut buffer, main_screen) =
            MainScreen::new_with_widget(dnbuffer, Arc::clone(&typed_text))?;
        let ui_clone = shared_ui.clone();
        task::spawn(async move {
            let mut ui = ui_clone.lock().unwrap();
            ui.set_root(main_screen);
            MainScreen::main_event_loop(&mut buffer, &mut ui).unwrap();
        });

        // Second UI
        let dnbuffer2 = MainScreen::new_buffered_term()?;
        let typed_text2 = Arc::new(Mutex::new(String::new()));
        let (mut buffer2, main_screen2) =
            MainScreen::new_with_widget(dnbuffer2, Arc::clone(&typed_text2))?;
        let ui_clone2 = shared_ui.clone();

        task::spawn(async move {
            let mut ui = ui_clone2.lock().unwrap();
            let _widget_id = ui.add(None, main_screen2);
            MainScreen::main_event_loop(&mut buffer2, &mut ui).unwrap();
        });
        //FIXME: Wait for Ctrl+C signal
        tokio::signal::ctrl_c().await?;
        println!("Shutting down...");
        exit(0);
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
