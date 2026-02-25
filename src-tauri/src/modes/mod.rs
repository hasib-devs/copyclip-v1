/// Gamepad mode system
/// Organized into managers and per-mode binding definitions

pub mod hotkey;
pub mod manager;
pub mod motion;
pub mod normal;

// Re-export the manager for convenient access
pub use manager::GamepadModeManager;

/// Get keybinding registry for a specific mode
pub fn get_mode_bindings(mode: crate::types::GamepadMode) -> crate::types::KeyBindingRegistry {
    match mode {
        crate::types::GamepadMode::Normal => normal::build_normal_mode_bindings(),
        crate::types::GamepadMode::Motion => motion::build_motion_mode_bindings(),
        crate::types::GamepadMode::Hotkey => hotkey::build_hotkey_mode_bindings(),
    }
}
