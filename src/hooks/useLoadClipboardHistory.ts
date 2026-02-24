import { useEffect } from "react";
import { useClipboard } from "./useClipboard";
import { databaseService } from "@/services/databaseService";

/**
 * Hook to load initial clipboard history from database
 * Should be called once on app startup
 */
export const useLoadClipboardHistory = () => {
  const { items, initializeItems } = useClipboard();

  useEffect(() => {
    // Only load if history is empty (first load)
    if (items.length === 0) {
      loadHistory();
    }
  }, []);

  const loadHistory = async () => {
    try {
      const dbItems = await databaseService.loadInitialHistory();
      initializeItems(dbItems);
    } catch (error) {
      console.error("Failed to load clipboard history:", error);
    }
  };

  return { loadHistory };
};
