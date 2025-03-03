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
    pub status_mode: Modes,
    /// <div>
    /// <code>Current Branch:</code> Stores the current Branch (e.g.,
    /// <code>Main</code>) which will be displayed in
    /// the status bar.
    /// </div>
    pub branch: String,
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
    pub fn new() -> Self {
        Self {
            status_mode: Default::default(),
            filename: Default::default(),
            branch: Default::default(),
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
        self.status_mode = mode.clone();
    }

    // code must be refactored so it can be generalized !
    /// Renders the status bar at the bottom of the screen.
    ///
    /// # Arguments
    /// * `args` - A mutable reference to `RenderArgs`, which provides the rendering context.
    /// * `cursor_pos` - A tuple representing the cursor position (line, column) in the editor.
    ///
    /// This function adds changes to the surface to display the status bar with the current mode,
    /// filename, and cursor position.
    pub fn render(&mut self, args: &mut RenderArgs, cursor_x: usize, cursor_y: usize) {
        const PADDING: usize = 2; // Padding around the mode text
        const STATUS_BAR_OFFSET: usize = 2; // Distance from the bottom of the screen
        const GRUVBOX_SOFT_BACKGROUND: (u8, u8, u8) = (50, 48, 47); // Default background color
        const WHITE: (u8, u8, u8) = (251, 241, 194); // Default text color
        const D_GRAY: (u8, u8, u8) = (29, 32, 33); // Default text color (new)
        const MODE_BK: (u8, u8, u8) = (168, 153, 132); // Background color for the mode text

        let (width, height) = args.surface.dimensions();
        let status_bar_y = height.saturating_sub(STATUS_BAR_OFFSET);

        // Prepare the mode text with padding
        let mode_text = format!(" {} ", self.status_mode.to_string().to_uppercase());
        let mode_text_width = mode_text.len();

        // Prepare other components
        let branch_info = self.branch.clone(); // Example branch name
        let file_info = self.filename.clone(); // Example file path
        let cursor_pos = format!("{}:{}", cursor_y + 1, cursor_x + 1); // Cursor position

        // Calculate positions
        let left_x = 0; // Start of the mode text
        let middle_x = left_x + mode_text_width + 1; // Start of the middle section
        let right_x = width.saturating_sub(PADDING + cursor_pos.len()); // Start of the right section

        // Build the status bar
        let mut changes = vec![
            // Set background for the entire line
            Change::CursorPosition {
                x: Position::Absolute(0),
                y: Position::Absolute(status_bar_y),
            },
            Change::Attribute(AttributeChange::Background(
                ColorAttribute::TrueColorWithPaletteFallback(
                    GRUVBOX_SOFT_BACKGROUND.into(),
                    AnsiColor::Maroon.into(),
                ),
            )),
            Change::Text(" ".repeat(width)), // Fill entire line with background
        ];

        // Render mode text with custom background
        changes.extend([
            Change::CursorPosition {
                x: Position::Absolute(left_x),
                y: Position::Absolute(status_bar_y),
            },
            Change::Attribute(AttributeChange::Background(
                ColorAttribute::TrueColorWithPaletteFallback(
                    MODE_BK.into(),
                    AnsiColor::Maroon.into(),
                ),
            )),
            Change::Attribute(AttributeChange::Foreground(
                ColorAttribute::TrueColorWithPaletteFallback(
                    D_GRAY.into(),
                    AnsiColor::Black.into(),
                ),
            )),
            Change::Attribute(AttributeChange::Intensity(termwiz::cell::Intensity::Bold)),
            Change::Attribute(AttributeChange::Italic(true)),
            Change::Text(mode_text.clone()), // Apply background to the text area only
        ]);

        // Reset background for the rest of the line
        changes.extend([
            Change::Attribute(AttributeChange::Italic(false)),
            Change::Attribute(AttributeChange::Intensity(termwiz::cell::Intensity::Normal)),
            // rest the text color to white
            Change::Attribute(AttributeChange::Foreground(
                ColorAttribute::TrueColorWithPaletteFallback(WHITE.into(), AnsiColor::Black.into()),
            )),
            Change::CursorPosition {
                x: Position::Absolute(left_x + mode_text_width),
                y: Position::Absolute(status_bar_y),
            },
            Change::Attribute(AttributeChange::Background(
                ColorAttribute::TrueColorWithPaletteFallback(
                    GRUVBOX_SOFT_BACKGROUND.into(),
                    AnsiColor::Maroon.into(),
                ),
            )),
        ]);

        // Render middle section (branch and file info)
        changes.extend([
            Change::CursorPosition {
                x: Position::Absolute(middle_x),
                y: Position::Absolute(status_bar_y),
            },
            Change::Text(format!("{} | {}", branch_info, file_info)),
        ]);

        // Render right section (cursor position)
        changes.extend([
            Change::CursorPosition {
                x: Position::Absolute(right_x),
                y: Position::Absolute(status_bar_y),
            },
            Change::Text(cursor_pos),
        ]);

        // Clear line below
        changes.push(Change::CursorPosition {
            x: Position::Absolute(0),
            y: Position::Relative(1),
        });

        args.surface.add_changes(changes);
    }
}
// // Prepare all changes in a vector
// let mut changes = vec![
//     Change::CursorPosition {
//         x: Position::Absolute(0),
//         y: Position::Absolute(status_bar_y),
//     },
//     Change::Attribute(AttributeChange::Foreground(
//         ColorAttribute::TrueColorWithPaletteFallback(
//             (WHITE).into(),
//             AnsiColor::White.into(),
//         ),
//     )),
//     Change::Attribute(AttributeChange::Background(
//         ColorAttribute::TrueColorWithPaletteFallback(
//             (GRUVBOX_SOFT_BACKGROUND).into(),
//             AnsiColor::Maroon.into(),
//         ),
//     )),
//     Change::Text(" ".repeat(PADDING)),
//     Change::Text(status_text),
//     Change::Text(" ".repeat(PADDING)),
// ];

// // Clear the line below the status bar
// // why ?
// changes.extend_from_slice(&[Change::CursorPosition {
//     x: Position::Absolute(0),
//     y: Position::Relative(1),
// }]);

// // why not just this
// // changes.append(Change::CursorPosition {
// //     x: Position::Absolute(0),
// //     y: Position::Relative(1),
// // });

// // Why not just declare everything on top ?

// // Apply all changes at once
// args.surface.add_changes(changes);
