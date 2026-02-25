/// Keybinding definitions - maps gamepad buttons to actions
/// Supports multiple input pattern types: tap, hold, double-tap, etc.

use crate::gamepad::GamepadButtonIndex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A single gamepad button as identified by gilrs
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct GamepadButton {
    pub index: GamepadButtonIndex,
}

/// Input pattern for matching button presses
/// Supports single button, modifier combinations, and sequences
#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum InputPattern {
    /// Single button press
    SingleButton {
        button: GamepadButton,
        input_type: crate::types::InputType,
    },

    /// Button with modifier key
    ModifiedButton {
        button: GamepadButton,
        modifier: crate::types::InputModifier,
        input_type: crate::types::InputType,
    },

    /// Multiple buttons pressed together (chord)
    Chord {
        buttons: Vec<GamepadButton>,
    },

    /// Button sequence (first button sets context, second triggers)
    Sequence {
        first: GamepadButton,
        second: GamepadButton,
        timeout_ms: u32,
    },
}

impl InputPattern {
    /// Check if pattern contains a specific button
    pub fn contains_button(&self, button: GamepadButton) -> bool {
        match self {
            InputPattern::SingleButton { button: b, .. } => b == &button,
            InputPattern::ModifiedButton { button: b, .. } => b == &button,
            InputPattern::Chord { buttons } => buttons.contains(&button),
            InputPattern::Sequence { first, second, .. } => first == &button || second == &button,
        }
    }

    /// Get all buttons in this pattern
    pub fn buttons(&self) -> Vec<GamepadButton> {
        match self {
            InputPattern::SingleButton { button, .. } => vec![*button],
            InputPattern::ModifiedButton { button, .. } => vec![*button],
            InputPattern::Chord { buttons } => buttons.clone(),
            InputPattern::Sequence { first, second, .. } => vec![*first, *second],
        }
    }
}

/// A binding of input pattern to action
/// Associates a gamepad button/pattern with an action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyBinding {
    /// Pattern that triggers this binding
    pub pattern: InputPattern,

    /// Action to execute
    pub action: crate::types::Action,

    /// Priority (higher = takes precedence in conflicts)
    pub priority: u8,

    /// Whether binding is active
    pub enabled: bool,

    /// Optional description for UI display
    pub description: Option<String>,

    /// Mode this binding is active in
    pub mode: crate::types::GamepadMode,
}

impl KeyBinding {
    /// Create a new binding with default priority
    pub fn new(
        pattern: InputPattern,
        action: crate::types::Action,
        mode: crate::types::GamepadMode,
    ) -> Self {
        Self {
            pattern,
            action,
            priority: 50,
            enabled: true,
            description: None,
            mode,
        }
    }

    /// Set priority for this binding
    pub fn with_priority(mut self, priority: u8) -> Self {
        self.priority = priority;
        self
    }

    /// Set description for this binding
    pub fn with_description(mut self, desc: String) -> Self {
        self.description = Some(desc);
        self
    }

    /// Set enabled/disabled state
    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
}

/// Registry of all keybindings for a mode
#[derive(Debug, Clone, Default)]
pub struct KeyBindingRegistry {
    /// Map from pattern hash to binding
    /// We use HashMap for O(1) lookup performance
    bindings: HashMap<String, KeyBinding>,
}

impl KeyBindingRegistry {
    /// Create new empty registry
    pub fn new() -> Self {
        Self {
            bindings: HashMap::new(),
        }
    }

    /// Add binding to registry
    pub fn add_binding(&mut self, binding: KeyBinding) {
        let key = Self::pattern_key(&binding.pattern);
        self.bindings.insert(key, binding);
    }

    /// Remove binding by pattern
    pub fn remove_binding(&mut self, pattern: &InputPattern) -> Option<KeyBinding> {
        let key = Self::pattern_key(pattern);
        self.bindings.remove(&key)
    }

    /// Get binding for pattern
    pub fn get_binding(&self, pattern: &InputPattern) -> Option<&KeyBinding> {
        let key = Self::pattern_key(pattern);
        self.bindings.get(&key)
    }

    /// Get all bindings
    pub fn all_bindings(&self) -> Vec<&KeyBinding> {
        let mut bindings: Vec<_> = self.bindings.values().collect();
        // Sort by priority (descending)
        bindings.sort_by(|a, b| b.priority.cmp(&a.priority));
        bindings
    }

    /// Get bindings for specific button
    pub fn bindings_for_button(&self, button: GamepadButton) -> Vec<&KeyBinding> {
        let mut matching: Vec<_> = self
            .bindings
            .values()
            .filter(|b| b.pattern.contains_button(button) && b.enabled)
            .collect();
        // Sort by priority
        matching.sort_by(|a, b| b.priority.cmp(&a.priority));
        matching
    }

    /// Clear all bindings
    pub fn clear(&mut self) {
        self.bindings.clear();
    }

    /// Get count of bindings
    pub fn len(&self) -> usize {
        self.bindings.len()
    }

    /// Check if registry is empty
    pub fn is_empty(&self) -> bool {
        self.bindings.is_empty()
    }

    /// Generate unique key for pattern (for HashMap)
    fn pattern_key(pattern: &InputPattern) -> String {
        format!("{pattern:?}")
    }
}

/// Configuration for button timing thresholds
#[derive(Debug, Clone, Copy)]
pub struct InputTiming {
    /// Milliseconds to distinguish tap from hold
    pub tap_threshold_ms: u32,

    /// Milliseconds for double-tap window
    pub double_tap_window_ms: u32,

    /// Milliseconds before long-hold triggers
    pub long_hold_threshold_ms: u32,

    /// Milliseconds for sequence timeout
    pub sequence_timeout_ms: u32,

    /// Milliseconds for chord detection window
    pub chord_window_ms: u32,
}

impl Default for InputTiming {
    fn default() -> Self {
        Self {
            tap_threshold_ms: 150,
            double_tap_window_ms: 300,
            long_hold_threshold_ms: 500,
            sequence_timeout_ms: 2000,
            chord_window_ms: 100,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Action, GamepadMode, InputModifier, InputType};

    #[test]
    fn test_input_pattern_contains_button() {
        let button = GamepadButton {
            index: GamepadButtonIndex::A,
        };

        let pattern = InputPattern::SingleButton {
            button,
            input_type: InputType::Tap,
        };

        assert!(pattern.contains_button(button));

        let other = GamepadButton {
            index: GamepadButtonIndex::B,
        };
        assert!(!pattern.contains_button(other));
    }

    #[test]
    fn test_keybinding_registry() {
        let mut registry = KeyBindingRegistry::new();
        assert!(registry.is_empty());

        let button = GamepadButton {
            index: GamepadButtonIndex::A,
        };
        let pattern = InputPattern::SingleButton {
            button,
            input_type: InputType::Tap,
        };

        let binding = KeyBinding::new(pattern.clone(), Action::MouseClick, GamepadMode::Normal);
        registry.add_binding(binding);

        assert_eq!(registry.len(), 1);
        assert!(registry.get_binding(&pattern).is_some());
    }
}
