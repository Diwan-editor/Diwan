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
        let mut typed_text = String::new();
        let buffer = MainScreen::new_buffered_term()?;
        let (mut buffer, main_screen) = MainScreen::new_with_widget(buffer, &mut typed_text)?;
        let ui = main_screen.setup_ui();
        // create a mutable var that holds typed_text
        MainScreen::main_event_loop(&mut buffer, ui)?;

        let (w, h) = buffer.dimensions();
        println!("The text you entered: {}", typed_text);
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
