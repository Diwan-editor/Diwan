use std::fs::File;
use std::io::{self, Read, Write};
use tokio::fs;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub struct FileManager {
    // File manager-related fields
}

impl FileManager {
    pub fn new() -> Self {
        Self {
            // Initialize fields
        }
    }

    pub async fn load_file(&self, file_name: &str) -> io::Result<String> {
        let mut file = fs::File::open(file_name).await?;
        let mut contents = String::new();
        file.read_to_string (&mut contents).await?;
        Ok(contents)
    }

    pub async fn save_file(&self, file_name: &str, contents: &str) -> io::Result<()> {
        let mut file = fs::File::create(file_name).await?;
        file.write_all(contents.as_bytes()).await?;
        Ok(())
    }
}
