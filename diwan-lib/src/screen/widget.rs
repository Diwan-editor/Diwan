use crate::screen::MainScreen;
use termwiz::cell::AttributeChange;
use termwiz::color::{AnsiColor, ColorAttribute};
use termwiz::input::*;
use termwiz::surface::{Change, Position, Surface};
use termwiz::widgets::*;

use super::{Keymap, StatusBar};
unsafe impl<'a> Send for MainScreen<'a> {}
unsafe impl<'a> Sync for MainScreen<'a> {}

impl<'a> Widget for MainScreen<'a> {
    fn process_event(&mut self, event: &WidgetEvent, _args: &mut UpdateArgs) -> bool {
        // let mode = Modes::Normal;
        // Keymap::map_key_to_action(event, &mode);
        if let Some(action) = Keymap::map_key_to_action(event, &self.mode) {
            // Use the `handle_action` function to update the state of `MainScreen`
            Keymap::handle_action(action, self.text, &mut self.cursor_pos, &mut self.mode);
            self.status_bar.update(&self.mode);
        }
        // match event {
        //     WidgetEvent::Input(InputEvent::Key(KeyEvent {
        //         key: KeyCode::Char(c),
        //         ..
        //     })) => self.text.push(*c),
        //     WidgetEvent::Input(InputEvent::Key(KeyEvent {
        //         key: KeyCode::Enter,
        //         ..
        //     })) => {
        //         self.text.push_str("\r\n");
        //     }
        //     WidgetEvent::Input(InputEvent::Paste(s)) => {
        //         self.text.push_str(&s);
        //     }
        //     _ => {}
        // }

        true
    }

    /// Draw ourselves into the surface provided by RenderArgs
    fn render(&mut self, args: &mut RenderArgs) {
        // Apply a dark background and light foreground for dark mode
        args.surface.add_change(Change::ClearScreen(
            ColorAttribute::TrueColorWithPaletteFallback(
                (0x00, 0x00, 0x00).into(), // Pure black background
                AnsiColor::Black.into(),
            ),
        ));
        args.surface
            .add_change(Change::Attribute(AttributeChange::Foreground(
                ColorAttribute::TrueColorWithPaletteFallback(
                    (0xB3, 0x88, 0xFF).into(),
                    AnsiColor::Green.into(),
                ),
            )));
        let dims = args.surface.dimensions();
        args.surface
            .add_change(format!("🤷 surface size is {:?}\r\n", dims));
        args.surface.add_change(self.text.clone());
        self.status_bar.render(args);
        // Place the cursor at the end of the text.
        // A more advanced text editing widget would manage the
        // cursor position differently.
        *args.cursor = CursorShapeAndPosition {
            coords: args.surface.cursor_position().into(),
            shape: termwiz::surface::CursorShape::SteadyBar,
            ..Default::default()
        };
    }

    // fn get_size_constraints(&self) -> layout::Constraints {
    //    let (w, h) = Surface::dimensions();
    //     layout::Constraints::with_fixed_width_height(80, 80)
    // }
}

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
