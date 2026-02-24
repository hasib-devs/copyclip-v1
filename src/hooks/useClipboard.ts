import { useContext } from "react";
import { ClipboardContext } from "@/contexts/clipboard-context";
import { ClipboardContextType } from "@/types/clipboard.types";

/**
 * Custom hook to access clipboard context
 * Throws error if used outside ClipboardProvider
 */
export const useClipboard = (): ClipboardContextType => {
  const context = useContext(ClipboardContext);

  if (context === undefined) {
    throw new Error(
      "useClipboard must be used within a ClipboardProvider. " +
        "Make sure your component is wrapped with <ClipboardProvider>",
    );
  }

  return context;
};
