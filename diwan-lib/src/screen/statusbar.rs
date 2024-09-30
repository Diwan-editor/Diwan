use termwiz::cell::AttributeChange;
use termwiz::color::{AnsiColor, ColorAttribute};
use termwiz::input::*;
use termwiz::surface::{Change, Position, Surface};
use termwiz::widgets::*;

use super::Modes;

pub struct StatusBar {
    pub status_text: String, // Make it a String so we can modify it
}

impl StatusBar {
    pub fn new(status_text: &str) -> Self {
        Self {
            status_text: status_text.to_string(),
        }
    }

    pub fn update(&mut self, mode: &Modes) {
        self.status_text = format!("Mode: {:?}", mode);
    }
}

/// widget for the statusbar
impl Widget for StatusBar {
    fn process_event(&mut self, _event: &WidgetEvent, _args: &mut UpdateArgs) -> bool {
        // The status bar is static and doesn't need to process events in this example.
        false
    }

    fn render(&mut self, args: &mut RenderArgs) {
        let dims = args.surface.dimensions();
        let status_text_padded = format!("{:<width$}", self.status_text, width = dims.0);
        args.surface.add_change(Change::CursorPosition {
            x: Position::Relative(0), // x position at the start of the line
            y: Position::Relative(((dims.1 - 1) as u16).try_into().unwrap()), // y position at the last row
        });
        args.surface
            .add_change(Change::Attribute(AttributeChange::Foreground(
                ColorAttribute::TrueColorWithPaletteFallback(
                    (0xFF, 0xFF, 0xFF).into(), // White text
                    AnsiColor::White.into(),
                ),
            )));
        args.surface
            .add_change(Change::Attribute(AttributeChange::Background(
                ColorAttribute::TrueColorWithPaletteFallback(
                    (0x80, 0x00, 0x80).into(), // Black background
                    AnsiColor::Purple.into(),
                ),
            )));
        args.surface.add_change(Change::Text(status_text_padded));
    }
}
