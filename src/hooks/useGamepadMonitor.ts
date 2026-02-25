import { useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Gamepad, GamepadAction, GamepadProfile } from "@/types/gamepad.types";

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
    if (!isListening) {
      console.info("[GamepadMonitor] Not listening, skipping polling");
      return;
    }

    console.info("[GamepadMonitor] Starting gamepad polling loop (backend already listening)...");
    const interval = setInterval(async () => {
      try {
        console.info("[GamepadMonitor] Polling gamepads...");
        const gamepads = await invoke<Gamepad[]>("get_gamepads");
        dispatch({ type: "SET_GAMEPADS", payload: gamepads });
      } catch (err) {
        console.error("[GamepadMonitor] Failed to poll gamepads:", err);
        // Don't dispatch error on poll failures - keep monitoring
      }
    }, pollInterval);

    return () => {
      console.info("[GamepadMonitor] Stopping polling loop");
      clearInterval(interval);
    };
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
        console.info(
          "[useLoadGamepadProfiles] Starting profile load from Tauri command...",
        );
        dispatch({ type: "SET_LOADING", payload: true });
        console.info(
          "[useLoadGamepadProfiles] Invoking 'get_gamepad_profiles' command...",
        );
        const profiles = await invoke<GamepadProfile[]>("get_gamepad_profiles");

        console.log("[useLoadGamepadProfiles] Response from Tauri:", {
          profiles,
        });
        console.info(
          `[useLoadGamepadProfiles] Received ${profiles.length} profiles from backend`,
        );

        if (profiles.length === 0) {
          console.warn(
            "[useLoadGamepadProfiles] No profiles returned (fresh install or no saved profiles)",
          );
        } else {
          console.info(
            "[useLoadGamepadProfiles] Processing profiles:",
            profiles,
          );
        }

        dispatch({ type: "SET_PROFILES", payload: profiles });
        console.info("[useLoadGamepadProfiles] Profiles dispatched to state");
      } catch (err) {
        console.error(
          "[useLoadGamepadProfiles] Failed to load gamepad profiles:",
          err,
        );
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
