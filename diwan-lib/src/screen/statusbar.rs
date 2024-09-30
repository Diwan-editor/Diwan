use termwiz::cell::AttributeChange;
use termwiz::color::{AnsiColor, ColorAttribute};
use termwiz::input::*;
use termwiz::surface::{Change, Position, Surface};
use termwiz::widgets::*;

use super::Modes;

/// `StatusBar` is a widget used to display information
/// about the current state of the editor. This includes:
/// <ul>
///     <li><code style="color:yellow">Current Mode</code>: e.g., Insert or Normal</li>
///     <li><code style="color:yellow">Filename</code>: The name of the file currently being edited</li>
///     <li><code style="color:yellow">Language</code>: The programming language of the file</li>
///     <li><code style="color:yellow">Git Status</code>: The current Git status of the file</li>
/// </ul>
pub struct StatusBar {
    /// <div>
    /// <code>Current Mode:</code> Stores the current mode (e.g.,
    /// <code>INSERT</code>, <code>NORMAL</code>) which will be displayed in
    /// the status bar.
    /// </div>
    pub status_mode: String,
}

impl StatusBar {
    /// Creates a new `StatusBar` instance with the default mode.
    ///
    /// # Returns
    /// A new `StatusBar` initialized with "NORMAL" as the mode.
    ///
    /// # Example
    /// ```
    /// let status_bar = StatusBar::new();
    /// ```
    pub fn new() -> Self {
        Self {
            status_mode: "NORMAL".to_string(),
        }
    }
    /// Updates the `status_mode` of the `StatusBar` based on the current mode.
    ///
    /// # Arguments
    /// * `mode` - The new mode to display, which is of type `Modes`.
    ///
    /// <div class="warning">
    /// The mode enum implements the <code style="color:yellow">fmt::Display</code> trait in order to make the
    /// Normal and Insert mode in uppercase.
    /// </div>
    ///
    /// This function is called whenever the editor mode changes (e.g., switching
    /// between normal and insert mode).
    ///
    /// # Example
    /// ```
    /// status_bar.update(&Modes::Insert);
    /// ```
    pub fn update(&mut self, mode: &Modes) {
        // Update the status_mode field with the current mode.
        self.status_mode = format!("{}", mode);
    }
}

/// Implementation of the `Widget` trait for `StatusBar`, which allows it to be
/// used as part of the UI. This trait defines how the widget handles events
/// and how it should be rendered on the screen.
impl Widget for StatusBar {
    /// Processes any events passed to the `StatusBar`.
    ///
    /// Since the `StatusBar` is static (i.e., it doesn't respond to events
    /// directly), this function always returns `false`.
    ///
    /// # Arguments
    /// * `_event` - The event to process.
    /// * `_args` - Additional arguments related to the event.
    ///
    /// # Returns
    /// Always returns `false` because the `StatusBar` doesn't respond to events.
    fn process_event(&mut self, _event: &WidgetEvent, _args: &mut UpdateArgs) -> bool {
        false
    }

    /// Renders the `StatusBar` on the terminal surface.
    ///
    /// This function positions the status bar at the bottom row of the terminal,
    /// with white text on a purple background. It fills the entire row with the
    /// current mode (e.g., "NORMAL") and pads the remaining space with spaces.
    ///
    /// # Arguments
    /// * `args` - The rendering arguments, including the surface to render on.
    fn render(&mut self, args: &mut RenderArgs) {
        // Get the terminal dimensions (width and height).
        let dims = args.surface.dimensions();

        // Define padding values
        let left_padding = 4;
        let vertical_offset = 1;
        // Create a padded status text to fill the entire width of the terminal.
        let status_text_padded = format!("{:<width$}", self.status_mode, width = dims.0);

        // Set the cursor position to the start of the bottom row.
        args.surface.add_change(Change::CursorPosition {
            x: Position::Relative(0), // Start of the line
            y: Position::Relative(((dims.0 - 1) as u16).try_into().unwrap()), // Bottom row
        });

        // Set the foreground (text) color to white.
        args.surface
            .add_change(Change::Attribute(AttributeChange::Foreground(
                ColorAttribute::TrueColorWithPaletteFallback(
                    (0xFF, 0xFF, 0xFF).into(), // RGB for white
                    AnsiColor::White.into(),
                ),
            )));

        // Set the background color to purple.
        args.surface
            .add_change(Change::Attribute(AttributeChange::Background(
                ColorAttribute::TrueColorWithPaletteFallback(
                    (0x28, 0x28, 0x28).into(), // RGB for purple
                    AnsiColor::Maroon.into(),
                ),
            )));

        // Render the padded status text.
        args.surface.add_change(Change::Text(status_text_padded));
    }
}
