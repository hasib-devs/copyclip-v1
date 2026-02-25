/// Keyboard actions: key presses, combinations, text input
/// Uses platform-specific implementations for cross-platform reliability

#[cfg(target_os = "macos")]
mod macos {
    use std::process::Command;

    pub fn press_key(key_name: &str) -> Result<(), String> {
        let binding = key_name.to_lowercase();
        let remapped = match binding.as_str() {
            "escape" => "Escape",
            "tab" => "Tab",
            "enter" | "return" => "Return",
            "backspace" => "BackSpace",
            "delete" => "Delete",
            "home" => "Home",
            "end" => "End",
            "pageup" | "page_up" => "Page_Up",
            "pagedown" | "page_down" => "Page_Down",
            "leftarrow" | "left" => "Left",
            "rightarrow" | "right" => "Right",
            "uparrow" | "up" => "Up",
            "downarrow" | "down" => "Down",
            "f1" => "F1",
            "f2" => "F2",
            "f3" => "F3",
            "f4" => "F4",
            "f5" => "F5",
            "f6" => "F6",
            "f7" => "F7",
            "f8" => "F8",
            "f9" => "F9",
            "f10" => "F10",
            "f11" => "F11",
            "f12" => "F12",
            "space" => "space",
            other => other,
        };

        Command::new("osascript")
            .arg("-e")
            .arg(format!(
                "tell application \"System Events\" to key code (name to key code \"{}\")",
                remapped
            ))
            .output()
            .map_err(|e| format!("Failed to execute osascript: {}", e))?;

        Ok(())
    }

    pub fn press_key_combination(keys: &[&str]) -> Result<(), String> {
        if keys.is_empty() {
            return Ok(());
        }

        // Build modifier key list
        let mut modifiers = Vec::new();
        for key in keys[..keys.len().saturating_sub(1)].iter() {
            match key.to_lowercase().as_str() {
                "shift" => modifiers.push("shift down"),
                "control" | "ctrl" => modifiers.push("control down"),
                "alt" => modifiers.push("option down"),
                "meta" | "cmd" | "command" => modifiers.push("command down"),
                _ => {}
            }
        }

        let last_key = keys[keys.len() - 1];
        let binding = last_key.to_lowercase();
        let remapped = match binding.as_str() {
            "escape" => "Escape",
            "tab" => "Tab",
            "enter" | "return" => "Return",
            "backspace" => "BackSpace",
            "delete" => "Delete",
            "home" => "Home",
            "end" => "End",
            "pageup" | "page_up" => "Page_Up",
            "pagedown" | "page_down" => "Page_Down",
            "leftarrow" | "left" => "Left",
            "rightarrow" | "right" => "Right",
            "uparrow" | "up" => "Up",
            "downarrow" | "down" => "Down",
            "f1" => "F1",
            "f2" => "F2",
            "f3" => "F3",
            "f4" => "F4",
            "f5" => "F5",
            "f6" => "F6",
            "f7" => "F7",
            "f8" => "F8",
            "f9" => "F9",
            "f10" => "F10",
            "f11" => "F11",
            "f12" => "F12",
            "space" => "space",
            other => other,
        };

        let mut script = "tell application \"System Events\"\n".to_string();
        for mod_key in &modifiers {
            script.push_str(&format!("  {}\n", mod_key));
        }
        script.push_str(&format!("  key code (name to key code \"{}\")\n", remapped));
        for _ in &modifiers {
            script.push_str("  key up\n");
        }
        script.push_str("end tell");

        Command::new("osascript")
            .arg("-e")
            .arg(&script)
            .output()
            .map_err(|e| format!("Failed to execute osascript: {}", e))?;

        Ok(())
    }

    pub fn type_text(text: &str) -> Result<(), String> {
        let escaped = text.replace("\"", "\\\"");
        Command::new("osascript")
            .arg("-e")
            .arg(format!(
                "tell application \"System Events\" to keystroke \"{}\"",
                escaped
            ))
            .output()
            .map_err(|e| format!("Failed to execute osascript: {}", e))?;

        Ok(())
    }
}

