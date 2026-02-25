/// HOTKEY Mode Bindings (Phase 4 Stub)
/// Key combination & leader key mode for efficient shortcuts
///
/// Activated by: Hold LB + Y
/// Ideal for: Vim-style mnemonics, complex shortcuts, application-specific commands
///
/// Pattern examples (to be implemented in Phase 4):
/// - Leader: Hold Y, then tap (Y+A=Cmd+A, Y+C=Cmd+C, etc.)
/// - Chords: LB+X=Cmd+X, RB+S=Cmd+S, etc.
/// - Sequences: Complex multi-button patterns
///
/// This is a stub. Full implementation in Phase 4.

use crate::types::*;
use crate::gamepad::GamepadButtonIndex;

/// Build HOTKEY mode keybinding registry (stub for Phase 4)
pub fn build_hotkey_mode_bindings() -> KeyBindingRegistry {
    let mut registry = KeyBindingRegistry::new();

    // ============ ESCAPE FROM HOTKEY MODE ============

    // LB + Y Hold - Return to NORMAL mode
    registry.add_binding(
        KeyBinding::new(
            InputPattern::Chord {
                buttons: vec![
                    GamepadButton { index: GamepadButtonIndex::LB },
                    GamepadButton { index: GamepadButtonIndex::North },
                ],
            },
            Action::SwitchMode {
                mode: GamepadMode::Normal,
            },
            GamepadMode::Hotkey,
        )
        .with_description("Hold LB+Y to exit HOTKEY mode".to_string())
        .with_priority(75),
    );

    // B Button - Escape (always available)
    registry.add_binding(
        KeyBinding::new(
            InputPattern::SingleButton {
                button: GamepadButton { index: GamepadButtonIndex::East },
                input_type: InputType::Tap,
            },
            Action::KeyPress {
                key: "Escape".to_string(),
            },
            GamepadMode::Hotkey,
        )
        .with_description("Escape / Cancel".to_string())
        .with_priority(50),
    );

    // ============ PLACEHOLDER BINDINGS ============
    // These will be replaced with actual leader key and chord patterns in Phase 4

    // A Button - Placeholder (Select All in Phase 4: Leader+A = Cmd+A)
    registry.add_binding(
        KeyBinding::new(
            InputPattern::SingleButton {
                button: GamepadButton { index: GamepadButtonIndex::South },
                input_type: InputType::Tap,
            },
            Action::NoOp,
            GamepadMode::Hotkey,
        )
        .with_description("[Phase 4] Leader+A for Select All".to_string())
        .with_priority(30),
    );

    // X Button - Placeholder (Cut in Phase 4: LB+X = Cmd+X)
    registry.add_binding(
        KeyBinding::new(
            InputPattern::SingleButton {
                button: GamepadButton { index: GamepadButtonIndex::West },
                input_type: InputType::Tap,
            },
            Action::NoOp,
            GamepadMode::Hotkey,
        )
        .with_description("[Phase 4] LB+X for Cut".to_string())
        .with_priority(30),
    );

    registry
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hotkey_mode_registry_has_exit() {
        let registry = build_hotkey_mode_bindings();
        assert!(!registry.is_empty());

        // Verify escape binding exists
        let b_button = GamepadButton {
            index: GamepadButtonIndex::East,
        };
        let bindings = registry.bindings_for_button(b_button);
        assert!(!bindings.is_empty());
    }
}
