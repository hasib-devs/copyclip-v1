# Phase 2 Implementation Plan: Modal System & NORMAL Mode

**Status:** Planning & Initial Implementation  
**Target Duration:** 2-3 weeks  
**Priority:** Core infrastructure for all future features

---

## Overview

Phase 2 transforms the basic gamepad input system into a sophisticated modal control system inspired by Vim, enabling context-aware features and efficient PC control.

## Implementation Steps (Sequential)

### Step 1: Type System Refactor ✓ Planning
**Files:**
- `src-tauri/src/types/` (new directory)
  - `mode.rs` - GamepadMode enum
  - `action.rs` - Action enum & system
  - `binding.rs` - InputPattern & KeyBinding
  - `mod.rs` - Module exports

**What this enables:** Type-safe mode system, action resolution, pattern matching

---

### Step 2: Mode Manager Module ✓ Planning
**Files:**
- `src-tauri/src/modes/` (new directory)
  - `manager.rs` - GamepadModeManager (state + transitions)
  - `normal.rs` - NORMAL mode implementation
  - `motion.rs` - MOTION mode implementation
  - `hotkey.rs` - HOTKEY mode stub (Phase 4)
  - `mod.rs` - Module exports

**What this enables:** Mode switching, mode-specific behaviors, state tracking

---

### Step 3: Action System Module ✓ Planning
**Files:**
- `src-tauri/src/actions/` (new directory)
  - `system.rs` - System-level actions (volume, brightness, etc.)
  - `app.rs` - App switcher, launcher, window management
  - `mouse.rs` - Mouse movement, clicks, scrolling
  - `keyboard.rs` - Keyboard input, combos
  - `executor.rs` - Action execution engine
  - `mod.rs` - Module exports

**What this enables:** Decoupled action execution, reusable action definitions

---

### Step 4: Keybinding Resolver ✓ Planning
**Files:**
- `src-tauri/src/bindings/` (new directory)
  - `resolver.rs` - Pattern matching & binding resolution
  - `registry.rs` - Built-in binding registry
  - `mod.rs` - Module exports

**What this enables:** Efficient button→action mapping, priority resolution

---

### Step 5: Integration & Polish ✓ Planning
**Files:**
- `src-tauri/src/gamepad_manager.rs` - Update to use new modules
- `src/components/ModeIndicator.tsx` - Show current mode
- `src/components/GamepadDebug.tsx` - Show button presses, mode, assigned action

**What this enables:** Working Phase 2 system, visibility into what's happening

---

## File Structure After Phase 2

```
src-tauri/src/
├── main.rs
├── lib.rs
├── db.rs
├── gamepad.rs
├── gamepad_manager.rs (UPDATED)
├── scroll.rs
├── commands.rs
├── models/
│   └── mod.rs
│
├── types/                    (NEW - Phase 2)
│   ├── mod.rs
│   ├── mode.rs
│   ├── action.rs
│   └── binding.rs
│
├── modes/                    (NEW - Phase 2)
│   ├── mod.rs
│   ├── manager.rs
│   ├── normal.rs
│   ├── motion.rs
│   └── hotkey.rs
│
├── actions/                  (NEW - Phase 2)
│   ├── mod.rs
│   ├── system.rs
│   ├── app.rs
│   ├── mouse.rs
│   ├── keyboard.rs
│   └── executor.rs
│
└── bindings/                (NEW - Phase 2)
    ├── mod.rs
    ├── resolver.rs
    └── registry.rs

src/
├── components/
│   ├── GamepadConfig.tsx
│   ├── ModeIndicator.tsx        (NEW - Phase 2)
│   └── GamepadDebug.tsx         (NEW - Phase 2)
├── types/
│   └── gamepad.types.ts         (UPDATED)
└── hooks/
    └── useGamepadMode.ts        (NEW - Phase 2)
```

---

## Type Definitions Detail

### Core Types