#[cfg(target_os = "windows")]
mod windows {
    use std::process::Command;

    pub fn press_key(key_name: &str) -> Result<(), String> {
        // Use PowerShell to send key presses
        let script = format!(
            "[System.Windows.Forms.SendKeys]::SendWait('{{{}}}') ",
            map_to_sendkeys(key_name)
        );

        Command::new("powershell")
            .arg("-NoProfile")
            .arg("-Command")
            .arg(format!(
                "Add-Type -AssemblyName System.Windows.Forms; {}",
                script
            ))
            .output()
            .map_err(|e| format!("Failed to execute PowerShell: {}", e))?;

        Ok(())
    }

    pub fn press_key_combination(keys: &[&str]) -> Result<(), String> {
        if keys.is_empty() {
            return Ok(());
        }

        let mut script = String::from("[System.Windows.Forms.SendKeys]::SendWait('");

        for key in keys {
            match key.to_lowercase().as_str() {
                "shift" => script.push('+'),
                "control" | "ctrl" => script.push('^'),
                "alt" => script.push('%'),
                "meta" | "cmd" | "command" => {} // Windows doesn't have a direct Win key equivalent
                other => script.push_str(&format!("{{{}}}", map_to_sendkeys(other))),
            }
        }

        script.push('\'');
        script.push(')');

        Command::new("powershell")
            .arg("-NoProfile")
            .arg("-Command")
            .arg(format!(
                "Add-Type -AssemblyName System.Windows.Forms; {}",
                script
            ))
            .output()
            .map_err(|e| format!("Failed to execute PowerShell: {}", e))?;

        Ok(())
    }

    pub fn type_text(text: &str) -> Result<(), String> {
        let escaped = text.replace("'", "''");
        Command::new("powershell")
            .arg("-NoProfile")
            .arg("-Command")
            .arg(format!(
                "Add-Type -AssemblyName System.Windows.Forms; [System.Windows.Forms.SendKeys]::SendWait('{}') ",
                escaped
            ))
            .output()
            .map_err(|e| format!("Failed to execute PowerShell: {}", e))?;

        Ok(())
    }

    fn map_to_sendkeys(key_name: &str) -> String {
        match key_name.to_lowercase().as_str() {
            "escape" => "ESC".to_string(),
            "tab" => "TAB".to_string(),
            "enter" | "return" => "ENTER".to_string(),
            "backspace" => "BACKSPACE".to_string(),
            "delete" => "DELETE".to_string(),
            "home" => "HOME".to_string(),
            "end" => "END".to_string(),
            "pageup" | "page_up" => "PGUP".to_string(),
            "pagedown" | "page_down" => "PGDN".to_string(),
            "leftarrow" | "left" => "LEFT".to_string(),
            "rightarrow" | "right" => "RIGHT".to_string(),
            "uparrow" | "up" => "UP".to_string(),
            "downarrow" | "down" => "DOWN".to_string(),
            "f1" => "F1".to_string(),
            "f2" => "F2".to_string(),
            "f3" => "F3".to_string(),
            "f4" => "F4".to_string(),
            "f5" => "F5".to_string(),
            "f6" => "F6".to_string(),
            "f7" => "F7".to_string(),
            "f8" => "F8".to_string(),
            "f9" => "F9".to_string(),
            "f10" => "F10".to_string(),
            "f11" => "F11".to_string(),
            "f12" => "F12".to_string(),
            "space" => "space".to_string(),
            other => other.to_string(),
        }
    }
}

#[cfg(target_os = "linux")]
mod linux {
    use std::process::Command;

