use std::sync::{Arc, Mutex};
use termwiz::caps::Capabilities;
use termwiz::input::*;
use termwiz::surface::{Change, Position};
use termwiz::terminal::UnixTerminal;
use termwiz::terminal::{buffered::BufferedTerminal, Terminal};
use termwiz::widgets::{Ui, WidgetEvent};
use termwiz::Error;

use super::keymap::Modes;
use super::{Keymap, SendableUi, StatusBar};

/// The `MainScreen` struct deals with rendering the main screen of the Diwan editor.
pub struct MainScreen {
    /// Shared text content
    pub text: Arc<Mutex<String>>,
    /// Modes (Normal, Insert, etc.)
    pub mode: Modes,
    /// X position of the cursor
    pub cursor_x: usize,
    /// Y position of the cursor
    pub cursor_y: usize,
    /// Status bar displaying mode, etc.
    pub status_bar: StatusBar,
    /// History
    pub yank: Vec<String>,
}

impl MainScreen {
    /// # new_buffered_term
    /// function that returns a `Result<BufferedTerminal<UnixTerminal>, Error>`
    /// it was needed to initalize the terminal buffer before feeding it to the widget
    pub fn new_buffered_term() -> Result<BufferedTerminal<UnixTerminal>, Error> {
        let caps = Capabilities::new_from_env()?;
        let term = UnixTerminal::new(caps)?;
        let buffer = BufferedTerminal::new(term)?;
        Ok(buffer)
    }
    /// Creates a new instance of the struct along with a terminal buffer and initializes a status bar.
    ///
    /// This function sets up the terminal in raw mode and enters an alternate screen mode.
    /// It also initializes the editor or widget state with the provided content and a status bar
    /// showing the current mode.
    ///
    /// # Arguments
    ///
    /// * `buffer` - A mutable `BufferedTerminal` instance that interacts with a Unix terminal.
    /// * `content` - A shared, thread-safe string wrapped in an `Arc<Mutex<String>>`, which represents the editable content.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing:
    /// - A tuple with the `BufferedTerminal` instance (set in raw mode and alternate screen) and
    ///   the newly created struct with initialized state and status bar, or
    /// - An `Error` if there is a failure in setting the terminal to raw mode or entering the alternate screen.
    ///
    /// # Errors
    ///
    /// This function can return an error if setting the terminal to raw mode or entering the alternate screen mode fails.
    /// # Example
    ///
    /// ```rust
    /// use std::sync::{Arc, Mutex};
    /// let buffer = MainScreen::new_buffered_term()?;
    /// let content = Arc::new(Mutex::new(String::new()));
    /// let (buffer, widget) = MainScreen::new_with_widget(buffer, content).unwrap();
    /// ```
    pub fn new_with_widget(
        mut buffer: BufferedTerminal<UnixTerminal>,
        content: Arc<Mutex<String>>,
    ) -> Result<(BufferedTerminal<UnixTerminal>, Self), Error> {
        buffer.terminal().set_raw_mode()?;
        buffer.terminal().enter_alternate_screen()?;
        let status_bar = StatusBar::default();
        Ok((
            buffer,
            Self {
                text: content,
                mode: Modes::Normal,
                cursor_x: 0,
                cursor_y: 0,
                yank: vec![],
                status_bar,
            },
        ))
    }
    /// Sets up the UI for the main screen by creating and configuring the root widget.
    ///
    /// This function initializes a new `Ui` instance, sets the current `MainScreen`
    /// as the root widget, and returns a `SendableUi` instance with the configured UI.
    ///
    /// # Returns
    /// - `SendableUi<'static>`: A `SendableUi` object that holds the UI with the
    ///   root set to the `MainScreen`.
    ///
    /// # Example
    /// ```
    /// let main_screen = MainScreen::new();
    /// let ui = main_screen.setup_ui();
    /// ```
    /// This will set the `MainScreen` as the root widget and prepare the UI for rendering
    pub fn setup_ui(self) -> SendableUi<'static> {
        let mut ui = Ui::new();
        ui.set_root(self);
        SendableUi::new(ui)
    }
    /// Main event loop for handling terminal input and refreshing the UI.
    ///
    /// # Arguments
    /// - `buffer`: The terminal buffer to read and write terminal events.
    /// - `ui`: The UI structure handling rendering.
    ///
    /// This function processes user input events like keystrokes and updates the UI accordingly.
    pub fn main_event_loop(
        buf: &mut BufferedTerminal<impl Terminal>,
        ui: &mut SendableUi,
    ) -> Result<(), Error> {
        loop {
            // Process any queued UI events (if present)
            ui.process_event_queue()?;

            // Render the updated UI to the screen
            if ui.render_to_screen(buf)? {
                buf.flush()?; // Ensure that the terminal is flushed after rendering
                continue;
            }

            // Flush the terminal buffer after every loop iteration to prevent delays
            buf.flush()?;

            // Handle user input (polling for key presses)
            match buf.terminal().poll_input(None) {
                Ok(Some(input)) => match input {
                    // Quit on Alt+Q
                    InputEvent::Key(KeyEvent {
                        key: KeyCode::Char('q'),
                        modifiers: Modifiers::ALT,
                    }) => {
                        Self::quit_application(buf); // to check what is this ?
                        break;
                    }
                    InputEvent::Resized { rows, cols } => {
                        buf.add_change(Change::ClearScreen(Default::default()));
                        buf.resize(cols, rows);
                    }
                    // Other inputs are queued as widget events
                    other_input => {
                        ui.queue_event(WidgetEvent::Input(other_input));
                    }
                },
                Ok(None) => {}
                Err(e) => {
                    println!("{:?}\r\n", e);
                    break;
                }
            }
        }
        Ok(())
    }
    /// func that exit the terminal gracefully
    fn quit_application(buffer: &mut BufferedTerminal<impl Terminal>) {
        if let Err(e) = Keymap::close_terminal(buffer) {
            eprintln!("Failed to close terminal gracefully: {}", e);
        }
    }
}
