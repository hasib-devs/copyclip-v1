/// Type system for gamepad control architecture
/// Organized into logical submodules for mode, action, and binding definitions

pub mod action;
pub mod binding;
pub mod mode;

// Re-export commonly used types for convenience
pub use action::{Action, WindowPosition};
pub use binding::{GamepadButton, InputPattern, KeyBinding, KeyBindingRegistry, InputTiming};
pub use mode::{GamepadMode, InputModifier, InputType, ModeState};
