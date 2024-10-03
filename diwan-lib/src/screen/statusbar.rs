use super::{MainScreen, Modes};
use termwiz::cell::AttributeChange;
use termwiz::color::{AnsiColor, ColorAttribute};
use termwiz::surface::{Change, Position};
use termwiz::widgets::*;

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
    /// <div>
    /// <code>Filename:</code> Stores the filename (e.g. <code>dummy.rs</code>) which will be displayed in
    /// the status bar.
    /// </div>
    pub filename: String,
}

impl StatusBar {
    /// Creates a new `StatusBar` instance with the specified filename and mode.
    ///
    /// # Arguments
    /// * `filename` - The name of the file to be displayed in the status bar.
    /// * `status_mode` - The current mode (e.g., "NORMAL" or "INSERT").
    ///
    /// # Returns
    /// A new `StatusBar` initialized with the provided filename and mode.
    ///
    /// # Example
    /// ```
    /// let status_bar = StatusBar::new("dummy.rs", "NORMAL");
    /// ```
    pub fn new(filename: &str, status_mode: &str) -> Self {
        Self {
            status_mode: status_mode.to_string(),
            filename: filename.to_string(),
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
        self.status_mode = format!("{:?}", mode);
    }
    /// Renders the status bar at the bottom of the screen.
    ///
    /// # Arguments
    /// * `args` - A mutable reference to `RenderArgs`, which provides the rendering context.
    /// * `cursor_pos` - A tuple representing the cursor position (line, column) in the editor.
    ///
    /// This function adds changes to the surface to display the status bar with the current mode,
    /// filename, and cursor position.
    pub fn render(&mut self, args: &mut RenderArgs, cursor_x: usize, cursor_y: usize) {
        let dims = args.surface.dimensions();

        // Ensure that we don't subtract below 0 (which would cause overflow)
        let min_width = 40; // Minimum width for the status text layout
        let width_for_center = if dims.0 > min_width {
            dims.0 - min_width
        } else {
            // If terminal width is too small, set to 0 to prevent negative width
            0
        };

        // Create the string that shows the cursor position (e.g., "Ln 3, Col 5")
        let cursor_pos_text = format!("{}:{}", cursor_x + 1, cursor_y + 1); // Convert to 1-based indexing
        let status_text = format!(
            "{:<20}{:^width$}{:>20}",
            self.status_mode.to_uppercase(), // FIXME(In future): the uppercase is not applicable regardles the impl of fmt::Display of Modes enum
            self.filename,
            cursor_pos_text,
            width = width_for_center
        );

        args.surface.add_change(Change::CursorPosition {
            x: Position::Relative(0),
            y: Position::Relative((dims.1 - 1) as isize), // Position at the last row
        });

        // White text on a dark background for the status bar
        args.surface
            .add_change(Change::Attribute(AttributeChange::Foreground(
                ColorAttribute::TrueColorWithPaletteFallback(
                    (0xFF, 0xFF, 0xFF).into(),
                    AnsiColor::White.into(),
                ),
            )));
        args.surface
            .add_change(Change::Attribute(AttributeChange::Background(
                ColorAttribute::TrueColorWithPaletteFallback(
                    (0x28, 0x28, 0x28).into(),
                    AnsiColor::Maroon.into(),
                ),
            )));

        // Render the status text with mode, filename, and cursor position
        args.surface.add_change(Change::Text(status_text));
    }
}
