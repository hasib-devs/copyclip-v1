import { GamepadContextState, GamepadAction } from "@/types/gamepad.types";

export const initialGamepadState: GamepadContextState = {
  gamepads: [],
  activeGamepadIndex: 0,
  profiles: [],
  activeProfile: null,
  isListening: true,  // Gamepad listener starts on app startup
  isLoading: false,
  error: null,
};

export function gamepadReducer(
  state: GamepadContextState,
  action: GamepadAction,
): GamepadContextState {
  switch (action.type) {
    case "SET_GAMEPADS": {
      const gamepads = action.payload;
      return {
        ...state,
        gamepads,
        // Keep activeGamepadIndex valid if it's out of bounds
        activeGamepadIndex: Math.min(
          state.activeGamepadIndex,
          Math.max(0, gamepads.length - 1),
        ),
      };
    }

    case "SET_ACTIVE_GAMEPAD":
      return {
        ...state,
        activeGamepadIndex: action.payload,
      };

    case "SET_PROFILES":
      return {
        ...state,
        profiles: action.payload,
      };

    case "SET_ACTIVE_PROFILE":
      return {
        ...state,
        activeProfile: action.payload,
      };

    case "SET_LISTENING":
      return {
        ...state,
        isListening: action.payload,
      };

    case "SET_LOADING":
      return {
        ...state,
        isLoading: action.payload,
      };

    case "SET_ERROR":
      return {
        ...state,
        error: action.payload,
      };

    case "RESET_ERROR":
      return {
        ...state,
        error: null,
      };

    case "PROFILE_ADDED": {
      const newProfile = action.payload;
      const profiles = [...state.profiles, newProfile];
      return {
        ...state,
        profiles,
        activeProfile: newProfile,
      };
    }

    case "PROFILE_DELETED": {
      const profileName = action.payload;
      const profiles = state.profiles.filter((p) => p.name !== profileName);
      const activeProfile =
        state.activeProfile?.name === profileName ? null : state.activeProfile;
      return {
        ...state,
        profiles,
        activeProfile,
      };
    }

    case "PROFILE_UPDATED": {
      const updatedProfile = action.payload;
      const profiles = state.profiles.map((p) =>
        p.name === updatedProfile.name ? updatedProfile : p,
      );
      const activeProfile =
        state.activeProfile?.name === updatedProfile.name
          ? updatedProfile
          : state.activeProfile;
      return {
        ...state,
        profiles,
        activeProfile,
      };
    }

    default:
      return state;
  }
}
