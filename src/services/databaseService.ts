import { invoke } from "@tauri-apps/api/core";
import { ItemType } from "@/types/clipboard";

/**
 * Database service
 * Calls Tauri backend commands for clipboard persistence
 */
export const databaseService = {
  /**
   * Save clipboard item to database
   */
  async saveItem(item: Omit<ItemType, "id" | "timestamp">): Promise<boolean> {
    try {
      const id = crypto.randomUUID();
      console.log(
        "[DB-SERVICE] Calling save_clipboard_item. ID:",
        id,
        "Type:",
        item.type,
        "Content length:",
        item.content.length,
      );
      const result = await invoke<boolean>("save_clipboard_item", {
        id,
        content: item.content,
        itemType: item.type,
        imageBase64: item.imageBase64 || null,
        filePaths: item.filePaths ? JSON.stringify(item.filePaths) : null,
      });
      console.log("[DB-SERVICE] save_clipboard_item returned:", result);
      return result;
    } catch (error) {
      console.error("[DB-SERVICE] Failed to save item:", error);
      return false;
    }
  },

  /**
   * Get all clipboard items with optional filters
   */
  async getItems(
    search?: string,
    itemType?: string,
    isPinned?: boolean,
    limit: number = 100,
    offset: number = 0,
  ): Promise<ItemType[]> {
    try {
      const items = await invoke<any[]>("get_clipboard_items", {
        search: search || null,
        itemType: itemType || null,
        isPinned: isPinned !== undefined ? isPinned : null,
        limit,
        offset,
      });

      // Map database items to frontend format
      return items.map((item) => ({
        id: item.id,
        content: item.content,
        type: item.item_type as "text" | "image" | "html" | "file",
        timestamp: item.timestamp,
        isPinned: item.is_pinned,
        imageBase64: item.image_base64,
        filePaths: item.file_paths ? JSON.parse(item.file_paths) : undefined,
      }));
    } catch (error) {
      console.error("Failed to get items:", error);
      return [];
    }
  },

  /**
   * Get single item by id
   */
  async getItem(id: string): Promise<ItemType | null> {
    try {
      const item = await invoke<any | null>("get_clipboard_item", { id });
      if (!item) return null;

      return {
        id: item.id,
        content: item.content,
        type: item.item_type as "text" | "image" | "html" | "file",
        timestamp: item.timestamp,
        isPinned: item.is_pinned,
        imageBase64: item.image_base64,
        filePaths: item.file_paths ? JSON.parse(item.file_paths) : undefined,
      };
    } catch (error) {
      console.error("Failed to get item:", error);
      return null;
    }
  },

  /**
   * Update item (toggle pin status)
   */
  async updateItem(id: string, isPinned: boolean): Promise<boolean> {
    try {
      return await invoke<boolean>("update_clipboard_item", { id, isPinned });
    } catch (error) {
      console.error("Failed to update item:", error);
      return false;
    }
  },

  /**
   * Delete item by id
   */
  async deleteItem(id: string): Promise<boolean> {
    try {
      console.log("Deleting item from database:", id);
      const result = await invoke<boolean>("delete_clipboard_item", { id });
      console.log("Delete result:", result);
      return result;
    } catch (error) {
      console.error("Failed to delete item:", error);
      throw error; // Re-throw to catch in UI
    }
  },

  /**
   * Clear all clipboard history
   */
  async clearAll(): Promise<boolean> {
    try {
      return await invoke<boolean>("clear_clipboard_history");
    } catch (error) {
      console.error("Failed to clear history:", error);
      return false;
    }
  },

  /**
   * Get total count of items
   */
  async getCount(): Promise<number> {
    try {
      return await invoke<number>("get_clipboard_count");
    } catch (error) {
      console.error("Failed to get count:", error);
      return 0;
    }
  },

  /**
   * Load all items on app startup
   */
  async loadInitialHistory(): Promise<ItemType[]> {
    try {
      const items = await invoke<any[]>("load_initial_history");

      return items.map((item) => ({
        id: item.id,
        content: item.content,
        type: item.item_type as "text" | "image" | "html" | "file",
        timestamp: item.timestamp,
        isPinned: item.is_pinned,
        imageBase64: item.image_base64,
        filePaths: item.file_paths ? JSON.parse(item.file_paths) : undefined,
      }));
    } catch (error) {
      console.error("Failed to load initial history:", error);
      return [];
    }
  },
};
