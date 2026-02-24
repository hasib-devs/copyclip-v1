import { useEffect } from "react";
import { useClipboard } from "./useClipboard";
import { databaseService } from "@/services/databaseService";

/**
 * Hook to load initial clipboard history from database
 * Should be called once on app startup
 */
export const useLoadClipboardHistory = () => {
  const { items, addItem } = useClipboard();

  useEffect(() => {
    // Only load if history is empty (first load)
    if (items.length === 0) {
      loadHistory();
    }
  }, []);

  const loadHistory = async () => {
    try {
      const dbItems = await databaseService.loadInitialHistory();
      dbItems.forEach((item) => {
        addItem({
          content: item.content,
          type: item.type,
          isPinned: item.isPinned,
          imageBase64: item.imageBase64,
          filePaths: item.filePaths,
        });
      });
    } catch (error) {
      console.error("Failed to load clipboard history:", error);
    }
  };

  return { loadHistory };
};
