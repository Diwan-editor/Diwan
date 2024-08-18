pub struct TextBuffer {
    contents: String,
}

impl TextBuffer {
    pub fn new() -> Self {
        Self {
            contents: String::new(),
        }
    }

    pub fn load_contents(&mut self, contents: String) {
        self.contents = contents;
    }

    pub fn get_contents(&self) -> String {
        self.contents.clone()
    }
}
