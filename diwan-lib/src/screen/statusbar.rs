/// struct for status bar
pub struct StatusBar<'a> {
    pub status_text: &'a str,
}

impl<'a> StatusBar<'a> {
    pub fn new(status_text: &'a str) -> Self {
        Self { status_text }
    }
}
