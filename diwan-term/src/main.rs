use anyhow::{Error, Result};
use clap::{
    builder::{styling::AnsiColor, Styles},
    Parser,
};
use diwan::{broker::Broker, screen::MainScreen};
use std::{
    process::exit,
    sync::{Arc, Mutex},
};
use tokio::task;
use diwan::loging::loging::Logger;

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
        // init the a new buffered terminal
        let dnbuffer = MainScreen::new_buffered_term()?;
        // init a mutex string for our poet :)
        let typed_text = Arc::new(Mutex::new(String::new()));
        // in simplified lang: combine the initialized bufer and content String
        // and returna mutable buffer and main_screen for displaying everingthing
        let (mut buffer, main_screen) =
            MainScreen::new_with_widget(dnbuffer, Arc::clone(&typed_text))?;

        // set up the ui
        let shared_ui = Arc::new(Mutex::new(main_screen.setup_ui()));

        // clone the shared ui
        let ui_clone = shared_ui.clone();
        let _ = task::spawn(async move {
            // lock the ui in order to be used for this green thread
            let mut ui = ui_clone.lock().unwrap();
            // enter the main loop
            MainScreen::main_event_loop(&mut buffer, &mut ui).unwrap();
        })
        .await;

        let logger_handler = task::spawn(async move {
            let mut logger = Logger::setup_login().unwrap();

            logger.write_logs("example log", diwan::loging::loging::Criticality::Normal)
        }).await;
        // // Second UI
        // let dnbuffer2 = MainScreen::new_buffered_term()?;
        // let typed_text2 = Arc::new(Mutex::new(String::new()));
        // let (mut buffer2, main_screen2) =
        //     MainScreen::new_with_widget(dnbuffer2, Arc::clone(&typed_text2))?;
        // let ui_clone2 = shared_ui.clone();

        // task::spawn(async move {
        //     let mut ui = ui_clone2.lock().unwrap();
        //     let _widget_id = ui.add(None, main_screen2);
        //     MainScreen::main_event_loop(&mut buffer2, &mut ui).unwrap();
        // });
        // tokio::signal::ctrl_c().await?;
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
