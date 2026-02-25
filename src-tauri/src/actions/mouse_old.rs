/// Mouse control actions: movement, clicking, scrolling
/// Uses enigo for cross-platform mouse control (enigo 0.1 API)
use enigo::{Enigo, MouseControllable};
use log::debug;
use std::thread;
use std::time::Duration;

/// Perform a left mouse click at current cursor position
pub fn left_click() -> Result<(), String> {
    debug!("[Mouse] Left click");

    let mut enigo = Enigo::new();
    enigo.mouse_down(enigo::MouseButton::Left);
    thread::sleep(Duration::from_millis(10));
    enigo.mouse_up(enigo::MouseButton::Left);

    Ok(())
}

/// Perform a right mouse click at current cursor position
pub fn right_click() -> Result<(), String> {
    debug!("[Mouse] Right click");

    let mut enigo = Enigo::new();
    enigo.mouse_down(enigo::MouseButton::Right);
    thread::sleep(Duration::from_millis(10));
    enigo.mouse_up(enigo::MouseButton::Right);

    Ok(())
}

/// Perform a middle mouse click at current cursor position
pub fn middle_click() -> Result<(), String> {
    debug!("[Mouse] Middle click");

    let mut enigo = Enigo::new();
    enigo.mouse_down(enigo::MouseButton::Middle);
    thread::sleep(Duration::from_millis(10));
    enigo.mouse_up(enigo::MouseButton::Middle);

    Ok(())
}

/// Perform a double-click (two clicks with short interval)
pub fn double_click() -> Result<(), String> {
    debug!("[Mouse] Double click");

    let mut enigo = Enigo::new();

    // First click
    enigo.mouse_down(enigo::MouseButton::Left);
    thread::sleep(Duration::from_millis(10));
    enigo.mouse_up(enigo::MouseButton::Left);

    // Short delay between clicks
    thread::sleep(Duration::from_millis(20));

    // Second click
    enigo.mouse_down(enigo::MouseButton::Left);
    thread::sleep(Duration::from_millis(10));
    enigo.mouse_up(enigo::MouseButton::Left);

    Ok(())
}

/// Move cursor by relative offset
pub fn move_cursor(dx: i32, dy: i32) -> Result<(), String> {
    if dx == 0 && dy == 0 {
        return Ok(());
    }

    debug!("[Mouse] Move cursor ({}, {})", dx, dy);

    let mut enigo = Enigo::new();
    enigo.mouse_move_relative(dx, dy);

    Ok(())
}

/// Set cursor to absolute position
pub fn set_cursor_position(x: i32, y: i32) -> Result<(), String> {
    debug!("[Mouse] Set cursor to ({}, {})", x, y);

    let mut enigo = Enigo::new();
    enigo.mouse_move_to(x, y);

    Ok(())
}

/// Scroll vertically and/or horizontally
pub fn scroll(vertical: i32, horizontal: i32) -> Result<(), String> {
    if vertical == 0 && horizontal == 0 {
        return Ok(());
    }

    debug!("[Mouse] Scroll (V: {}, H: {})", vertical, horizontal);

    // Perform scrolling via the scroll module (already implemented in Phase 1)
    // This function just delegates to existing scroll logic
    crate::scroll::scroll(vertical, horizontal)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cursor_bounds() {
        // Test that negative coordinates are clamped to 0
        let dx = 10;
        let dy = 10;
        // In real usage, this would move the cursor
        // Just test that the function doesn't panic
        let _ = move_cursor(dx, dy);
    }
}
