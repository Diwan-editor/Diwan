use diterm::Editor;
use std::env;
fn main() -> anyhow::Result<()> {
    // Get the command-line arguments
    let args: Vec<String> = env::args().collect();

    // Match on the command-line arguments to handle different cases
    match args.get(1).map(String::as_str) {
        Some("-h") | Some("--help") => {
            print_help();
            Ok(())
        }
        _ => {
            // init the editor with no argument
            Editor::new()?;
            Ok(())
        }
    }
}

fn print_help() {
    println!("Diwanwiz is a minimal rust text editor like vim");
    println!("Usage: diwan [OPTIONS] [FILE]");
    println!();
    println!("Arguments:");
    println!("  FILE             The file to open in the editor");
    println!();
    println!("Options:");
    println!("  -h, --help       Print this help message");
    println!();
}
