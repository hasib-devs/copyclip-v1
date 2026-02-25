import { useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Gamepad, GamepadAction } from "@/types/gamepad.types";

/**
 * Hook to monitor and update gamepad state via polling
 * Fetches latest gamepad state when listening is enabled
 */
export function useGamepadMonitor(
  isListening: boolean,
  dispatch: React.Dispatch<GamepadAction>,
  pollInterval: number = 100, // ms
) {
  useEffect(() => {
    if (!isListening) return;

    const interval = setInterval(async () => {
      try {
        const gamepads = await invoke<Gamepad[]>("get_gamepads");
        dispatch({ type: "SET_GAMEPADS", payload: gamepads });
      } catch (err) {
        console.error("Failed to poll gamepads:", err);
        // Don't dispatch error on poll failures - keep monitoring
      }
    }, pollInterval);

    return () => clearInterval(interval);
  }, [isListening, dispatch, pollInterval]);
}

/**
 * Hook to load profiles on component mount
 */
export function useLoadGamepadProfiles(
  dispatch: React.Dispatch<GamepadAction>,
) {
  useEffect(() => {
    const loadProfiles = async () => {
      try {
        dispatch({ type: "SET_LOADING", payload: true });
        const profiles = await invoke<any[]>("get_gamepad_profiles");
        dispatch({ type: "SET_PROFILES", payload: profiles });
      } catch (err) {
        console.error("Failed to load gamepad profiles:", err);
        dispatch({
          type: "SET_ERROR",
          payload: "Failed to load profiles",
        });
      } finally {
        dispatch({ type: "SET_LOADING", payload: false });
      }
    };

    loadProfiles();
  }, [dispatch]);
}
