pub mod widget;

use termwiz::widgets::*;
use termwiz::input::*;
use termwiz::terminal::{
    buffered::BufferedTerminal,
    new_terminal,
    Terminal
};
use termwiz::caps::Capabilities;
use termwiz::Error;
use termwiz::surface::Change;

/// This is a widget for our application
pub struct MainScreen<'a, T> where T: Terminal {
    /// Holds the input text that we wish the widget to display
    pub text: &'a mut String,
    pub buf: BufferedTerminal<T>,
}

impl<'a, T> MainScreen<'a, T> where T: Terminal + 'a  {

    pub fn new_with_widget(content: &mut String) -> Result<Self, Error> {
        let caps = Capabilities::new_from_env()?;
        let mut buf = BufferedTerminal::new(new_terminal(caps)?)?;
        buf.terminal().set_raw_mode()?;
        buf.terminal().enter_alternate_screen()?;

        Ok(
            Self {
                text: content,
                buf: buf
            }
        )
    }

    pub fn setup_ui(self) -> Ui<'a> {
        let mut ui = Ui::new();
        ui.set_root(self);
        ui
    }

    pub fn main_event_loop(&mut self, mut ui:Ui) -> Result<(), Error>{
        // let mut buf = self.buf;
        loop {
            ui.process_event_queue()?;

            // After updating and processing all of the widgets, compose them
            // and render them to the screen.
            if ui.render_to_screen(&mut self.buf)? {
                // We have more events to process immediately; don't block waiting
                // for input below, but jump to the top of the loop to re-run the
                // updates.
                continue;
            }
            // Compute an optimized delta to apply to the terminal and display it
            self.buf.flush()?;

            // Wait for user input
            match self.buf.terminal().poll_input(None) {
                Ok(Some(InputEvent::Resized { rows, cols })) => {
                    // FIXME: this is working around a bug where we don't realize
                    // that we should redraw everything on resize in BufferedTerminal.
                    self.buf.add_change(Change::ClearScreen(Default::default()));
                    self.buf.resize(cols, rows);
                }
                Ok(Some(input)) => match input {
                    InputEvent::Key(KeyEvent {
                        key: KeyCode::Escape,
                        ..
                    }) => {
                        // Quit the app when escape is pressed
                        break;
                    }
                    input @ _ => {
                        // Feed input into the Ui
                        ui.queue_event(WidgetEvent::Input(input));
                    }
                },
                Ok(None) => {}
                Err(e) => {
                    print!("{:?}\r\n", e);
                    break;
                }

            }
        }
        Ok(())
    }
}
