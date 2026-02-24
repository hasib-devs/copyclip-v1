import { useEffect, useCallback, useRef } from "react";
import { useClipboard } from "./useClipboard";
import {
  onTextUpdate,
  onImageUpdate,
  onRTFUpdate,
  onImageBinaryUpdate,
  onSomethingUpdate,
  onHTMLUpdate,
  onFilesUpdate,
  startListening,
  onClipboardUpdate,
} from "tauri-plugin-clipboard-api";
import { databaseService } from "@/services/databaseService";
import { ItemType } from "@/types/clipboard.types";

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
    try {
      console.info("[CLIPBOARD] Setting up clipboard listeners...");
      // Create text update listener
      const unlistenText = await onTextUpdate((text: string) => {
        if (text && text.trim()) {
          const item: ItemType = {
            content: text,
            type: "text" as const,
            isPinned: false,
            id: crypto.randomUUID(),
            timestamp: Date.now(),
          };
          addItem(item);
          // Save to database asynchronously
          databaseService
            .saveItem(item)
            .then((result) => console.info("[CLIPBOARD] Save result:", result))
            .catch((err) =>
              console.error("[CLIPBOARD] Failed to save text to DB:", err),
            );
        } else {
          console.info("[CLIPBOARD] Text is empty or whitespace only");
        }
      });

      // Create image update listener
      const unlistenImage = await onImageUpdate((base64: string) => {
        if (base64) {
          const item: ItemType = {
            content: "[Image]",
            type: "image_base64" as const,
            isPinned: false,
            id: crypto.randomUUID(),
            timestamp: Date.now(),
            imageBase64: base64,
          };
          addItem(item);
          // Save to database asynchronously
          databaseService
            .saveItem(item)
            .catch((err) =>
              console.error("[CLIPBOARD] Failed to save image to DB:", err),
            );
        }
      });

      // Create HTML update listener
      const unlistenHtml = await onHTMLUpdate((html: string) => {
        if (html && html.trim()) {
          const item = {
            content: html,
            type: "html" as const,
            isPinned: false,
            id: crypto.randomUUID(),
            timestamp: Date.now(),
          };
          addItem(item);
          // Save to database asynchronously
          databaseService
            .saveItem(item)
            .catch((err) =>
              console.error("[CLIPBOARD] Failed to save HTML to DB:", err),
            );
        }
      });

      // Create file update listener
      const unlistenFiles = await onFilesUpdate((files: string[]) => {
        if (files && files.length > 0) {
          const item: ItemType = {
            content: files.join(", "),
            type: "file" as const,
            isPinned: false,
            id: crypto.randomUUID(),
            timestamp: Date.now(),
            filePaths: files,
          };
          addItem(item);
          // Save to database asynchronously
          databaseService
            .saveItem(item)
            .catch((err) =>
              console.error("[CLIPBOARD] Failed to save files to DB:", err),
            );
        }
      });

      // onRTFUpdate, onImageBinaryUpdate, onSomethingUpdate can be added similarly if needed
      const unlistenRTF = await onRTFUpdate((rtf: string) => {
        console.info("[CLIPBOARD] RTF update received, ignoring for now", rtf);
      });

      const unlistenImageBinary = await onImageBinaryUpdate((data: any) => {
        console.info(
          "[CLIPBOARD] Image binary update received, ignoring for now",
          data,
        );
      });

      const unlistenSomething = await onSomethingUpdate((data: any) => {
        console.info(
          "[CLIPBOARD] Something update received, ignoring for now",
          data,
        );
      });

      // Start listening for clipboard events
      const unlistenClipboard = await startListening();

      // Store unlisteners for cleanup
      unlistenersRef.current = [
        unlistenText as () => void,
        unlistenImage as () => void,
        unlistenHtml as () => void,
        unlistenFiles as () => void,
        unlistenRTF as () => void,
        unlistenImageBinary as () => void,
        unlistenSomething as () => void,
        unlistenClipboard as () => void,
      ];

      await startMonitoring();
      console.info("[CLIPBOARD] Listeners set up successfully");
    } catch (error) {
      const errorMessage =
        error instanceof Error ? error.message : "Unknown error";
      setError(`Failed to set up clipboard monitoring: ${errorMessage}`);
      console.error("[CLIPBOARD] Error setting up listeners:", error);
    }
  }, [addItem, startMonitoring, setError]);

  /**
   * Clean up event listeners
   */
  const cleanup = useCallback(async () => {
    try {
      console.info("[CLIPBOARD] Cleaning up listeners...");
      // Call all unlisteners
      unlistenersRef.current.forEach((unlisten) => {
        try {
          unlisten();
        } catch (err) {
          console.error("[CLIPBOARD] Unlisten error:", err);
        }
      });
      unlistenersRef.current = [];
      await stopMonitoring();
      console.info("[CLIPBOARD] Listeners cleaned up successfully");
    } catch (error) {
      console.error(
        "[CLIPBOARD] Error during clipboard listener cleanup:",
        error,
      );
    }
  }, [stopMonitoring]);

  onClipboardUpdate(() => {
    console.info("[CLIPBOARD] Clipboard updated");
  });

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
