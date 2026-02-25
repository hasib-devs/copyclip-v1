/// Mode manager - orchestrates mode transitions and state tracking
/// Responsible for switching between NORMAL, MOTION, and HOTKEY modes

use crate::types::{GamepadMode, ModeState};
use std::time::{SystemTime, UNIX_EPOCH};

/// Manages gamepad mode state and transitions
pub struct GamepadModeManager {
    /// Current mode state
    state: ModeState,

    /// Last mode switch timestamp (for preventing rapid switching)
    last_switch_time: u64,

    /// Minimum milliseconds between mode switches
    min_switch_interval: u32,
}

impl GamepadModeManager {
    /// Create new mode manager starting in NORMAL mode
    pub fn new() -> Self {
        Self {
            state: ModeState::default(),
            last_switch_time: 0,
            min_switch_interval: 50, // 50ms debounce
        }
    }

    /// Get current mode
    pub fn current_mode(&self) -> GamepadMode {
        self.state.current
    }

    /// Get previous mode (for fallback)
    pub fn previous_mode(&self) -> GamepadMode {
        self.state.previous
    }

    /// Check if currently in transition
    pub fn is_transitioning(&self) -> bool {
        self.state.transitioning
    }

    /// Switch to a new mode (with debouncing)
    pub fn switch_mode(&mut self, new_mode: GamepadMode) -> bool {
        let now = current_time_ms();

        // Debounce: prevent mode switches that are too close together
        if now - self.last_switch_time < self.min_switch_interval as u64 {
            return false;
        }

        // Only switch if different from current mode
        if new_mode == self.state.current {
            return false;
        }

        log::info!(
            "Mode switch: {:?} -> {:?}",
            self.state.current,
            new_mode
        );

        self.state.previous = self.state.current;
        self.state.current = new_mode;
        self.state.activated_at = now;
        self.state.transitioning = true;
        self.last_switch_time = now;

        true
    }

    /// Return to previous mode
    pub fn revert_mode(&mut self) -> bool {
        if self.state.previous != self.state.current {
            self.switch_mode(self.state.previous)
        } else {
            false
        }
    }

    /// Reset to NORMAL mode (hard reset)
    pub fn reset_to_normal(&mut self) {
        self.switch_mode(GamepadMode::Normal);
    }

    /// Get full mode state (for serialization to frontend)
    pub fn get_state(&self) -> ModeState {
        self.state.clone()
    }

    /// Check if mode has been active for specified duration
    pub fn mode_active_for(&self, ms: u64) -> bool {
        let now = current_time_ms();
        now - self.state.activated_at >= ms
    }

    /// Get time in milliseconds since mode activation
    pub fn time_in_mode(&self) -> u64 {
        current_time_ms() - self.state.activated_at
    }
}

impl Default for GamepadModeManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Get current time in milliseconds since UNIX epoch
fn current_time_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mode_switch() {
        let mut manager = GamepadModeManager::new();
        assert_eq!(manager.current_mode(), GamepadMode::Normal);

        let switched = manager.switch_mode(GamepadMode::Motion);
        assert!(switched);
        assert_eq!(manager.current_mode(), GamepadMode::Motion);
        assert_eq!(manager.previous_mode(), GamepadMode::Normal);
    }

    #[test]
    fn test_no_duplicate_switch() {
        let mut manager = GamepadModeManager::new();

        let switched = manager.switch_mode(GamepadMode::Normal);
        assert!(!switched); // Same mode, should not switch
    }

    #[test]
    fn test_mode_revert() {
        let mut manager = GamepadModeManager::new();
        manager.switch_mode(GamepadMode::Motion);
        manager.switch_mode(GamepadMode::Hotkey);

        let reverted = manager.revert_mode();
        assert!(reverted);
        assert_eq!(manager.current_mode(), GamepadMode::Motion);
    }

    #[test]
    fn test_reset_to_normal() {
        let mut manager = GamepadModeManager::new();
        manager.switch_mode(GamepadMode::Motion);
        manager.switch_mode(GamepadMode::Hotkey);

        manager.reset_to_normal();
        assert_eq!(manager.current_mode(), GamepadMode::Normal);
    }
}
