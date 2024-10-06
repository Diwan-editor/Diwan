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
        const PADDING: usize = 2;
        const MIN_CONTENT_WIDTH: usize = 40;
        const STATUS_BAR_OFFSET: usize = 2;
        const GRUVBOX_SOFT_BACKGROUND: (u8, u8, u8) = (50, 48, 47);
        const WHITE: (u8, u8, u8) = (251, 241, 194);

        let (width, height) = args.surface.dimensions();
        let available_width = width.saturating_sub(2 * PADDING);

        let width_for_center = available_width.saturating_sub(MIN_CONTENT_WIDTH);

        let status_text = format!(
            "{:<20}{:^width$}{:>20}",
            self.status_mode.to_uppercase(),
            self.filename,
            format!("{}:{}", cursor_x + 1, cursor_y + 1),
            width = width_for_center
        );

        let status_bar_y = height.saturating_sub(STATUS_BAR_OFFSET);

        // Prepare all changes in a vector
        let mut changes = vec![
            Change::CursorPosition {
                x: Position::Absolute(0),
                y: Position::Absolute(status_bar_y),
            },
            Change::Attribute(AttributeChange::Foreground(
                ColorAttribute::TrueColorWithPaletteFallback(
                    (WHITE).into(),
                    AnsiColor::White.into(),
                ),
            )),
            Change::Attribute(AttributeChange::Background(
                ColorAttribute::TrueColorWithPaletteFallback(
                    (GRUVBOX_SOFT_BACKGROUND).into(),
                    AnsiColor::Maroon.into(),
                ),
            )),
            Change::Text(" ".repeat(PADDING)),
            Change::Text(status_text),
            Change::Text(" ".repeat(PADDING)),
        ];

        // Fill the rest of the line if necessary
        // if available_width > status_text.len() {
        //     let remaining_space = available_width.saturating_sub(status_text.len());
        //     changes.push(Change::Text(" ".repeat(remaining_space)));
        // }

        // Clear the line below the status bar
        changes.extend_from_slice(&[Change::CursorPosition {
            x: Position::Absolute(0),
            y: Position::Relative(1),
        }]);

        // Apply all changes at once
        args.surface.add_changes(changes);
    }
}
