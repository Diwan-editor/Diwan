use super::Modes;

pub struct StatusBar {
    pub status_text: String, // Make it a String so we can modify it
}

impl StatusBar {
    pub fn new(status_text: &str) -> Self {
        Self {
            status_text: status_text.to_string(),
        }
    }

    pub fn update(&mut self, mode: &Modes) {
        self.status_text = format!("Mode: {:?}", mode);
    }
}
