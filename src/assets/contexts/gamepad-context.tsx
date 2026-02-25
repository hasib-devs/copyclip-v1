import React, { createContext, useReducer, ReactNode } from "react";
import { GamepadContextState, GamepadAction } from "@/types/gamepad.types";
import { gamepadReducer, initialGamepadState } from "@/hooks/gamepadReducer";

/**
 * Gamepad Context
 */
export const GamepadContext = createContext<
  | {
      state: GamepadContextState;
      dispatch: React.Dispatch<GamepadAction>;
    }
  | undefined
>(undefined);

/**
 * Gamepad Context Provider Component
 */
export function GamepadProvider({ children }: { children: ReactNode }) {
  const [state, dispatch] = useReducer(gamepadReducer, initialGamepadState);

  return (
    <GamepadContext.Provider value={{ state, dispatch }}>
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
