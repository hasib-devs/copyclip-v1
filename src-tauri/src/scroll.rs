/// Cross-platform scroll event handling
/// Supports macOS, Windows, and Linux with different implementations

#[cfg(target_os = "macos")]
mod macos {
    pub fn scroll(vertical: i32, horizontal: i32) -> Result<(), String> {
        use core_graphics::event::CGEvent;
        use core_graphics::event_source::{CGEventSource, CGEventSourceStateID};

        // Send scroll wheel events
        // macOS scroll: positive = scroll down, negative = scroll up
        if vertical != 0 {
            if let Ok(source) = CGEventSource::new(CGEventSourceStateID::HIDSystemState) {
                if let Ok(event) =
                    CGEvent::new_scroll_event(source, vertical.abs() as u32, 0, 0, 0, 0)
                {
                    event.post(core_graphics::event::CGEventTapLocation::HID);
                }
            }
            std::thread::sleep(std::time::Duration::from_millis(5));
        }

        if horizontal != 0 {
            if let Ok(source) = CGEventSource::new(CGEventSourceStateID::HIDSystemState) {
                if let Ok(event) =
                    CGEvent::new_scroll_event(source, 0, horizontal.abs() as u32, 0, 0, 0)
                {
                    event.post(core_graphics::event::CGEventTapLocation::HID);
                }
            }
            std::thread::sleep(std::time::Duration::from_millis(5));
        }

        Ok(())
    }
}

#[cfg(target_os = "windows")]
mod windows {
    use winapi::um::winuser::{mouse_event, MOUSEEVENTF_HWHEEL, MOUSEEVENTF_WHEEL};

    pub fn scroll(vertical: i32, horizontal: i32) -> Result<(), String> {
        unsafe {
            // Windows uses 120 units per scroll notch
            // Positive = scroll down/right, Negative = scroll up/left

            if vertical != 0 {
                let wheel_delta = (vertical * 120 / 10) as u32;
                mouse_event(MOUSEEVENTF_WHEEL, 0, 0, wheel_delta, 0);
            }

            if horizontal != 0 {
                let wheel_delta = (horizontal * 120 / 10) as u32;
                mouse_event(MOUSEEVENTF_HWHEEL, 0, 0, wheel_delta, 0);
            }

            Ok(())
        }
    }
}

#[cfg(target_os = "linux")]
mod linux {
    use std::process::Command;

    pub fn scroll(vertical: i32, horizontal: i32) -> Result<(), String> {
        // Vertical scrolling: positive = scroll down, negative = scroll up
        if vertical != 0 {
            let button = if vertical > 0 { "5" } else { "4" };
            let count = vertical.abs() as u32;
            for _ in 0..count {
                Command::new("xdotool")
                    .args(&["click", button])
                    .output()
                    .map_err(|e| format!("Failed to execute xdotool: {}", e))?;
            }
        }

        // Horizontal scrolling: positive = scroll right, negative = scroll left
        if horizontal != 0 {
            let button = if horizontal > 0 { "7" } else { "6" };
            let count = horizontal.abs() as u32;
            for _ in 0..count {
                Command::new("xdotool")
                    .args(&["click", button])
                    .output()
                    .map_err(|e| format!("Failed to execute xdotool: {}", e))?;
            }
        }

        Ok(())
    }
}

/// Platform-independent scroll interface
pub fn scroll(vertical: i32, horizontal: i32) -> Result<(), String> {
    #[cfg(target_os = "macos")]
    return macos::scroll(vertical, horizontal);

    #[cfg(target_os = "windows")]
    return windows::scroll(vertical, horizontal);

    #[cfg(target_os = "linux")]
    return linux::scroll(vertical, horizontal);

    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
    {
        Err("Scroll not supported on this platform".to_string())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_scroll_builds() {
        // Just verify the module can be imported and referenced
        eprintln!("Scroll module for platform: {}", std::env::consts::OS);
    }
}
