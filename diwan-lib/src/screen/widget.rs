use crate::screen::MainScreen;
use termwiz::cell::AttributeChange;
use termwiz::color::{AnsiColor, ColorAttribute};
use termwiz::surface::{Change, Position, Surface};
use termwiz::widgets::*;

use super::{Keymap, Modes, StatusBar};

impl Widget for MainScreen {
    /// Process input events and update the screen
    fn process_event(&mut self, event: &WidgetEvent, _args: &mut UpdateArgs) -> bool {
        if let Some(action) = Keymap::map_key_to_action(event, &self.mode) {
            Keymap::handle_action(
                action,
                self.text.clone(),
                &mut self.cursor_x,
                &mut self.cursor_y,
                &mut self.mode,
            );
            self.status_bar.update(&self.mode);
        }
        true // Always return true to indicate the UI should re-render
    }

    /// Render the screen content, including text and the status bar
    fn render(&mut self, args: &mut RenderArgs) {
        let text_guarded = self.text.lock().unwrap(); // Lock the content only briefly to access it
        let (width, height) = args.surface.dimensions();

        // Clear the screen with Gruvbox dark background color
        args.surface.add_change(Change::ClearScreen(
            ColorAttribute::TrueColorWithPaletteFallback(
                (0x1d, 0x20, 0x21).into(),
                AnsiColor::Black.into(),
            ),
        ));

        // Render the text content
        let lines: Vec<&str> = text_guarded.lines().collect();
        for (y, line) in lines.iter().enumerate() {
            if y >= height {
                break; // Avoid rendering past screen dimensions
            }
            args.surface.add_change(Change::CursorPosition {
                x: Position::Absolute(0),
                y: Position::Absolute(y),
            });
            args.surface.add_change(format!("{}\r\n", line));
        }
        // Render the status bar (mode, cursor position, etc.)
        self.status_bar.render(args, self.cursor_x, self.cursor_y);

        // Position the cursor based on its current coordinates
        args.surface.add_change(Change::CursorPosition {
            x: Position::Absolute(self.cursor_x),
            y: Position::Absolute(self.cursor_y),
        });

        // Set the cursor shape based on the current mode
        *args.cursor = CursorShapeAndPosition {
            coords: ParentRelativeCoords::new(self.cursor_x, self.cursor_y),
            shape: match self.mode {
                Modes::Normal => termwiz::surface::CursorShape::BlinkingBlock,
                Modes::Insert => termwiz::surface::CursorShape::BlinkingBar,
            },
            ..Default::default()
        };
    }
}
