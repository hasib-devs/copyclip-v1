import { gamepadReducer, initialGamepadState } from "@/hooks/gamepadReducer";
import { Gamepad, GamepadProfile } from "@/types/gamepad.types";
import { invoke } from "@tauri-apps/api/core";
import React, {
  createContext,
  ReactNode,
  useCallback,
  useEffect,
  useReducer,
} from "react";

type GamepadContextType = {
  startListening: () => Promise<void>;
  stopListening: () => Promise<void>;
};

/**
 * Gamepad Context
 */
export const GamepadContext = createContext<GamepadContextType | undefined>(
  undefined,
);

/**
 * Gamepad Context Provider Component
 */
export function GamepadProvider({ children }: { children: ReactNode }) {
  const [state, dispatch] = useReducer(gamepadReducer, initialGamepadState);

  /**
   * Start gamepad listener
   */
  const startListening = useCallback(async () => {
    try {
      dispatch({ type: "SET_LOADING", payload: true });
      dispatch({ type: "RESET_ERROR" });

      await invoke("start_gamepad");
      dispatch({ type: "SET_LISTENING", payload: true });

      const gamepads = await invoke<Gamepad[]>("get_gamepads");
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
      dispatch({ type: "SET_ERROR", payload: null });

      await invoke("stop_gamepad");

      dispatch({ type: "SET_LISTENING", payload: false });
    } catch (err) {
      console.error("[useGamepad::stopListening] Failed:", err);
      const errorMsg = err instanceof Error ? err.message : String(err);
      dispatch({ type: "SET_ERROR", payload: errorMsg });
      console.error("Failed to stop gamepad listener:", err);
    }
  }, [dispatch]);

  // Start listener on mount if isListening is true (default)
  useEffect(() => {
    if (!state.isListening) {
      startListening();
    }

    return () => {
      if (state.isListening) {
        stopListening();
      }
    };
  }, [state.isListening, startListening, stopListening]);

  return (
    <GamepadContext.Provider value={{ stopListening, startListening }}>
      {children}
    </GamepadContext.Provider>
  );
}

/**
 * Hook to use gamepad context
 * Throws if used outside of GamepadProvider
 */
export function useGamepadContext() {
  const context = React.useContext(GamepadContext);
  if (!context) {
    throw new Error("useGamepadContext must be used within a GamepadProvider");
  }
  return context;
}
