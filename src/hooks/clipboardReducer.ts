import { ClipboardHistoryState, ItemType } from "@/types/clipboard.types";

/**
 * Action types for reducer
 */
type ClipboardAction =
  | { type: "ADD_ITEM"; payload: ItemType }
  | { type: "REMOVE_ITEM"; payload: string }
  | { type: "CLEAR_HISTORY" }
  | { type: "TOGGLE_PIN"; payload: string }
  | { type: "SET_MONITORING"; payload: boolean }
  | { type: "SET_ERROR"; payload: string | null }
  | { type: "REORDER_ITEMS"; payload: ItemType[] }
  | { type: "INITIALIZE_ITEMS"; payload: ItemType[] };
/**
 * Initial state
 */

/**
 * Reducer function to manage clipboard state
 */
export function clipboardReducer(
  state: ClipboardHistoryState,
  action: ClipboardAction,
): ClipboardHistoryState {
  switch (action.type) {
    case "ADD_ITEM": {
      // Avoid duplicates - if same content was just added, don't add again
      if (
        state.items.length > 0 &&
        state.items[0].content === action.payload.content &&
        action.payload.type === state.items[0].type
      ) {
        return state;
      }

      // Keep at most maxItems
      const newItems = [action.payload, ...state.items].slice(
        0,
        state.maxItems,
      );
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
      const updatedItems = state.items.map((item) =>
        item.id === action.payload
          ? { ...item, isPinned: !item.isPinned }
          : item,
      );
      // Re-sort: pinned items first, then by timestamp DESC
      const pinnedToggle = updatedItems.filter((item) => item.isPinned);
      const unpinnedToggle = updatedItems.filter((item) => !item.isPinned);
      return {
        ...state,
        items: [...pinnedToggle, ...unpinnedToggle],
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

    case "INITIALIZE_ITEMS":
      return {
        ...state,
        items: action.payload,
      };
    default:
      return state;
  }
}
