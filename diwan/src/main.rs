use anyhow::{Error, Result};
use clap::{
    builder::{styling::AnsiColor, Styles},
    Parser,
};
use diwan::{
    core::{
        broker::Broker,
        sync::command::{BrokerCommand, CommandField},
        utils::*,
    },
    logs::{DiwanLevelLog, DiwanLogger},
};

use std::{
    process::exit,
    sync::{Arc, Mutex},
    thread::sleep,
    time::Duration,
};
use termwiz::surface::Change;
use tokio::task;
use tokio::{spawn, sync::mpsc};

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
    } else {
        // init the a new buffered terminal
        let dnbuffer = new_buffered_term()?;
        let dnbuffer2 = new_buffered_term()?;
        let dnbuffer3 = new_buffered_term()?;
        // works !
        // if i didn't specify the await here , how / when it's awaited ?
        let (tx, rx) = mpsc::channel::<BrokerCommand>(32);
        tokio::spawn(Broker::new(rx));

        // init a mutex string for our poet :)
        // potentially no need
        //let typed_text = Arc::new(Mutex::new(String::new()));
        // in simplified lang: combine the initialized bufer and content String
        // and return a mutable buffer and main_screen for displaying everingthing
        // let (mut buffer, main_screen) =
        //     MainScreen::new_with_widget(dnbuffer, Arc::clone(&typed_text))?;
        let (mut buffer, main_screen) = new_with_widget(dnbuffer)?;
        let (mut buffer2, mut widget2) = new_with_widget(dnbuffer2)?;
        let (_, mut widget3) = new_with_widget(dnbuffer3)?;

        // set up the ui
        let shared_ui = Arc::new(Mutex::new(setup_ui()));
        let main_screen_id = set_root(&mut shared_ui.lock().unwrap(), main_screen)?;
        set_focus(&mut shared_ui.lock().unwrap(), &main_screen_id);

        //tempo </
        // buffer2.resize(10, 100); //
        let id_value_2 = shared_ui.lock().unwrap().add(None, widget2.clone());
        widget2.update_widget_id(id_value_2);
        let id_value_3 = shared_ui.lock().unwrap().add(None, widget3.clone());
        widget3.update_widget_id(id_value_3);

        // clone the shared ui
        let ui_clone = shared_ui.clone();
        let _ = task::spawn(async move {
            // lock the ui in order to be used for this green thread
            let mut ui = ui_clone.lock().unwrap();
            buffer.add_change(Change::Title("this is a dumb title".to_owned()));
            buffer2.add_change(Change::Title("this is another dumb title".to_owned()));
            // enter the main loop
            main_event_loop(&mut buffer, &mut ui).unwrap();
            // main_event_loop(&mut buffer2, &mut ui).unwrap();
        })
        .await;

        tx.send(BrokerCommand::GetBuffers).await.unwrap();
        tx.send(BrokerCommand::GetBuffers).await.unwrap();
        tx.clone()
            .send(BrokerCommand::Mes(
                12,
                CommandField::Regular,
                "test_comm".to_owned(),
            ))
            .await
            .unwrap();
        exit(0);
    }
    println!("Shutting down Normally...");
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
