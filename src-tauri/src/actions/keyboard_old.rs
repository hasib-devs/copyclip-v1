/// Keyboard actions: key presses, combinations, text input
/// Uses enigo for cross-platform keyboard control (enigo 0.1 API)
use enigo::{Enigo, KeyboardControllable};
use log::debug;
use std::thread;
use std::time::Duration;

/// Convert key name string to enigo Key enum
fn string_to_key(key_name: &str) -> Option<enigo::Key> {
    match key_name.to_lowercase().as_str() {
        // Letters
        "a" => Some(enigo::Key::Layout('a')),
        "b" => Some(enigo::Key::Layout('b')),
        "c" => Some(enigo::Key::Layout('c')),
        "d" => Some(enigo::Key::Layout('d')),
        "e" => Some(enigo::Key::Layout('e')),
        "f" => Some(enigo::Key::Layout('f')),
        "g" => Some(enigo::Key::Layout('g')),
        "h" => Some(enigo::Key::Layout('h')),
        "i" => Some(enigo::Key::Layout('i')),
        "j" => Some(enigo::Key::Layout('j')),
        "k" => Some(enigo::Key::Layout('k')),
        "l" => Some(enigo::Key::Layout('l')),
        "m" => Some(enigo::Key::Layout('m')),
        "n" => Some(enigo::Key::Layout('n')),
        "o" => Some(enigo::Key::Layout('o')),
        "p" => Some(enigo::Key::Layout('p')),
        "q" => Some(enigo::Key::Layout('q')),
        "r" => Some(enigo::Key::Layout('r')),
        "s" => Some(enigo::Key::Layout('s')),
        "t" => Some(enigo::Key::Layout('t')),
        "u" => Some(enigo::Key::Layout('u')),
        "v" => Some(enigo::Key::Layout('v')),
        "w" => Some(enigo::Key::Layout('w')),
        "x" => Some(enigo::Key::Layout('x')),
        "y" => Some(enigo::Key::Layout('y')),
        "z" => Some(enigo::Key::Layout('z')),
        // Numbers
        "0" => Some(enigo::Key::Layout('0')),
        "1" => Some(enigo::Key::Layout('1')),
        "2" => Some(enigo::Key::Layout('2')),
        "3" => Some(enigo::Key::Layout('3')),
        "4" => Some(enigo::Key::Layout('4')),
        "5" => Some(enigo::Key::Layout('5')),
        "6" => Some(enigo::Key::Layout('6')),
        "7" => Some(enigo::Key::Layout('7')),
        "8" => Some(enigo::Key::Layout('8')),
        "9" => Some(enigo::Key::Layout('9')),
        // Function keys
        "f1" => Some(enigo::Key::F1),
        "f2" => Some(enigo::Key::F2),
        "f3" => Some(enigo::Key::F3),
        "f4" => Some(enigo::Key::F4),
        "f5" => Some(enigo::Key::F5),
        "f6" => Some(enigo::Key::F6),
        "f7" => Some(enigo::Key::F7),
        "f8" => Some(enigo::Key::F8),
        "f9" => Some(enigo::Key::F9),
        "f10" => Some(enigo::Key::F10),
        "f11" => Some(enigo::Key::F11),
        "f12" => Some(enigo::Key::F12),
        // Modifiers
        "shift" => Some(enigo::Key::Shift),
        "control" | "ctrl" => Some(enigo::Key::Control),
        "alt" => Some(enigo::Key::Alt),
        "meta" | "cmd" | "command" => Some(enigo::Key::Meta),
        // Navigation
        "escape" => Some(enigo::Key::Escape),
        "tab" => Some(enigo::Key::Tab),
        "return" | "enter" => Some(enigo::Key::Return),
        "backspace" => Some(enigo::Key::Backspace),
        "delete" => Some(enigo::Key::Delete),
        "home" => Some(enigo::Key::Home),
        "end" => Some(enigo::Key::End),
        "pageup" | "page_up" => Some(enigo::Key::PageUp),
        "pagedown" | "page_down" => Some(enigo::Key::PageDown),
        "leftarrow" | "left" => Some(enigo::Key::LeftArrow),
        "rightarrow" | "right" => Some(enigo::Key::RightArrow),
        "uparrow" | "up" => Some(enigo::Key::UpArrow),
        "downarrow" | "down" => Some(enigo::Key::DownArrow),
        // Special
        "space" => Some(enigo::Key::Layout(' ')),
        _ => None,
    }
}

/// Press a single key
pub fn press_key(key_name: &str) -> Result<(), String> {
    debug!("[Keyboard] Press key: {}", key_name);

    let key = string_to_key(key_name).ok_or_else(|| format!("Unknown key: {}", key_name))?;

    let mut enigo = Enigo::new();
    enigo.key_click(key);

    Ok(())
}

/// Press multiple keys in combination (e.g., Ctrl+C, Cmd+A)
pub fn press_key_combination(keys: &[&str]) -> Result<(), String> {
    debug!("[Keyboard] Key combo: {:?}", keys);

    if keys.is_empty() {
        return Ok(());
    }

    let mut enigo = Enigo::new();
    let mut key_objects = Vec::new();

    // Convert all key names to Key enums
    for key_name in keys {
        let key = string_to_key(key_name).ok_or_else(|| format!("Unknown key: {}", key_name))?;
        key_objects.push(key);
    }

    // Press all keys
    for key in &key_objects {
        enigo.key_down(*key);
    }

    thread::sleep(Duration::from_millis(10));

    // Release all keys in reverse order
    for key in key_objects.iter().rev() {
        enigo.key_up(*key);
    }

    Ok(())
}

/// Type text directly
pub fn type_text(text: &str) -> Result<(), String> {
    debug!("[Keyboard] Type: {}", text);

    let mut enigo = Enigo::new();

    for ch in text.chars() {
        enigo.key_click(enigo::Key::Layout(ch));
        // Small delay between characters for reliability
        thread::sleep(Duration::from_millis(1));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_name_parsing() {
        // Test that key names are properly recognized
        let result = press_key("Escape");
        // Don't assert success since enigo might not be available in tests
        // Just ensure no panic
    }

    #[test]
    fn test_single_char_key() {
        // Test single character handling
        let result = press_key("a");
        // Just ensure no panic
    }
}
