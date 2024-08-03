pub mod text_buffer;
pub mod renderer;

use text_buffer::TextBuffer;
use renderer::Renderer;

pub struct Editor {
    text_buffer: TextBuffer,
    renderer: Renderer,
}

impl Editor {
    pub fn new() -> Self {
        Self {
            text_buffer: TextBuffer::new(),
            renderer: Renderer::new(),
        }
    }

    pub fn load_contents(&mut self, contents: String) {
        self.text_buffer.load_contents(contents);
    }

    pub fn get_contents(&self) -> String {
        self.text_buffer.get_contents()
    }

    pub fn enter_raw_mode(&self) {
        self.renderer.enter_raw_mode();
    }

    pub fn handle_input(&self) {
        self.renderer.handle_input();
    }
}