```rust
// types/mode.rs
pub enum GamepadMode {
    Normal,      // Navigation, app control
    Motion,      // Cursor precision
    Hotkey,      // Key combinations (Phase 4)
}

// types/action.rs
pub enum Action {
    // System
    VolumeUp(i32),
    VolumeDown(i32),
    BrightnessUp(i32),
    BrightnessDown(i32),
    Screenshot,
    
    // App
    AppLauncher,
    AppPrevious,
    AppNext,
    
    // Mouse
    MouseMove(i32, i32),
    MouseClick,
    MouseRightClick,
    
    // Keyboard
    KeyPress(String),
    KeyCombo(Vec<String>),
    
    // Mode
    SwitchMode(GamepadMode),
}

// types/binding.rs
pub struct KeyBinding {
    pub button: GamepadButton,
    pub modifier: InputModifier,
    pub input_type: InputType,
    pub action: Action,
}

pub enum InputType {
    Tap,
    Hold,
    DoubleTap,
}

pub enum InputModifier {
    None,
    Alt,       // LB
    Ctrl,      // RB
    Shift,     // LB + RB
}
```

---

## Implementation Checklist

### Step 1: Type System
- [ ] Create `src-tauri/src/types/` directory
- [ ] Implement `mode.rs` with GamepadMode enum
- [ ] Implement `action.rs` with Action enum
- [ ] Implement `binding.rs` with KeyBinding & InputPattern
- [ ] Create `types/mod.rs` with exports
- [ ] Update `lib.rs` to include types module

### Step 2: Mode Manager
- [ ] Create `src-tauri/src/modes/` directory
- [ ] Implement `manager.rs` - GamepadModeManager struct
- [ ] Implement `normal.rs` - NORMAL mode binding definitions
- [ ] Implement `motion.rs` - MOTION mode definitions
- [ ] Implement `hotkey.rs` - HOTKEY mode stub
- [ ] Create `modes/mod.rs` with exports
- [ ] Update `lib.rs` to include modes module

### Step 3: Action System
- [ ] Create `src-tauri/src/actions/` directory
- [ ] Implement `system.rs` - Volume, brightness, media
- [ ] Implement `app.rs` - App switcher, window management
- [ ] Implement `mouse.rs` - Mouse control actions
- [ ] Implement `keyboard.rs` - Keyboard actions
- [ ] Implement `executor.rs` - Action execution engine
- [ ] Create `actions/mod.rs` with exports
- [ ] Update `lib.rs` to include actions module

### Step 4: Keybinding Resolver
- [ ] Create `src-tauri/src/bindings/` directory
- [ ] Implement `resolver.rs` - Button to action resolution
- [ ] Implement `registry.rs` - Built-in bindings
- [ ] Create `bindings/mod.rs` with exports
- [ ] Update `lib.rs` to include bindings module

### Step 5: Integration
- [ ] Update `gamepad_manager.rs` to use new modules
- [ ] Create `ModeIndicator.tsx` component
- [ ] Create `GamepadDebug.tsx` component
- [ ] Create `useGamepadMode.ts` hook
- [ ] Add mode indicator to main UI
- [ ] Test all features end-to-end

### Step 6: Testing & Validation
- [ ] Compile and test (ensure zero errors)
- [ ] Verify mode switching
- [ ] Test all NORMAL mode actions
- [ ] Test MOTION mode controls
- [ ] Test cross-platform functionality

---

## Success Criteria

- ✅ Code compiles with zero errors
- ✅ Mode switching works reliably (RB+Y, LB+Y)
- ✅ All NORMAL mode features functional
- ✅ MOTION mode provides precision control
- ✅ UI shows current mode
- ✅ Debug panel shows button→action mapping
- ✅ < 20ms latency maintained
- ✅ Professional modular code structure
- ✅ Complete inline documentation

---

## Notes

- **Testing approach:** Implement feature > compile > test on macOS > validate
- **Dependencies:** All existing (enigo, gilrs, serde)
- **Backwards compatibility:** Full compatibility with Phase 1 features
- **Next phase:** After Phase 2 complete, move to Phase 2.5 (app detection)
