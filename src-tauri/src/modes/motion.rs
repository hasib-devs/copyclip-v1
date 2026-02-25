/// MOTION Mode Bindings
/// Precision cursor control mode with sensitivity adjustments
///
/// Activated by: Hold RB + Y
/// Ideal for: Drawing, UI interaction, detailed pointing
///
/// Key patterns:
/// - Left Stick: Fine cursor movement (slower, more precise)
/// - Right Stick: Cursor adjustment or alternative control
/// - Face buttons: Click actions with precision variants
/// - Triggers: Drag operations, slow mode modifier
/// - Modifiers: Extend selection, alternative click types

use crate::types::*;
use crate::gamepad::GamepadButtonIndex;

/// Build MOTION mode keybinding registry
pub fn build_motion_mode_bindings() -> KeyBindingRegistry {
    let mut registry = KeyBindingRegistry::new();

    // ============ FACE BUTTONS (MOTION MODE) ============

    // A - Confirm / Left Click
    registry.add_binding(
        KeyBinding::new(
            InputPattern::SingleButton {
                button: GamepadButton { index: GamepadButtonIndex::South },
                input_type: InputType::Tap,
            },
            Action::MouseClick,
            GamepadMode::Motion,
        )
        .with_description("Confirm / Left click".to_string())
        .with_priority(50),
    );

    // B - Cancel / Escape
    registry.add_binding(
        KeyBinding::new(
            InputPattern::SingleButton {
                button: GamepadButton { index: GamepadButtonIndex::East },
                input_type: InputType::Tap,
            },
            Action::KeyPress {
                key: "Escape".to_string(),
            },
            GamepadMode::Motion,
        )
        .with_description("Cancel / Escape".to_string())
        .with_priority(50),
    );

    // X - Right Click
    registry.add_binding(
        KeyBinding::new(
            InputPattern::SingleButton {
                button: GamepadButton { index: GamepadButtonIndex::West },
                input_type: InputType::Tap,
            },
            Action::MouseRightClick,
            GamepadMode::Motion,
        )
        .with_description("Right click / Context menu".to_string())
        .with_priority(50),
    );

    // Y - Double Click
    registry.add_binding(
        KeyBinding::new(
            InputPattern::SingleButton {
                button: GamepadButton { index: GamepadButtonIndex::North },
                input_type: InputType::Tap,
            },
            Action::MouseDoubleClick,
            GamepadMode::Motion,
        )
        .with_description("Double click".to_string())
        .with_priority(50),
    );

    // ============ D-PAD (MOTION MODE) ============

    // D-Pad Up - Scroll up (precision)
    registry.add_binding(
        KeyBinding::new(
            InputPattern::SingleButton {
                button: GamepadButton { index: GamepadButtonIndex::DPadUp },
                input_type: InputType::Tap,
            },
            Action::MouseScroll {
                vertical: -5,
                horizontal: 0,
            },
            GamepadMode::Motion,
        )
        .with_description("Scroll up (precision)".to_string())
        .with_priority(50),
    );

    // D-Pad Down - Scroll down (precision)
    registry.add_binding(
        KeyBinding::new(
            InputPattern::SingleButton {
                button: GamepadButton { index: GamepadButtonIndex::DPadDown },
                input_type: InputType::Tap,
            },
            Action::MouseScroll {
                vertical: 5,
                horizontal: 0,
            },
            GamepadMode::Motion,
        )
        .with_description("Scroll down (precision)".to_string())
        .with_priority(50),
    );

    // D-Pad Left - Scroll left (precision)
    registry.add_binding(
        KeyBinding::new(
            InputPattern::SingleButton {
                button: GamepadButton { index: GamepadButtonIndex::DPadLeft },
                input_type: InputType::Tap,
            },
            Action::MouseScroll {
                vertical: 0,
                horizontal: -5,
            },
            GamepadMode::Motion,
        )
        .with_description("Scroll left (precision)".to_string())
        .with_priority(50),
    );

    // D-Pad Right - Scroll right (precision)
    registry.add_binding(
        KeyBinding::new(
            InputPattern::SingleButton {
                button: GamepadButton { index: GamepadButtonIndex::DPadRight },
                input_type: InputType::Tap,
            },
            Action::MouseScroll {
                vertical: 0,
                horizontal: 5,
            },
            GamepadMode::Motion,
        )
        .with_description("Scroll right (precision)".to_string())
        .with_priority(50),
    );

    // ============ SHOULDER BUTTONS (MOTION MODE) ============

    // LB - Modifier (Alt for selections)
    // This is handled by the input system as a modifier
    // No direct binding needed here

    // RB - Modifier (Ctrl for extended selection)
    // This is handled by the input system as a modifier
    // No direct binding needed here

    // ============ TRIGGER BUTTONS (MOTION MODE) ============

    // LT Hold - Drag mode (click + hold for drag)
    registry.add_binding(
        KeyBinding::new(
            InputPattern::SingleButton {
                button: GamepadButton { index: GamepadButtonIndex::LT },
                input_type: InputType::Hold,
            },
            Action::KeyPress {
                key: "MouseDrag".to_string(), // Special action for drag mode
            },
            GamepadMode::Motion,
        )
        .with_description("Drag mode (hold)".to_string())
        .with_priority(60),
    );

    // RT - Slow mode (0.5x sensitivity)
    registry.add_binding(
        KeyBinding::new(
            InputPattern::SingleButton {
                button: GamepadButton { index: GamepadButtonIndex::RT },
                input_type: InputType::Hold,
            },
            Action::KeyPress {
                key: "SlowMode".to_string(), // Special action for slow mode
            },
            GamepadMode::Motion,
        )
        .with_description("Slow mode (precise movement)".to_string())
        .with_priority(60),
    );

    // ============ SPECIAL BUTTONS (MOTION MODE) ============

    // Select Button - Cycle sensitivity (0.5x → 1.0x → 2.0x → 3.0x)
    registry.add_binding(
        KeyBinding::new(
            InputPattern::SingleButton {
                button: GamepadButton { index: GamepadButtonIndex::Select },
                input_type: InputType::Tap,
            },
            Action::KeyPress {
                key: "CycleSensitivity".to_string(), // Special action
            },
            GamepadMode::Motion,
        )
        .with_description("Cycle sensitivity (0.5x → 1.0x → 2.0x → 3.0x)".to_string())
        .with_priority(50),
    );

    // RB + Y Hold - Return to NORMAL mode (same toggle button)
    registry.add_binding(
        KeyBinding::new(
            InputPattern::Chord {
                buttons: vec![
                    GamepadButton { index: GamepadButtonIndex::RB },
                    GamepadButton { index: GamepadButtonIndex::North },
                ],
            },
            Action::SwitchMode {
                mode: GamepadMode::Normal,
            },
            GamepadMode::Motion,
        )
        .with_description("Hold RB+Y to exit MOTION mode".to_string())
        .with_priority(70),
    );

    registry
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_motion_mode_registry() {
        let registry = build_motion_mode_bindings();
        assert!(!registry.is_empty());

        // Verify motion mode has click bindings
        let a_button = GamepadButton {
            index: GamepadButtonIndex::South,
        };
        let bindings = registry.bindings_for_button(a_button);
        assert!(!bindings.is_empty());
    }
}
