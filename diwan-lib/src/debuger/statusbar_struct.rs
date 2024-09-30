use crate::screen::StatusBar;

impl Default for StatusBar {
    fn default() -> Self {
        StatusBar {
            status_mode: "NORMAL".to_string(),
        }
    }
}
