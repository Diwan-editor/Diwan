use anyhow::{Error, Result};
use clap::{
    builder::{styling::AnsiColor, Styles},
    Parser,
};
use diwan::logs::{DiwanLevelLog, DiwanLogger};
use diwan::screen::DWidget;
use termwiz::widgets::WidgetId;
use diwan::screen::pubsub::*;
use std::{
    process::exit,
    sync::{Arc, Mutex},
};
use tokio::task;

/// diwan is a rust based text editor that is fast and secure.
#[derive(Parser, Debug)]
#[command(version = env!("CARGO_PKG_VERSION"), author = env!("CARGO_PKG_AUTHORS"), about = env!("CARGO_PKG_DESCRIPTION"),styles = handle_cli_help_color())]
struct DiwanArgs {
    /// load the user manual
    #[arg(short, long)]
    man: bool,

    /// test log
    #[arg(short, long)]
    log: bool,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let arg = DiwanArgs::parse();

    if arg.man {
        println!("Loading the manual");
    } else if arg.log {
        let diwan_logger = DiwanLogger::new(DiwanLevelLog::Debug)?;
        diwan_logger.setup_dn_logger()?;

        diwan_logger.write_to_dn_log(DiwanLevelLog::Critical, "Sorry daddy I made an error!");
        diwan_logger.write_to_dn_log(DiwanLevelLog::Debug, "Oh yeah debug me daddy!");
        diwan_logger.write_to_dn_log(DiwanLevelLog::Info, "Daddy inform me if you reached home!");
        diwan_logger.write_to_dn_log(
            DiwanLevelLog::Warn,
            "Daddy don't be bad boy! I will tell mommy",
        );
    } else {
        // init the a new buffered terminal
        let dnbuffer = new_buffered_term()?;
        let dnbuffer2 = new_buffered_term()?;
        let dnbuffer3 = new_buffered_term()?;
        // init a mutex string for our poet :)
        // potentially no need
        //let typed_text = Arc::new(Mutex::new(String::new()));
        // in simplified lang: combine the initialized bufer and content String
        // and return a mutable buffer and main_screen for displaying everingthing
        // let (mut buffer, main_screen) =
        //     MainScreen::new_with_widget(dnbuffer, Arc::clone(&typed_text))?;
        let (mut buffer, mut main_screen) =
            new_with_widget(dnbuffer)?;
        let (mut buffer2, mut widget2) =
            new_with_widget(dnbuffer2)?;
        let (_, mut widget3) =
            new_with_widget(dnbuffer3)?;
        // set up the ui
        let shared_ui = Arc::new(Mutex::new(setup_ui()));
        let main_screen_id = set_root(&mut shared_ui.lock().unwrap(), main_screen);
        //tempo </
        buffer2.resize(10, 10); //
        dbg!(buffer2.title());
        // let widget_id = widget2.widget_id.clone(); // we dont actually need this !
        let id_value_2 = shared_ui.lock().unwrap().add(None, widget2.clone());
        widget2.update_widget_id(id_value_2);
        let id_value_3 = shared_ui.lock().unwrap().add(None, widget3.clone());
        widget3.update_widget_id(id_value_3);
        // let ui_clone = shared_ui.clone();
        // let mut locked = ui_clone.lock().unwrap();
        // let _ = locked.render_to_screen(&mut buffer2);

         //shared_ui.lock().unwrap().set_focus(id_value); // this cause the tempo !
         //issue />


        // clone the shared ui
        let ui_clone = shared_ui.clone();
        let _ = task::spawn(async move {
            // lock the ui in order to be used for this green thread
            let mut ui = ui_clone.lock().unwrap();
            // enter the main loop
            main_event_loop(&mut buffer, &mut ui).unwrap();
            main_event_loop(&mut buffer2, &mut ui).unwrap();
        })
        .await;




        // let _ = task::spawn(async move {
        //     // lock the ui in order to be used for this green thread
        //     let mut ui = ui_clone.lock().unwrap();
        //     // enter the main loop
        //     DWidget::main_event_loop(&mut buffer2, &mut ui).unwrap();
        // }).await;
        // shared_ui.lock().unwrap().add_child(, widget2);
        println!("Shutting down Normally...");
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