    pub fn press_key(key_name: &str) -> Result<(), String> {
        let xdotool_key = match key_name.to_lowercase().as_str() {
            "escape" => "Escape",
            "tab" => "Tab",
            "enter" | "return" => "Return",
            "backspace" => "BackSpace",
            "delete" => "Delete",
            "home" => "Home",
            "end" => "End",
            "pageup" | "page_up" => "Page_Up",
            "pagedown" | "page_down" => "Page_Down",
            "leftarrow" | "left" => "Left",
            "rightarrow" | "right" => "Right",
            "uparrow" | "up" => "Up",
            "downarrow" | "down" => "Down",
            "f1" => "F1",
            "f2" => "F2",
            "f3" => "F3",
            "f4" => "F4",
            "f5" => "F5",
            "f6" => "F6",
            "f7" => "F7",
            "f8" => "F8",
            "f9" => "F9",
            "f10" => "F10",
            "f11" => "F11",
            "f12" => "F12",
            "space" => "space",
            other => other,
        };

        Command::new("xdotool")
            .args(&["key", xdotool_key])
            .output()
            .map_err(|e| format!("Failed to execute xdotool: {}", e))?;

        Ok(())
    }

    pub fn press_key_combination(keys: &[&str]) -> Result<(), String> {
        if keys.is_empty() {
            return Ok(());
        }

        let xdotool_keys: Vec<&str> = keys
            .iter()
            .map(|k| match k.to_lowercase().as_str() {
                "escape" => "Escape",
                "tab" => "Tab",
                "enter" | "return" => "Return",
                "backspace" => "BackSpace",
                "delete" => "Delete",
                "home" => "Home",
                "end" => "End",
                "pageup" | "page_up" => "Page_Up",
                "pagedown" | "page_down" => "Page_Down",
                "leftarrow" | "left" => "Left",
                "rightarrow" | "right" => "Right",
                "uparrow" | "up" => "Up",
                "downarrow" | "down" => "Down",
                "f1" => "F1",
                "f2" => "F2",
                "f3" => "F3",
                "f4" => "F4",
                "f5" => "F5",
                "f6" => "F6",
                "f7" => "F7",
                "f8" => "F8",
                "f9" => "F9",
                "f10" => "F10",
                "f11" => "F11",
                "f12" => "F12",
                "shift" => "shift",
                "control" | "ctrl" => "ctrl",
                "alt" => "alt",
                "meta" | "cmd" | "command" => "super",
                "space" => "space",
                other => other,
            })
            .collect();

        let combo = xdotool_keys.join("+");
        Command::new("xdotool")
            .args(&["key", &combo])
            .output()
            .map_err(|e| format!("Failed to execute xdotool: {}", e))?;

        Ok(())
    }

    pub fn type_text(text: &str) -> Result<(), String> {
        Command::new("xdotool")
            .args(&["type", text])
            .output()
            .map_err(|e| format!("Failed to execute xdotool: {}", e))?;

        Ok(())
    }
}

// Public API that dispatches to platform-specific implementation
pub fn press_key(key_name: &str) -> Result<(), String> {
    #[cfg(target_os = "macos")]
    return macos::press_key(key_name);
    #[cfg(target_os = "windows")]
    return windows::press_key(key_name);
    #[cfg(target_os = "linux")]
    return linux::press_key(key_name);
    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
    return Err("Unsupported operating system".to_string());
}

pub fn press_key_combination(keys: &[&str]) -> Result<(), String> {
    #[cfg(target_os = "macos")]
    return macos::press_key_combination(keys);
    #[cfg(target_os = "windows")]
    return windows::press_key_combination(keys);
    #[cfg(target_os = "linux")]
    return linux::press_key_combination(keys);
    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
    return Err("Unsupported operating system".to_string());
}

pub fn type_text(text: &str) -> Result<(), String> {
    #[cfg(target_os = "macos")]
    return macos::type_text(text);
    #[cfg(target_os = "windows")]
    return windows::type_text(text);
    #[cfg(target_os = "linux")]
    return linux::type_text(text);
    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
    return Err("Unsupported operating system".to_string());
}
