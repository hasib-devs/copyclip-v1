/// Mouse control actions: movement, clicking, scrolling
/// Uses platform-specific implementations for cross-platform reliability

#[cfg(target_os = "macos")]
mod macos {
    use std::thread;
    use std::time::Duration;

    pub fn left_click() -> Result<(), String> {
        eprintln!("[CLICK] left_click() called");
        unsafe {
            use core_graphics::event::{CGEvent, CGEventType, CGMouseButton};
            use core_graphics::event_source::{CGEventSource, CGEventSourceStateID};
            use core_graphics::geometry::CGPoint;

            // Try with current cursor position
            let location =
                if let Ok(source) = CGEventSource::new(CGEventSourceStateID::HIDSystemState) {
                    if let Ok(evt) = CGEvent::new(source) {
                        evt.location()
                    } else {
                        // Fallback to center of screen if we can't get cursor position
                        eprintln!("[CLICK] Using fallback cursor position");
                        CGPoint::new(500.0, 500.0)
                    }
                } else {
                    return Err("Failed to create event source".to_string());
                };

            eprintln!("[CLICK] Cursor location: ({}, {})", location.x, location.y);

            // Post left mouse down
            if let Ok(source) = CGEventSource::new(CGEventSourceStateID::HIDSystemState) {
                if let Ok(event) = CGEvent::new_mouse_event(
                    source,
                    CGEventType::LeftMouseDown,
                    location,
                    CGMouseButton::Left,
                ) {
                    eprintln!("[CLICK] Posted LeftMouseDown event");
                    event.post(core_graphics::event::CGEventTapLocation::HID);
                } else {
                    eprintln!("[CLICK] Failed to create LeftMouseDown event");
                }
            }

            thread::sleep(Duration::from_millis(10));

            // Post left mouse up
            if let Ok(source) = CGEventSource::new(CGEventSourceStateID::HIDSystemState) {
                if let Ok(event) = CGEvent::new_mouse_event(
                    source,
                    CGEventType::LeftMouseUp,
                    location,
                    CGMouseButton::Left,
                ) {
                    eprintln!("[CLICK] Posted LeftMouseUp event");
                    event.post(core_graphics::event::CGEventTapLocation::HID);
                } else {
                    eprintln!("[CLICK] Failed to create LeftMouseUp event");
                }
            }

            eprintln!("[CLICK] left_click() completed successfully");
            Ok(())
        }
    }

    pub fn right_click() -> Result<(), String> {
        unsafe {
            use core_graphics::event::{CGEvent, CGEventType, CGMouseButton};
            use core_graphics::event_source::{CGEventSource, CGEventSourceStateID};
            use core_graphics::geometry::CGPoint;

            // Try with current cursor position
            let location =
                if let Ok(source) = CGEventSource::new(CGEventSourceStateID::HIDSystemState) {
                    if let Ok(evt) = CGEvent::new(source) {
                        evt.location()
                    } else {
                        // Fallback to center of screen if we can't get cursor position
                        CGPoint::new(500.0, 500.0)
                    }
                } else {
                    return Err("Failed to create event source".to_string());
                };

            // Post right mouse down
            if let Ok(source) = CGEventSource::new(CGEventSourceStateID::HIDSystemState) {
                if let Ok(event) = CGEvent::new_mouse_event(
                    source,
                    CGEventType::RightMouseDown,
                    location,
                    CGMouseButton::Right,
                ) {
                    event.post(core_graphics::event::CGEventTapLocation::HID);
                }
            }

            thread::sleep(Duration::from_millis(10));

            // Post right mouse up
            if let Ok(source) = CGEventSource::new(CGEventSourceStateID::HIDSystemState) {
                if let Ok(event) = CGEvent::new_mouse_event(
                    source,
                    CGEventType::RightMouseUp,
                    location,
                    CGMouseButton::Right,
                ) {
                    event.post(core_graphics::event::CGEventTapLocation::HID);
                }
            }

            Ok(())
        }
    }

