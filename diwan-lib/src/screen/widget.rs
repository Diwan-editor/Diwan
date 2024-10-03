use crate::screen::MainScreen;
use termwiz::cell::AttributeChange;
use termwiz::color::{AnsiColor, ColorAttribute};
use termwiz::surface::{Change, Position, Surface};
use termwiz::widgets::*;

use super::{Keymap, Modes, StatusBar};

impl Widget for MainScreen {
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
        true
    }
    fn render(&mut self, args: &mut RenderArgs) {
        let text_guarded = self.text.lock().unwrap();
        let dims = args.surface.dimensions();

        args.surface.add_change(Change::ClearScreen(
            ColorAttribute::TrueColorWithPaletteFallback(
                (0x1d, 0x20, 0x21).into(), // Gruvbox dark background
                AnsiColor::Black.into(),
            ),
        ));

        // Render the text
        let lines: Vec<&str> = text_guarded.lines().collect();
        for line in lines {
            args.surface.add_change(format!("{}\r\n", line));
        }

        // Update the status bar (left: mode, center: filename, right: cursor position)
        self.status_bar.render(args, self.cursor_x, self.cursor_y);

        // Place the cursor at the correct position
        args.surface.add_change(Change::CursorPosition {
            x: Position::Absolute(self.cursor_x),
            y: Position::Absolute(self.cursor_y),
        });

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
