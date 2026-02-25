/// Action executor - centralized dispatcher for all gamepad actions
/// Translates Action enums into actual system/application interactions

use crate::types::Action;
use log::{debug, error, info};

/// Execute a single action
pub async fn execute_action(action: &Action) -> Result<(), String> {
    debug!("[Executor] Execute action: {}", action);

    match action {
        // ============ SYSTEM ACTIONS ============
        Action::VolumeUp { amount } => {
            super::system::set_volume(*amount)
        }
        Action::VolumeDown { amount } => {
            super::system::set_volume(-amount)
        }
        Action::BrightnessUp { amount } => {
            super::system::set_brightness(*amount)
        }
        Action::BrightnessDown { amount } => {
            super::system::set_brightness(-amount)
        }
        Action::Screenshot => super::system::take_screenshot(),
        Action::ScreenRecording { start: _ } => {
            info!("[Action] Screen recording (not fully implemented)");
            Ok(())
        }

        // ============ APP ACTIONS ============
        Action::AppLauncher => super::app::open_app_launcher(),
        Action::AppPrevious => super::app::switch_to_previous_app(),
        Action::AppNext => super::app::switch_to_next_app(),
        Action::AppSwitcher => super::app::show_app_switcher(),

        // ============ WINDOW ACTIONS ============
        Action::WindowSnap { position: _ } => {
            info!("[Action] Window snap (not fully implemented)");
            Ok(())
        }
        Action::WindowCycle => {
            info!("[Action] Window cycle (not fully implemented)");
            Ok(())
        }

        // ============ MOUSE ACTIONS ============
        Action::MouseMove { dx, dy } => super::mouse::move_cursor(*dx, *dy),
        Action::MousePosition { x, y } => super::mouse::set_cursor_position(*x, *y),
        Action::MouseClick => super::mouse::left_click(),
        Action::MouseRightClick => super::mouse::right_click(),
        Action::MouseMiddleClick => super::mouse::middle_click(),
        Action::MouseDoubleClick => super::mouse::double_click(),
        Action::MouseScroll { vertical, horizontal } => {
            super::mouse::scroll(*vertical, *horizontal)
        }

        // ============ KEYBOARD ACTIONS ============
        Action::KeyPress { key } => super::keyboard::press_key(key),
        Action::KeyCombo { keys } => {
            let key_refs: Vec<&str> = keys.iter().map(|k| k.as_str()).collect();
            super::keyboard::press_key_combination(&key_refs)
        }
        Action::TextInput { text } => super::keyboard::type_text(text),

        // ============ MEDIA ACTIONS ============
        Action::MediaPlayPause => super::system::play_pause_media(),
        Action::MediaNext => {
            // Use standard media key
            super::keyboard::press_key("MediaNext").ok();
            Ok(())
        }
        Action::MediaPrevious => {
            // Use standard media key
            super::keyboard::press_key("MediaPrevious").ok();
            Ok(())
        }
        Action::MediaStop => {
            // Use standard media key
            super::keyboard::press_key("MediaStop").ok();
            Ok(())
        }

        // ============ BROWSER ACTIONS ============
        Action::BrowserBack => super::keyboard::press_key_combination(&["Alt", "Left"]),
        Action::BrowserForward => super::keyboard::press_key_combination(&["Alt", "Right"]),
        Action::BrowserReload => super::keyboard::press_key("F5"),
        Action::BrowserNewTab => super::keyboard::press_key_combination(&["Control", "t"]),
        Action::BrowserCloseTab => super::keyboard::press_key_combination(&["Control", "w"]),
        Action::BrowserNextTab => super::keyboard::press_key_combination(&["Control", "Tab"]),
        Action::BrowserPrevTab => super::keyboard::press_key_combination(&["Control", "Shift", "Tab"]),
        Action::BrowserFind => super::keyboard::press_key_combination(&["Control", "f"]),

        // ============ MODE ACTIONS ============
        Action::SwitchMode { mode: _ } => {
            // Mode switching is handled by mode manager, not executor
            // This just logs it
            info!("[Action] Mode switch handled by manager");
            Ok(())
        }

        // ============ NO-OP ============
        Action::NoOp => {
            debug!("[Action] NoOp");
            Ok(())
        }
    }
}

/// Execute action and handle errors
pub async fn execute_action_safe(action: &Action) -> Option<String> {
    match execute_action(action).await {
        Ok(_) => {
            info!("[Executor] Action successful: {}", action);
            None
        }
        Err(e) => {
            error!("[Executor] Action failed: {} - {}", action, e);
            Some(e)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_noop_action() {
        let action = Action::NoOp;
        assert!(execute_action(&action).await.is_ok());
    }

    #[tokio::test]
    async fn test_action_display() {
        let action = Action::VolumeUp { amount: 10 };
        assert_eq!(
            action.to_string(),
            "Volume Up (10%)"
        );
    }
}