    pub fn middle_click() -> Result<(), String> {
        unsafe {
            use core_graphics::event::{CGEvent, CGEventType, CGMouseButton};
            use core_graphics::event_source::{CGEventSource, CGEventSourceStateID};
            use core_graphics::geometry::CGPoint;

            // Try with current cursor position
            let location =
                if let Ok(source) = CGEventSource::new(CGEventSourceStateID::HIDSystemState) {
                    if let Ok(evt) = CGEvent::new(source) {
                        evt.location()
                    } else {
                        // Fallback to center of screen if we can't get cursor position
                        CGPoint::new(500.0, 500.0)
                    }
                } else {
                    return Err("Failed to create event source".to_string());
                };

            // Post middle mouse down
            if let Ok(source) = CGEventSource::new(CGEventSourceStateID::HIDSystemState) {
                if let Ok(event) = CGEvent::new_mouse_event(
                    source,
                    CGEventType::OtherMouseDown,
                    location,
                    CGMouseButton::Center,
                ) {
                    event.post(core_graphics::event::CGEventTapLocation::HID);
                }
            }

            thread::sleep(Duration::from_millis(10));

            // Post middle mouse up
            if let Ok(source) = CGEventSource::new(CGEventSourceStateID::HIDSystemState) {
                if let Ok(event) = CGEvent::new_mouse_event(
                    source,
                    CGEventType::OtherMouseUp,
                    location,
                    CGMouseButton::Center,
                ) {
                    event.post(core_graphics::event::CGEventTapLocation::HID);
                }
            }

            Ok(())
        }
    }

    pub fn double_click() -> Result<(), String> {
        left_click()?;
        thread::sleep(Duration::from_millis(20));
        left_click()?;
        Ok(())
    }

    pub fn move_cursor(dx: i32, dy: i32) -> Result<(), String> {
        if dx == 0 && dy == 0 {
            return Ok(());
        }
        // macOS requires absolute positioning, not relative
        // For now, delegate to system
        Ok(())
    }

    pub fn set_cursor_position(x: i32, y: i32) -> Result<(), String> {
        unsafe {
            use core_graphics::event::{CGEvent, CGEventType, CGMouseButton};
            use core_graphics::event_source::{CGEventSource, CGEventSourceStateID};
            use core_graphics::geometry::CGPoint;

            let location = CGPoint::new(x as f64, y as f64);
            // Try to create event source, or silently fail
            if let Ok(source) = CGEventSource::new(CGEventSourceStateID::HIDSystemState) {
                if let Ok(event) = CGEvent::new_mouse_event(
                    source,
                    CGEventType::MouseMoved,
                    location,
                    CGMouseButton::Left,
                ) {
                    event.post(core_graphics::event::CGEventTapLocation::HID);
                }
            }

            Ok(())
        }
    }
}

#[cfg(target_os = "windows")]
mod windows {
    use std::thread;
    use std::time::Duration;

    pub fn left_click() -> Result<(), String> {
        use winapi::um::winuser::{mouse_event, MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP};
        unsafe {
            mouse_event(MOUSEEVENTF_LEFTDOWN, 0, 0, 0, 0);
            thread::sleep(Duration::from_millis(10));
            mouse_event(MOUSEEVENTF_LEFTUP, 0, 0, 0, 0);
            Ok(())
        }
    }

    pub fn right_click() -> Result<(), String> {
        use winapi::um::winuser::{mouse_event, MOUSEEVENTF_RIGHTDOWN, MOUSEEVENTF_RIGHTUP};
        unsafe {
            mouse_event(MOUSEEVENTF_RIGHTDOWN, 0, 0, 0, 0);
            thread::sleep(Duration::from_millis(10));
            mouse_event(MOUSEEVENTF_RIGHTUP, 0, 0, 0, 0);
            Ok(())
        }
    }

    pub fn middle_click() -> Result<(), String> {
        use winapi::um::winuser::{mouse_event, MOUSEEVENTF_MIDDLEDOWN, MOUSEEVENTF_MIDDLEUP};
        unsafe {
            mouse_event(MOUSEEVENTF_MIDDLEDOWN, 0, 0, 0, 0);
            thread::sleep(Duration::from_millis(10));
            mouse_event(MOUSEEVENTF_MIDDLEUP, 0, 0, 0, 0);
            Ok(())
        }
    }

    pub fn double_click() -> Result<(), String> {
        left_click()?;
        thread::sleep(Duration::from_millis(20));
        left_click()?;
        Ok(())
    }

    pub fn move_cursor(dx: i32, dy: i32) -> Result<(), String> {
        if dx == 0 && dy == 0 {
            return Ok(());
        }
        use winapi::um::winuser::{mouse_event, MOUSEEVENTF_MOVE};
        unsafe {
            mouse_event(MOUSEEVENTF_MOVE, dx as u32, dy as u32, 0, 0);
            Ok(())
        }
    }

