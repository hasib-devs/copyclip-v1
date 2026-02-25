/// Cross-platform scroll event handling
/// Supports macOS, Windows, and Linux with different implementations

#[cfg(target_os = "macos")]
mod macos {
    use std::thread;
    use std::time::Duration;

    pub fn scroll(vertical: i32, horizontal: i32) -> Result<(), String> {
        unsafe {
            use core_graphics::event::{CGEvent, CGEventType};
            use core_graphics::event_source::{CGEventSource, CGEventSourceStateID};
            use core_graphics::geometry::CGPoint;

            // Get current cursor location
            let location =
                if let Ok(source) = CGEventSource::new(CGEventSourceStateID::HIDSystemState) {
                    if let Ok(evt) = CGEvent::new(source) {
                        evt.location()
                    } else {
                        // Fallback to screen center
                        CGPoint::new(500.0, 500.0)
                    }
                } else {
                    return Err("Failed to create event source".to_string());
                };

            // Send scroll wheel events
            // macOS scroll: positive = scroll down, negative = scroll up
            if vertical != 0 {
                if let Ok(source) = CGEventSource::new(CGEventSourceStateID::HIDSystemState) {
                    if let Ok(event) = CGEvent::new_scroll_event(
                        source,
                        (vertical as i64).try_into().unwrap(),
                        0,
                        0,
                        0,
                        0,
                    ) {
                        event.post(core_graphics::event::CGEventTapLocation::HID);
                    }
                }
                thread::sleep(Duration::from_millis(5));
            }

            if horizontal != 0 {
                if let Ok(source) = CGEventSource::new(CGEventSourceStateID::HIDSystemState) {
                    if let Ok(event) =
                        CGEvent::new_scroll_event(source, 0, horizontal as u32, 0, 0, 0)
                    {
                        event.post(core_graphics::event::CGEventTapLocation::HID);
                    }
                }
                thread::sleep(Duration::from_millis(5));
            }

            Ok(())
        }
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
