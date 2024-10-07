use crate::screen::{Modes, StatusBar};

impl Default for StatusBar {
    fn default() -> Self {
        StatusBar {
            status_mode: Modes::Normal.to_string(),
            filename: "[SCRATCH]".to_string(), //NOTE: try to call struct field that controls over the file path
        }
    }
}
