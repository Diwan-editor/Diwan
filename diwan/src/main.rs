mod commands;
mod editor;
mod utils;

use editor::Editor;
use utils::FileManager;
use log::{info, error};
use env_logger;
use tokio;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let mut editor = Editor::new();
    let file_manager = FileManager::new();

    // Load file asynchronously
    match file_manager.load_file("lh7abs.txt").await {
        Ok(contents) => {
            // info!("File loaded successfully");
            editor.load_contents(contents);
        }
        Err(e) => {
            error!("if not working : {}", e);
            return Err(e);
        }
    }

    // Enter raw mode and handle input
    editor.enter_raw_mode();
    editor.handle_input();

    // Save file asynchronously
    match file_manager.save_file("example.txt", &editor.get_contents()).await {
        Ok(_) => {
            info!("File saved successfully");
        }
        Err(e) => {
            error!("Failed to save file: {}", e);
            return Err(e);
        }
    }

    Ok(())
}
