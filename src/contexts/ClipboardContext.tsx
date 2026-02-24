import React, {
  createContext,
  useCallback,
  useReducer,
  ReactNode,
} from "react";
import {
  ClipboardItem,
  ClipboardContextType,
  ClipboardHistoryState,
} from "@/types/clipboard";

/**
 * Create the context
 */
export const ClipboardContext = createContext<ClipboardContextType | undefined>(
  undefined,
);

/**
 * Action types for reducer
 */
type ClipboardAction =
  | { type: "ADD_ITEM"; payload: Omit<ClipboardItem, "id" | "timestamp"> }
  | { type: "REMOVE_ITEM"; payload: string }
  | { type: "CLEAR_HISTORY" }
  | { type: "TOGGLE_PIN"; payload: string }
  | { type: "SET_MONITORING"; payload: boolean }
  | { type: "SET_ERROR"; payload: string | null }
  | { type: "REORDER_ITEMS"; payload: ClipboardItem[] };

/**
 * Initial state
 */
const initialState: ClipboardHistoryState = {
  items: [],
  isMonitoring: false,
  error: null,
  maxItems: 100,
};

/**
 * Reducer function to manage clipboard state
 */
function clipboardReducer(
  state: ClipboardHistoryState,
  action: ClipboardAction,
): ClipboardHistoryState {
  switch (action.type) {
    case "ADD_ITEM": {
      const newItem: ClipboardItem = {
        ...action.payload,
        id: crypto.randomUUID(),
        timestamp: Date.now(),
      };

      // Avoid duplicates - if same content was just added, don't add again
      if (
        state.items.length > 0 &&
        state.items[0].content === newItem.content &&
        newItem.type === state.items[0].type
      ) {
        return state;
      }

      // Keep at most maxItems
      const newItems = [newItem, ...state.items].slice(0, state.maxItems);
      return { ...state, items: newItems };
    }

    case "REMOVE_ITEM":
      return {
        ...state,
        items: state.items.filter((item) => item.id !== action.payload),
      };

    case "CLEAR_HISTORY":
      return { ...state, items: [] };

    case "TOGGLE_PIN":
      return {
        ...state,
        items: state.items.map((item) =>
          item.id === action.payload
            ? { ...item, isPinned: !item.isPinned }
            : item,
        ),
      };

    case "SET_MONITORING":
      return { ...state, isMonitoring: action.payload };

    case "SET_ERROR":
      return { ...state, error: action.payload };

    case "REORDER_ITEMS":
      // Ensure pinned items stay at top
      const pinnedItems = action.payload.filter((item) => item.isPinned);
      const unpinnedItems = action.payload.filter((item) => !item.isPinned);
      return {
        ...state,
        items: [...pinnedItems, ...unpinnedItems],
      };

    default:
      return state;
  }
}

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
  const [state, dispatch] = useReducer(clipboardReducer, {
    ...initialState,
    maxItems,
  });

  /**
   * Add item to clipboard history
   */
  const addItem = useCallback(
    (item: Omit<ClipboardItem, "id" | "timestamp">) => {
      dispatch({ type: "ADD_ITEM", payload: item });
    },
    [],
  );

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
  const copyToClipboard = useCallback(async (content: string) => {
    try {
      // Try using the Tauri clipboard plugin
      const clipboard = await import("tauri-plugin-clipboard-api");
      await clipboard.writeText(content);
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
  }, []);

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
      return state.items.filter((item) =>
        item.content.toLowerCase().includes(lowerQuery),
      );
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

  /**
   * Get paginated items
   */
  const getPaginatedItems = useCallback(
    (page: number, pageSize: number) => {
      const start = page * pageSize;
      const end = start + pageSize;
      return state.items.slice(start, end);
    },
    [state.items],
  );

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
    getPaginatedItems,
  };

  return (
    <ClipboardContext.Provider value={value}>
      {children}
    </ClipboardContext.Provider>
  );
};
