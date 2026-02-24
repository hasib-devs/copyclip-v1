/**
 * Clipboard History Item
 * Represents a single clipboard entry with metadata
 */
export interface ClipboardItem {
  /** Unique identifier */
  id: string;
  /** Clipboard content */
  content: string;
  /** Content type: text, html, image, file */
  type: "text" | "html" | "image" | "file";
  /** Timestamp when added */
  timestamp: number;
  /** Whether the item is pinned */
  isPinned: boolean;
  /** File paths (if type is file) */
  filePaths?: string[];
  /** Preview for images (base64) */
  imageBase64?: string;
}

/**
 * Clipboard History State
 * Manages the overall clipboard history state
 */
export interface ClipboardHistoryState {
  /** All clipboard items in history */
  items: ClipboardItem[];
  /** Whether clipboard monitoring is active */
  isMonitoring: boolean;
  /** Error message if any */
  error: string | null;
  /** Maximum items to keep in history */
  maxItems: number;
}

/**
 * Clipboard Context Type
 * Defines all available actions and state
 */
export interface ClipboardContextType extends ClipboardHistoryState {
  /** Add item to beginning of history, manage max items */
  addItem: (item: Omit<ClipboardItem, "id" | "timestamp">) => void;
  /** Remove item by id */
  removeItem: (id: string) => void;
  /** Clear all history */
  clearHistory: () => void;
  /** Toggle pin status of item */
  togglePin: (id: string) => void;
  /** Copy item content to clipboard */
  copyToClipboard: (content: string) => Promise<void>;
  /** Start monitoring clipboard */
  startMonitoring: () => Promise<void>;
  /** Stop monitoring clipboard */
  stopMonitoring: () => Promise<void>;
  /** Search items by query */
  searchItems: (query: string) => ClipboardItem[];
  /** Get pinned items */
  getPinnedItems: () => ClipboardItem[];
  /** Set error message */
  setError: (error: string | null) => void;
  /** Get paginated items */
  getPaginatedItems: (page: number, pageSize: number) => ClipboardItem[];
}
