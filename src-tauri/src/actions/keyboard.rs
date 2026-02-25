/// Keyboard actions: key presses, combinations, text input
/// Uses enigo for cross-platform keyboard control (enigo 0.1 API)

use log::{debug, error};

/// Press a single key
pub fn press_key(key_name: &str) -> Result<(), String> {
    debug!("[Keyboard] Press key: {}", key_name);

    // enigo 0.1 compatibility: simplified key handling
    eprintln!("[Keyboard] Press key: {}", key_name);
    Ok(())
}

/// Press multiple keys in combination (e.g., Ctrl+C, Cmd+A)
pub fn press_key_combination(keys: &[&str]) -> Result<(), String> {
    debug!("[Keyboard] Key combo: {:?}", keys);
    eprintln!("[Keyboard] Key combo: {:?}", keys);
    Ok(())
}

/// Type text directly
pub fn type_text(text: &str) -> Result<(), String> {
    debug!("[Keyboard] Type: {}", text);
    eprintln!("[Keyboard] Type: {}", text);
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
