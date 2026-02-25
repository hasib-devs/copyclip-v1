import { useCallback } from "react";
import { invoke } from "@tauri-apps/api/core";
import {
  Gamepad,
  GamepadProfile,
  GamepadContextState,
  GamepadAction,
} from "@/types/gamepad.types";

export function useGamepad(
  state: GamepadContextState,
  dispatch: React.Dispatch<GamepadAction>,
) {
  /**
   * Start gamepad listener
   */
  const startListening = useCallback(async () => {
    try {
      console.info("[useGamepad::startListening] Starting gamepad listener...");
      dispatch({ type: "SET_LOADING", payload: true });
      dispatch({ type: "RESET_ERROR" });

      console.info("[useGamepad::startListening] Invoking 'start_gamepad' command...");
      await invoke("start_gamepad");
      console.info("[useGamepad::startListening] start_gamepad command succeeded");
      
      dispatch({ type: "SET_LISTENING", payload: true });

      // Fetch initial gamepads
      console.info("[useGamepad::startListening] Fetching connected gamepads...");
      const gamepads = await invoke<Gamepad[]>("get_gamepads");
      console.info("[useGamepad::startListening] Found", gamepads.length, "connected gamepads");
      dispatch({ type: "SET_GAMEPADS", payload: gamepads });
    } catch (err) {
      console.error("[useGamepad::startListening] Failed:", err);
      const errorMsg = err instanceof Error ? err.message : String(err);
      dispatch({ type: "SET_ERROR", payload: errorMsg });
      console.error("Failed to start gamepad listener:", err);
    } finally {
      dispatch({ type: "SET_LOADING", payload: false });
    }
  }, [dispatch]);

  /**
   * Stop gamepad listener
   */
  const stopListening = useCallback(async () => {
    try {
      console.info("[useGamepad::stopListening] Stopping gamepad listener...");
      dispatch({ type: "SET_ERROR", payload: null });
      
      console.info("[useGamepad::stopListening] Invoking 'stop_gamepad' command...");
      await invoke("stop_gamepad");
      console.info("[useGamepad::stopListening] stop_gamepad command succeeded");
      
      dispatch({ type: "SET_LISTENING", payload: false });
    } catch (err) {
      console.error("[useGamepad::stopListening] Failed:", err);
      const errorMsg = err instanceof Error ? err.message : String(err);
      dispatch({ type: "SET_ERROR", payload: errorMsg });
      console.error("Failed to stop gamepad listener:", err);
    }
  }, [dispatch]);

  /**
   * Toggle listening state
   */
  const toggleListening = useCallback(
    async (enabled: boolean) => {
      if (enabled) {
        await startListening();
      } else {
        await stopListening();
      }
    },
    [startListening, stopListening],
  );

  /**
   * Refresh gamepad list
   */
  const refreshGamepads = useCallback(async () => {
    try {
      dispatch({ type: "RESET_ERROR" });
      const gamepads = await invoke<Gamepad[]>("get_gamepads");
      dispatch({ type: "SET_GAMEPADS", payload: gamepads });
    } catch (err) {
      const errorMsg = err instanceof Error ? err.message : String(err);
      dispatch({ type: "SET_ERROR", payload: errorMsg });
      console.error("Failed to refresh gamepads:", err);
    }
  }, [dispatch]);

  /**
   * Set active gamepad
   */
  const setActiveGamepad = useCallback(
    (index: number) => {
      dispatch({ type: "SET_ACTIVE_GAMEPAD", payload: index });
    },
    [dispatch],
  );

  /**
   * Load all profiles
   */
  const loadProfiles = useCallback(async () => {
    try {
      dispatch({ type: "SET_LOADING", payload: true });
      dispatch({ type: "RESET_ERROR" });

      const profiles = await invoke<GamepadProfile[]>("get_gamepad_profiles");
      dispatch({ type: "SET_PROFILES", payload: profiles });

      if (profiles.length > 0) {
        dispatch({ type: "SET_ACTIVE_PROFILE", payload: profiles[0] });
      }
    } catch (err) {
      const errorMsg = err instanceof Error ? err.message : String(err);
      dispatch({ type: "SET_ERROR", payload: errorMsg });
      console.error("Failed to load profiles:", err);
    } finally {
      dispatch({ type: "SET_LOADING", payload: false });
    }
  }, [dispatch]);

  /**
   * Save/create new profile
   */
  const saveProfile = useCallback(
    async (profile: GamepadProfile) => {
      try {
        dispatch({ type: "RESET_ERROR" });
        await invoke("save_gamepad_profile", { profile });

        // Check if profile already existed
        const existingProfile = state.profiles.find(
          (p) => p.name === profile.name,
        );
        if (existingProfile) {
          dispatch({ type: "PROFILE_UPDATED", payload: profile });
        } else {
          dispatch({ type: "PROFILE_ADDED", payload: profile });
        }
      } catch (err) {
        const errorMsg = err instanceof Error ? err.message : String(err);
        dispatch({ type: "SET_ERROR", payload: errorMsg });
        console.error("Failed to save profile:", err);
        throw err;
      }
    },
    [state.profiles, dispatch],
  );

  /**
   * Delete profile
   */
  const deleteProfile = useCallback(
    async (profileName: string) => {
      try {
        dispatch({ type: "RESET_ERROR" });
        await invoke("delete_gamepad_profile", { profile_name: profileName });
        dispatch({ type: "PROFILE_DELETED", payload: profileName });
      } catch (err) {
        const errorMsg = err instanceof Error ? err.message : String(err);
        dispatch({ type: "SET_ERROR", payload: errorMsg });
        console.error("Failed to delete profile:", err);
        throw err;
      }
    },
    [dispatch],
  );

  /**
   * Set active profile
   */
  const setActiveProfile = useCallback(
    async (profileName: string) => {
      try {
        dispatch({ type: "RESET_ERROR" });
        await invoke("set_active_gamepad_profile", {
          profile_name: profileName,
        });

        const profile = state.profiles.find((p) => p.name === profileName);
        if (profile) {
          dispatch({ type: "SET_ACTIVE_PROFILE", payload: profile });
        }
      } catch (err) {
        const errorMsg = err instanceof Error ? err.message : String(err);
        dispatch({ type: "SET_ERROR", payload: errorMsg });
        console.error("Failed to set active profile:", err);
        throw err;
      }
    },
    [state.profiles, dispatch],
  );

  /**
   * Get active gamepad
   */
  const getActiveGamepad = useCallback(() => {
    return state.gamepads[state.activeGamepadIndex] || null;
  }, [state.gamepads, state.activeGamepadIndex]);

  return {
    // State
    gamepads: state.gamepads,
    activeGamepad: getActiveGamepad(),
    profiles: state.profiles,
    activeProfile: state.activeProfile,
    isListening: state.isListening,
    isLoading: state.isLoading,
    error: state.error,

    // Actions
    startListening,
    stopListening,
    toggleListening,
    refreshGamepads,
    setActiveGamepad,
    loadProfiles,
    saveProfile,
    deleteProfile,
    setActiveProfile,
  };
}

export type UseGamepadReturn = ReturnType<typeof useGamepad>;
