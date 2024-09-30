use core::fmt;
use std::fmt::write;

use crate::screen::Modes;

impl fmt::Display for Modes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Modes::Normal => write!(f, "NORMAL"),
            Modes::Insert => write!(f, "INSERT"),
        }
    }
}
