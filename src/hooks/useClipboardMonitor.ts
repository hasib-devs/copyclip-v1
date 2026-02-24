import { useEffect, useCallback, useRef } from "react";
import { useClipboard } from "./useClipboard";
import clipboard from "tauri-plugin-clipboard-api";
import { databaseService } from "@/services/databaseService";

/**
 * Custom hook to monitor clipboard changes
 * Automatically adds clipboard updates to history
 * Handles event listener cleanup properly
 */
export const useClipboardMonitor = (enabled: boolean = true) => {
  const { addItem, setError, startMonitoring, stopMonitoring } = useClipboard();

  // Keep track of unlisteners to clean them up
  const unlistenersRef = useRef<Array<() => void>>([]);

  /**
   * Set up clipboard event listeners
   */
  const setupListeners = useCallback(async () => {
    console.log("Setting up clipboard listeners...");
    try {
      // Create text update listener
      const unlistenText = await clipboard.onTextUpdate((text: string) => {
        if (text && text.trim()) {
          const item = {
            content: text,
            type: "text" as const,
            isPinned: false,
          };
          addItem(item);
          // Save to database asynchronously
          databaseService
            .saveItem(item)
            .catch((err) => console.error("Failed to save text to DB:", err));
        }
      });

      // Create image update listener
      const unlistenImage = await clipboard.onImageUpdate((base64: string) => {
        if (base64) {
          const item = {
            content: "[Image]",
            type: "image" as const,
            isPinned: false,
            imageBase64: base64,
          };
          addItem(item);
          // Save to database asynchronously
          databaseService
            .saveItem(item)
            .catch((err) => console.error("Failed to save image to DB:", err));
        }
      });

      // Create HTML update listener
      const unlistenHtml = await clipboard.onHTMLUpdate((html: string) => {
        if (html && html.trim()) {
          const item = {
            content: html,
            type: "html" as const,
            isPinned: false,
          };
          addItem(item);
          // Save to database asynchronously
          databaseService
            .saveItem(item)
            .catch((err) => console.error("Failed to save HTML to DB:", err));
        }
      });

      // Create file update listener
      const unlistenFiles = await clipboard.onFilesUpdate((files: string[]) => {
        if (files && files.length > 0) {
          const item = {
            content: files.join(", "),
            type: "file" as const,
            isPinned: false,
            filePaths: files,
          };
          addItem(item);
          // Save to database asynchronously
          databaseService
            .saveItem(item)
            .catch((err) => console.error("Failed to save files to DB:", err));
        }
      });

      // Start listening for clipboard events
      const unlistenClipboard = await clipboard.startListening();

      // Store unlisteners for cleanup
      unlistenersRef.current = [
        unlistenText as () => void,
        unlistenImage as () => void,
        unlistenHtml as () => void,
        unlistenFiles as () => void,
        unlistenClipboard as () => void,
      ];

      await startMonitoring();
    } catch (error) {
      const errorMessage =
        error instanceof Error ? error.message : "Unknown error";
      setError(`Failed to set up clipboard monitoring: ${errorMessage}`);
      console.error("Clipboard monitoring setup error:", error);
    }
  }, [addItem, startMonitoring, setError]);

  /**
   * Clean up event listeners
   */
  const cleanup = useCallback(async () => {
    try {
      // Call all unlisteners
      unlistenersRef.current.forEach((unlisten) => {
        try {
          unlisten();
        } catch (err) {
          console.error("Unlisten error:", err);
        }
      });
      unlistenersRef.current = [];
      await stopMonitoring();
    } catch (error) {
      console.error("Error during clipboard listener cleanup:", error);
    }
  }, [stopMonitoring]);

  // Set up listeners when component mounts or when enabled changes
  useEffect(() => {
    if (enabled) {
      setupListeners();
    }

    return () => {
      if (enabled) {
        cleanup();
      }
    };
  }, [enabled, setupListeners, cleanup]);

  return { setupListeners, cleanup };
};
