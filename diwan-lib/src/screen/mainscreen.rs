use std::sync::{Arc, Mutex, MutexGuard};

use termwiz::caps::Capabilities;
use termwiz::input::*;
use termwiz::surface::{Change, Position};
use termwiz::terminal::UnixTerminal;
use termwiz::terminal::{buffered::BufferedTerminal, new_terminal, Terminal};
use termwiz::widgets::*;
use termwiz::Error;

use super::keymap::Modes;
use super::{Keymap, SendableUi, StatusBar};

/// The `MainScreen` is a struct that deals with rendering
/// the main screen of Diwan editor.
pub struct MainScreen {
    /// Holds the input text that we wish the widget to display
    pub text: Arc<Mutex<String>>,
    /// Modes to let the user know what mode he/she in
    pub mode: Modes,
    /// cursor position
    pub cursor_pos: usize,
    /// status bar that feed the user with necessary information (mode, filename, cursor xy, language, and gitflow)
    pub status_bar: StatusBar,
}

/// Creates and returns a `MainScreen` object using a buffered terminal.
/// Initializes the terminal and sets up the UI components for interaction.
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
        let status_bar = StatusBar::new();
        Ok((
            buffer,
            Self {
                text: content,
                mode: Modes::Normal,
                cursor_pos: 0,
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
    /// This will set the `MainScreen` as the root widget and prepare the UI for rendering.
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
            ui.process_event_queue()?;

            // After updating and processing all of the widgets, compose them
            // and render them to the screen.
            if ui.render_to_screen(buf)? {
                // We have more events to process immediately; don't block waiting
                // for input below, but jump to the top of the loop to re-run the
                // updates.
                continue;
            }
            // Compute an optimized delta to apply to the terminal and display it
            buf.flush()?;

            // Wait for user input
            match buf.terminal().poll_input(None) {
                Ok(Some(InputEvent::Resized { rows, cols })) => {
                    // FIXME: this is working around a bug where we don't realize
                    // that we should redraw everything on resize in BufferedTerminal.
                    buf.add_change(Change::ClearScreen(Default::default()));
                    buf.resize(cols, rows);
                }
                Ok(Some(input)) => match input {
                    InputEvent::Key(KeyEvent {
                        key: KeyCode::Char('q'),
                        modifiers: Modifiers::ALT,
                    }) => {
                        // Quit the app when escape is pressed
                        Keymap::close_terminal(buf).unwrap();
                        break;
                    }
                    input => {
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

unsafe impl Send for MainScreen {}
unsafe impl Sync for MainScreen {}
