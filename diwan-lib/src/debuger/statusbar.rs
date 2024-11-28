use crate::screen::{Modes, StatusBar};
use core::fmt;
use std::fmt::write;

impl fmt::Display for Modes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Modes::Normal => write!(f, "NORMAL"),
            Modes::Insert => write!(f, "INSERT"),
        }
    }
}

impl Default for StatusBar {
    fn default() -> Self {
        StatusBar {
            status_mode: Modes::Normal.to_string(),
            filename: "[SCRATCH]".to_string(), //NOTE: try to call struct field that controls over the file path
        }
    }
}
