use termwiz::widgets::*;
use crate::screen::MainScreen;
use termwiz::input::*;
use termwiz::color::{AnsiColor, ColorAttribute};
use termwiz::surface::Change;
use termwiz::cell::AttributeChange;
use termwiz::terminal::Terminal;

impl<'a, T> Widget for MainScreen<'a, T> where T: Terminal {
    fn process_event(&mut self, event: &WidgetEvent, _args: &mut UpdateArgs) -> bool {
        match event {
            WidgetEvent::Input(InputEvent::Key(KeyEvent {
                key: KeyCode::Char(c),
                ..
            })) => self.text.push(*c),
            WidgetEvent::Input(InputEvent::Key(KeyEvent {
                key: KeyCode::Enter,
                ..
            })) => {
                self.text.push_str("\r\n");
            }
            WidgetEvent::Input(InputEvent::Paste(s)) => {
                self.text.push_str(&s);
            }
            _ => {}
        }

        true
    }

    /// Draw ourselves into the surface provided by RenderArgs
    fn render(&mut self, args: &mut RenderArgs) {
        args.surface.add_change(Change::ClearScreen(
            ColorAttribute::TrueColorWithPaletteFallback(
                (0x31, 0x1B, 0x92).into(),
                AnsiColor::Black.into(),
            ),
        ));
        args.surface
            .add_change(Change::Attribute(AttributeChange::Foreground(
                ColorAttribute::TrueColorWithPaletteFallback(
                    (0xB3, 0x88, 0xFF).into(),
                    AnsiColor::Purple.into(),
                ),
            )));
        let dims = args.surface.dimensions();
        args.surface
            .add_change(format!("🤷 surface size is {:?}\r\n", dims));
        args.surface.add_change(self.text.clone());

        // Place the cursor at the end of the text.
        // A more advanced text editing widget would manage the
        // cursor position differently.
        *args.cursor = CursorShapeAndPosition {
            coords: args.surface.cursor_position().into(),
            shape: termwiz::surface::CursorShape::SteadyBar,
            ..
                Default::default()
        };
    }

    fn get_size_constraints(&self) -> layout::Constraints {
        layout::Constraints::with_fixed_width_height(80, 80)
    }
}