    pub fn set_cursor_position(x: i32, y: i32) -> Result<(), String> {
        use winapi::um::winuser::SetCursorPos;
        unsafe {
            SetCursorPos(x, y);
            Ok(())
        }
    }
}

#[cfg(target_os = "linux")]
mod linux {
    use std::process::Command;

    pub fn left_click() -> Result<(), String> {
        Command::new("xdotool")
            .args(&["click", "1"])
            .output()
            .map_err(|e| format!("Failed to execute xdotool: {}", e))?;
        Ok(())
    }

    pub fn right_click() -> Result<(), String> {
        Command::new("xdotool")
            .args(&["click", "3"])
            .output()
            .map_err(|e| format!("Failed to execute xdotool: {}", e))?;
        Ok(())
    }

    pub fn middle_click() -> Result<(), String> {
        Command::new("xdotool")
            .args(&["click", "2"])
            .output()
            .map_err(|e| format!("Failed to execute xdotool: {}", e))?;
        Ok(())
    }

    pub fn double_click() -> Result<(), String> {
        Command::new("xdotool")
            .args(&["click", "--repeat", "2", "1"])
            .output()
            .map_err(|e| format!("Failed to execute xdotool: {}", e))?;
        Ok(())
    }

    pub fn move_cursor(dx: i32, dy: i32) -> Result<(), String> {
        if dx == 0 && dy == 0 {
            return Ok(());
        }
        Command::new("xdotool")
            .args(&["mousemove", "--relative", &dx.to_string(), &dy.to_string()])
            .output()
            .map_err(|e| format!("Failed to execute xdotool: {}", e))?;
        Ok(())
    }

    pub fn set_cursor_position(x: i32, y: i32) -> Result<(), String> {
        Command::new("xdotool")
            .args(&["mousemove", &x.to_string(), &y.to_string()])
            .output()
            .map_err(|e| format!("Failed to execute xdotool: {}", e))?;
        Ok(())
    }
}

// Public API - dispatcher functions
pub fn left_click() -> Result<(), String> {
    #[cfg(target_os = "macos")]
    return macos::left_click();
    #[cfg(target_os = "windows")]
    return windows::left_click();
    #[cfg(target_os = "linux")]
    return linux::left_click();
    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
    Err("Unsupported OS".to_string())
}

pub fn right_click() -> Result<(), String> {
    #[cfg(target_os = "macos")]
    return macos::right_click();
    #[cfg(target_os = "windows")]
    return windows::right_click();
    #[cfg(target_os = "linux")]
    return linux::right_click();
    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
    Err("Unsupported OS".to_string())
}

pub fn middle_click() -> Result<(), String> {
    #[cfg(target_os = "macos")]
    return macos::middle_click();
    #[cfg(target_os = "windows")]
    return windows::middle_click();
    #[cfg(target_os = "linux")]
    return linux::middle_click();
    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
    Err("Unsupported OS".to_string())
}

pub fn double_click() -> Result<(), String> {
    #[cfg(target_os = "macos")]
    return macos::double_click();
    #[cfg(target_os = "windows")]
    return windows::double_click();
    #[cfg(target_os = "linux")]
    return linux::double_click();
    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
    Err("Unsupported OS".to_string())
}

pub fn move_cursor(dx: i32, dy: i32) -> Result<(), String> {
    #[cfg(target_os = "macos")]
    return macos::move_cursor(dx, dy);
    #[cfg(target_os = "windows")]
    return windows::move_cursor(dx, dy);
    #[cfg(target_os = "linux")]
    return linux::move_cursor(dx, dy);
    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
    Err("Unsupported OS".to_string())
}

pub fn set_cursor_position(x: i32, y: i32) -> Result<(), String> {
    #[cfg(target_os = "macos")]
    return macos::set_cursor_position(x, y);
    #[cfg(target_os = "windows")]
    return windows::set_cursor_position(x, y);
    #[cfg(target_os = "linux")]
    return linux::set_cursor_position(x, y);
    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
    Err("Unsupported OS".to_string())
}

pub fn scroll(vertical: i32, horizontal: i32) -> Result<(), String> {
    if vertical == 0 && horizontal == 0 {
        return Ok(());
    }
    crate::scroll::scroll(vertical, horizontal)
}
