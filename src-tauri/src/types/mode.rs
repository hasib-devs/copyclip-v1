/// Gamepad modal system - inspired by Vim modal editing
/// Different modes provide different button behaviors for efficient control
use serde::{Deserialize, Serialize};

/// Core gamepad modes
/// Each mode changes how buttons are interpreted
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GamepadMode {
    /// Default mode: Navigation, app control, system features
    /// Features: App launcher, Alt+Tab, volume, brightness, screenshots
    Normal,

    /// Precision cursor control mode
    /// Features: Mouse movement, sensitivity adjustments, fine-tuning
    /// Triggered: Hold RB + Y
    Motion,

    /// Key combination & leader key mode
    /// Features: Keyboard shortcuts, Vim-style patterns, chord combinations
    /// Triggered: Hold LB + Y
    Hotkey,
}

impl GamepadMode {
    /// Get human-readable name for the mode
    pub fn name(&self) -> &'static str {
        match self {
            GamepadMode::Normal => "NORMAL",
            GamepadMode::Motion => "MOTION",
            GamepadMode::Hotkey => "HOTKEY",
        }
    }

    /// Get color code for UI (RGB-ish semantic naming)
    pub fn color_code(&self) -> &'static str {
        match self {
            GamepadMode::Normal => "blue",
            GamepadMode::Motion => "green",
            GamepadMode::Hotkey => "purple",
        }
    }

    /// Get description of the mode
    pub fn description(&self) -> &'static str {
        match self {
            GamepadMode::Normal => "Navigate apps, control system",
            GamepadMode::Motion => "Precision cursor movement",
            GamepadMode::Hotkey => "Keyboard shortcuts & combos",
        }
    }

    /// Get the toggle key combination for this mode
    pub fn toggle_combination(&self) -> &'static str {
        match self {
            GamepadMode::Normal => "[from any other mode]",
            GamepadMode::Motion => "Hold RB + Y",
            GamepadMode::Hotkey => "Hold LB + Y",
        }
    }
}

/// Input modifier state - represents additional context for button presses
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InputModifier {
    /// No modifier
    None,

    /// Alt key (mapped to LB)
    Alt,

    /// Ctrl key (mapped to RB)
    Ctrl,

    /// Shift key (mapped to LB + RB)
    Shift,

    /// Alt + Ctrl
    AltCtrl,

    /// Alt + Shift
    AltShift,

    /// Ctrl + Shift
    CtrlShift,

    /// Alt + Ctrl + Shift
    AltCtrlShift,
}

impl InputModifier {
    /// Convert to keyboard modifier names
    pub fn keys(&self) -> Vec<&'static str> {
        match self {
            InputModifier::None => vec![],
            InputModifier::Alt => vec!["Alt"],
            InputModifier::Ctrl => vec!["Control"],
            InputModifier::Shift => vec!["Shift"],
            InputModifier::AltCtrl => vec!["Alt", "Control"],
            InputModifier::AltShift => vec!["Alt", "Shift"],
            InputModifier::CtrlShift => vec!["Control", "Shift"],
            InputModifier::AltCtrlShift => vec!["Alt", "Control", "Shift"],
        }
    }

    /// Check if modifier includes Alt
    pub fn has_alt(&self) -> bool {
        matches!(
            self,
            InputModifier::Alt
                | InputModifier::AltCtrl
                | InputModifier::AltShift
                | InputModifier::AltCtrlShift
        )
    }

    /// Check if modifier includes Ctrl
    pub fn has_ctrl(&self) -> bool {
        matches!(
            self,
            InputModifier::Ctrl
                | InputModifier::AltCtrl
                | InputModifier::CtrlShift
                | InputModifier::AltCtrlShift
        )
    }

    /// Check if modifier includes Shift
    pub fn has_shift(&self) -> bool {
        matches!(
            self,
            InputModifier::Shift
                | InputModifier::AltShift
                | InputModifier::CtrlShift
                | InputModifier::AltCtrlShift
        )
    }
}

/// Type of input interaction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InputType {
    /// Short press (0-150ms)
    Tap,

    /// Button held down (>200ms)
    Hold,

    /// Two quick presses (within 300ms)
    DoubleTap,

    /// Multiple simultaneous buttons pressed
    Chord,

    /// Long hold for menu or alternative action (>500ms)
    LongHold,
}

impl InputType {
    /// Get human-readable name
    pub fn name(&self) -> &'static str {
        match self {
            InputType::Tap => "tap",
            InputType::Hold => "hold",
            InputType::DoubleTap => "double-tap",
            InputType::Chord => "chord",
            InputType::LongHold => "long-hold",
        }
    }
}

/// Mode state tracking for frontend display
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModeState {
    /// Current active mode
    pub current: GamepadMode,

    /// Previous mode (for fallback on release)
    pub previous: GamepadMode,

    /// Whether transitioning between modes
    pub transitioning: bool,

    /// Time mode was activated
    pub activated_at: u64,
}

impl Default for ModeState {
    fn default() -> Self {
        Self {
            current: GamepadMode::Normal,
            previous: GamepadMode::Normal,
            transitioning: false,
            activated_at: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mode_names() {
        assert_eq!(GamepadMode::Normal.name(), "NORMAL");
        assert_eq!(GamepadMode::Motion.name(), "MOTION");
        assert_eq!(GamepadMode::Hotkey.name(), "HOTKEY");
    }

    #[test]
    fn test_modifier_keys() {
        assert_eq!(InputModifier::None.keys(), vec![]);
        assert_eq!(InputModifier::Alt.keys(), vec!["Alt"]);
        assert_eq!(InputModifier::Ctrl.keys(), vec!["Control"]);
        assert!(InputModifier::AltCtrl.has_alt());
        assert!(InputModifier::AltCtrl.has_ctrl());
        assert!(!InputModifier::AltCtrl.has_shift());
    }
}
