use std::cmp::Ordering;

use crate::screen::MainScreen;
use termwiz::cell::AttributeChange;
use termwiz::color::{AnsiColor, ColorAttribute};
use termwiz::surface::{self, Change, Position, Surface};
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
                self.yank.clone()
            );
            self.status_bar.update(&self.mode);
        }

        true // Always return true to indicate the UI should re-render
    }

    /// Render the screen content, including text and the status bar
    // NOTE: the rendering of the line of numbers are multiplied idk why?
    fn render(&mut self, args: &mut RenderArgs) {
        let text_guarded = self.text.lock().unwrap(); // Lock the content only briefly to access it
        let (width, height) = args.surface.dimensions();
        const GRV_COLOR_BACK: (u8, u8, u8) = (29, 32, 33);
        const WHITE: (u8, u8, u8) = (251, 241, 194);
        const YELLOW_NUMBER_LINES: (u8, u8, u8) = (0xFA, 0xBD, 0x2F); // #FABD2F

        // Clear the screen with Gruvbox dark background color
        args.surface.add_change(Change::ClearScreen(
            ColorAttribute::TrueColorWithPaletteFallback(
                (GRV_COLOR_BACK).into(),
                AnsiColor::Black.into(),
            ),
        ));

        // Calculate the width required for line numbers
        let line_number_width = (height as f64).log10().ceil() as usize + 1;
        // Determine how many lines of text we have
        let total_lines = text_guarded.lines().count();
        let effective_lines = if total_lines > 0 { total_lines } else { 1 };

        let content_height = height.saturating_sub(1);
        // Render the line numbers
        for y in 0..content_height {
            let line_text = match y.cmp(&effective_lines) {
                Ordering::Less => format!("{:width$} ", y + 1, width = line_number_width),
                Ordering::Equal => "~".to_string(),
                Ordering::Greater => "~".to_string(),
            };

            // warp up the widgets
            let number_of_lines_widget = vec![
                Change::CursorPosition {
                    x: Position::Absolute(0),
                    y: Position::Absolute(y),
                },
                // color the number of lines
                Change::Attribute(AttributeChange::Foreground(
                    ColorAttribute::TrueColorWithPaletteFallback(
                        (YELLOW_NUMBER_LINES).into(),
                        AnsiColor::White.into(),
                    ),
                )),
                // render the numbers
                Change::Text(line_text),
                // reset the color
                Change::Attribute(AttributeChange::Foreground(
                    ColorAttribute::TrueColorWithPaletteFallback(
                        (WHITE).into(),
                        AnsiColor::White.into(),
                    ),
                )),
            ];

            // render the number of lines
            for change in number_of_lines_widget {
                args.surface.add_change(change);
            }
            // Render the text
            if y < total_lines {
                let line = text_guarded.lines().nth(y).unwrap_or("");
                args.surface.add_change(Change::CursorPosition {
                    x: Position::Absolute(line_number_width + 1),
                    y: Position::Absolute(y),
                });
                args.surface.add_change(format!("{}\r\n", line));
            }
        }

        // Render the text content
        args.surface.add_change(Change::CursorPosition {
            x: Position::Absolute(0),
            y: Position::Absolute(content_height), // One line above the status bar
        });

        // Render the status bar (mode, cursor position, etc.)
        self.status_bar.render(args, self.cursor_x, self.cursor_y);

        // Position the cursor based on its current coordinates
        args.surface.add_change(Change::CursorPosition {
            x: Position::Absolute(self.cursor_x + line_number_width + 1),
            y: Position::Absolute(self.cursor_y),
        });

        // Set the cursor shape based on the current mode
        *args.cursor = CursorShapeAndPosition {
            coords: ParentRelativeCoords::new(self.cursor_x + line_number_width + 1, self.cursor_y),
            shape: match self.mode {
                Modes::Normal => termwiz::surface::CursorShape::BlinkingBlock,
                Modes::Insert => termwiz::surface::CursorShape::BlinkingBar,
            },
            color: ColorAttribute::TrueColorWithPaletteFallback(
                (WHITE).into(),
                AnsiColor::White.into(),
            ),
            ..Default::default()
        };
    }
}
