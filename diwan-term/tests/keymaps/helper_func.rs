use anyhow::{Error as HelperError, Result as AnyhowResult};
use termwiz::{
    input::{KeyCode, KeyEvent, Modifiers},
    widgets::WidgetEvent,
};

/// Constructs a `WidgetEvent` for a given key and modifiers.
///
/// # Arguments
/// * `k` - The character representing the key.
/// * `modi` - The modifiers to apply (e.g., CTRL, ALT).
///
/// # Returns
/// * `Ok(WidgetEvent)` - The constructed key event.
/// * `Err(HelperError)` - If an error occurs.
pub fn apply_keybind(k: KeyCode, modi: Modifiers) -> AnyhowResult<WidgetEvent, HelperError> {
    let key = WidgetEvent::Input(termwiz::input::InputEvent::Key(KeyEvent {
        key: k,
        modifiers: modi,
    }));

    Ok(key)
}

/// Asserts the cursor's X and Y positions.
///
/// # Arguments
/// * `assertions` - A slice of `(expected, actual, message)` tuples.
///
/// # Panics
/// Panics if `expected` and `actual` do not match, displaying `message`.
pub fn assert_cursor(assertions: &[(usize, usize, &str)]) {
    for (expected, actual, msg) in assertions {
        assert_eq!(expected, actual, "{}", msg);
    }
}

/// Asserts the emptiness of content and buffers.
///
/// # Arguments
/// * `assertions` - A slice of `(expected, actual, message)` tuples:
///     - `expected`: The expected boolean value (e.g., `true` for empty).
///     - `actual`: The actual boolean value to check.
///     - `msg`: A descriptive message for failed assertions.
///
/// # Panics
/// Panics if `expected` and `actual` values do not match, displaying the `msg`.
pub fn assert_emptiness_content(assertions: &[(bool, bool, &str)]) {
    for (expected, actual, msg) in assertions {
        assert_eq!(expected, actual, "{}", msg);
    }
}
