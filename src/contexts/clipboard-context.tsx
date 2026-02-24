import { clipboardReducer } from "@/hooks/clipboardReducer";
import {
  ClipboardContextType,
  ClipboardHistoryState,
  ItemType,
} from "@/types/clipboard.types";
import {
  createContext,
  ReactNode,
  useCallback,
  useReducer,
  useState,
} from "react";
import {
  writeHtml,
  writeImageBase64,
  writeText,
} from "tauri-plugin-clipboard-api";

/**
 * Create the context
 */
export const ClipboardContext = createContext<ClipboardContextType | undefined>(
  undefined,
);

const initialState: ClipboardHistoryState = {
  items: [],
  isMonitoring: false,
  error: null,
  maxItems: 100,
};

/**
 * ClipboardProvider component
 * Wraps the application to provide clipboard context
 */
interface ClipboardProviderProps {
  children: ReactNode;
  maxItems?: number;
}

export const ClipboardProvider: React.FC<ClipboardProviderProps> = ({
  children,
  maxItems = 100,
}) => {
  const [pausedCopying, setPausedCopying] = useState(false);

  const [state, dispatch] = useReducer(clipboardReducer, {
    ...initialState,
    maxItems,
  });

  /**
   * Initialize items (used for loading from database)
   */

  const initializeItems = useCallback((items: ItemType[]) => {
    dispatch({ type: "INITIALIZE_ITEMS", payload: items });
  }, []);

  /**
   * Add item to clipboard history
   */
  const addItem = useCallback((item: ItemType) => {
    dispatch({ type: "ADD_ITEM", payload: item });
  }, []);

  /**
   * Remove item from history
   */
  const removeItem = useCallback((id: string) => {
    dispatch({ type: "REMOVE_ITEM", payload: id });
  }, []);

  /**
   * Clear all history
   */
  const clearHistory = useCallback(() => {
    dispatch({ type: "CLEAR_HISTORY" });
  }, []);

  /**
   * Toggle pin status
   */
  const togglePin = useCallback((id: string) => {
    dispatch({ type: "TOGGLE_PIN", payload: id });
  }, []);

  /**
   * Copy to system clipboard using clipboard API
   */
  const copyToClipboard = useCallback(
    async (content: string, type: ItemType["type"] = "text") => {
      try {
        if (type === "text") {
          await writeText(content);
        } else if (type === "html") {
          await writeHtml(content);
        } else if (type === "image_base64") {
          await writeImageBase64(content);
        } else {
          throw new Error("Unsupported content type for clipboard");
        }
      } catch (error) {
        // Fallback to browser clipboard API
        try {
          await navigator.clipboard.writeText(content);
        } catch (err) {
          dispatch({
            type: "SET_ERROR",
            payload: "Failed to copy to clipboard",
          });
          throw err;
        }
      }
    },
    [],
  );

  /**
   * Start monitoring clipboard
   */
  const startMonitoring = useCallback(async () => {
    try {
      dispatch({ type: "SET_MONITORING", payload: true });
      dispatch({ type: "SET_ERROR", payload: null });
    } catch (error) {
      dispatch({
        type: "SET_ERROR",
        payload: `Failed to start monitoring: ${error instanceof Error ? error.message : "Unknown error"}`,
      });
      dispatch({ type: "SET_MONITORING", payload: false });
      throw error;
    }
  }, []);

  /**
   * Stop monitoring clipboard
   */
  const stopMonitoring = useCallback(async () => {
    try {
      dispatch({ type: "SET_MONITORING", payload: false });
    } catch (error) {
      dispatch({
        type: "SET_ERROR",
        payload: `Failed to stop monitoring: ${error instanceof Error ? error.message : "Unknown error"}`,
      });
      throw error;
    }
  }, []);

  /**
   * Search items by query
   */
  const searchItems = useCallback(
    (query: string) => {
      if (!query.trim()) return state.items;

      const lowerQuery = query.toLowerCase();
      const data = state.items.filter((item) =>
        item.content.toLowerCase().includes(lowerQuery),
      );

      return data;
    },
    [state.items],
  );

  /**
   * Get pinned items
   */
  const getPinnedItems = useCallback(() => {
    return state.items.filter((item) => item.isPinned);
  }, [state.items]);

  /**
   * Set error message
   */
  const setError = useCallback((error: string | null) => {
    dispatch({ type: "SET_ERROR", payload: error });
  }, []);

  const value: ClipboardContextType = {
    ...state,
    addItem,
    removeItem,
    clearHistory,
    togglePin,
    copyToClipboard,
    startMonitoring,
    stopMonitoring,
    searchItems,
    getPinnedItems,
    setError,
    initializeItems,
    pausedCopying,
    setPausedCopying,
  };

  return (
    <ClipboardContext.Provider value={value}>
      {children}
    </ClipboardContext.Provider>
  );
};
