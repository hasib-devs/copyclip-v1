/// Action system - defines all possible gamepad actions
/// Actions are the output of button presses, executed by the action executor

use serde::{Deserialize, Serialize};
use std::fmt;

/// All possible actions that can be triggered by gamepad input
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Action {
    // ============ SYSTEM ACTIONS ============

    /// Increase volume by percentage
    VolumeUp {
        amount: i32, // 0-100
    },

    /// Decrease volume by percentage
    VolumeDown {
        amount: i32, // 0-100
    },

    /// Increase screen brightness
    BrightnessUp {
        amount: i32, // 0-100
    },

    /// Decrease screen brightness
    BrightnessDown {
        amount: i32, // 0-100
    },

    /// Take screenshot
    Screenshot,

    /// Start/stop screen recording
    ScreenRecording {
        start: bool,
    },

    // ============ APP MANAGEMENT ============

    /// Open application launcher/spotlight
    AppLauncher,

    /// Switch to previous application (Alt+Tab back)
    AppPrevious,

    /// Switch to next application (Alt+Tab forward)
    AppNext,

    /// Show all open applications
    AppSwitcher,

    // ============ WINDOW MANAGEMENT ============

    /// Snap window to position
    WindowSnap {
        position: WindowPosition,
    },

    /// Cycle through open windows in current app
    WindowCycle,

    // ============ MOUSE ACTIONS ============

    /// Move cursor with relative displacement
    MouseMove {
        dx: i32,
        dy: i32,
    },

    /// Set cursor to absolute position
    MousePosition {
        x: i32,
        y: i32,
    },

    /// Left mouse click
    MouseClick,

    /// Right mouse click
    MouseRightClick,

    /// Middle mouse click
    MouseMiddleClick,

    /// Double click
    MouseDoubleClick,

    /// Scroll vertically and/or horizontally
    MouseScroll {
        vertical: i32,   // Positive = down
        horizontal: i32, // Positive = right
    },

    // ============ KEYBOARD ACTIONS ============

    /// Press a single key
    KeyPress {
        key: String, // "Return", "Escape", "Tab", etc.
    },

    /// Press multiple keys in combination
    KeyCombo {
        keys: Vec<String>,
    },

    /// Type text directly
    TextInput {
        text: String,
    },

    // ============ MEDIA CONTROLS ============

    /// Play/Pause media
    MediaPlayPause,

    /// Next track
    MediaNext,

    /// Previous track
    MediaPrevious,

    /// Stop media
    MediaStop,

    // ============ BROWSER ACTIONS ============

    /// Navigate back (browser back button)
    BrowserBack,

    /// Navigate forward (browser forward button)
    BrowserForward,

    /// Reload page
    BrowserReload,

    /// Open new tab
    BrowserNewTab,

    /// Close current tab
    BrowserCloseTab,

    /// Cycle to next tab
    BrowserNextTab,

    /// Cycle to previous tab
    BrowserPrevTab,

    /// Open browser find/search
    BrowserFind,

    // ============ MODE ACTIONS ============

    /// Switch to specified mode
    SwitchMode {
        mode: crate::types::GamepadMode,
    },

    /// No operation - for disabled/empty bindings
    NoOp,
}

/// Window snap positions
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum WindowPosition {
    Top,
    Bottom,
    Left,
    Right,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Center,
    Maximize,
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Action::VolumeUp { amount } => write!(f, "Volume Up ({amount}%)"),
            Action::VolumeDown { amount } => write!(f, "Volume Down ({amount}%)"),
            Action::BrightnessUp { amount } => write!(f, "Brightness Up ({amount}%)"),
            Action::BrightnessDown { amount } => write!(f, "Brightness Down ({amount}%)"),
            Action::Screenshot => write!(f, "Screenshot"),
            Action::ScreenRecording { start } => write!(
                f,
                "Screen Recording {}",
                if *start { "Start" } else { "Stop" }
            ),
            Action::AppLauncher => write!(f, "App Launcher"),
            Action::AppPrevious => write!(f, "Previous App"),
            Action::AppNext => write!(f, "Next App"),
            Action::AppSwitcher => write!(f, "App Switcher"),
            Action::WindowSnap { position } => write!(f, "Snap Window {position:?}"),
            Action::WindowCycle => write!(f, "Cycle Windows"),
            Action::MouseMove { dx, dy } => write!(f, "Move Mouse ({dx}, {dy})"),
            Action::MousePosition { x, y } => write!(f, "Set Cursor ({x}, {y})"),
            Action::MouseClick => write!(f, "Left Click"),
            Action::MouseRightClick => write!(f, "Right Click"),
            Action::MouseMiddleClick => write!(f, "Middle Click"),
            Action::MouseDoubleClick => write!(f, "Double Click"),
            Action::MouseScroll { vertical, horizontal } => {
                write!(f, "Scroll (V:{vertical}, H:{horizontal})")
            }
            Action::KeyPress { key } => write!(f, "Key: {key}"),
            Action::KeyCombo { keys } => write!(f, "Keys: {}", keys.join("+").to_uppercase()),
            Action::TextInput { text } => write!(f, "Type: {text}"),
            Action::MediaPlayPause => write!(f, "Play/Pause"),
            Action::MediaNext => write!(f, "Next Track"),
            Action::MediaPrevious => write!(f, "Previous Track"),
            Action::MediaStop => write!(f, "Stop"),
            Action::BrowserBack => write!(f, "Browser Back"),
            Action::BrowserForward => write!(f, "Browser Forward"),
            Action::BrowserReload => write!(f, "Reload Page"),
            Action::BrowserNewTab => write!(f, "New Tab"),
            Action::BrowserCloseTab => write!(f, "Close Tab"),
            Action::BrowserNextTab => write!(f, "Next Tab"),
            Action::BrowserPrevTab => write!(f, "Previous Tab"),
            Action::BrowserFind => write!(f, "Find"),
            Action::SwitchMode { mode } => write!(f, "Switch to {} Mode", mode.name()),
            Action::NoOp => write!(f, "[No Action]"),
        }
    }
}

impl Action {
    /// Get a description of the action
    pub fn description(&self) -> String {
        self.to_string()
    }

    /// Check if this action requires continuous execution (not just once)
    pub fn is_continuous(&self) -> bool {
        matches!(
            self,
            Action::MouseMove { .. } | Action::MouseScroll { .. }
        )
    }

    /// Check if this action is a mode switch
    pub fn is_mode_switch(&self) -> bool {
        matches!(self, Action::SwitchMode { .. })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_action_display() {
        let action = Action::VolumeUp { amount: 10 };
        assert_eq!(action.to_string(), "Volume Up (10%)");

        let action = Action::KeyCombo {
            keys: vec!["Control".to_string(), "C".to_string()],
        };
        assert_eq!(action.to_string(), "Keys: CONTROL+C");
    }

    #[test]
    fn test_continuous_actions() {
        assert!(Action::MouseMove { dx: 10, dy: 0 }.is_continuous());
        assert!(Action::MouseScroll {
            vertical: 5,
            horizontal: 0
        }
        .is_continuous());
        assert!(!Action::MouseClick.is_continuous());
    }
}
