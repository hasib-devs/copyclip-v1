/// Action system - executes gamepad actions on the system
/// Organized into specific action categories for different system interactions

pub mod app;
pub mod executor;
pub mod keyboard;
pub mod mouse;
pub mod system;

// Re-export executor function for convenience
pub use executor::{execute_action, execute_action_safe};
