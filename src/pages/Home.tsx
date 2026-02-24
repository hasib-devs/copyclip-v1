import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { useClipboard } from "@/hooks/useClipboard";
import { cn } from "@/lib/utils";
import { databaseService } from "@/services/databaseService";
import { ItemType } from "@/types/clipboard.types";
import { confirm } from "@tauri-apps/plugin-dialog";
import {
  Activity,
  AlertCircle,
  CheckIcon,
  Copy,
  Pin,
  PinOff,
  Search,
  Trash2,
} from "lucide-react";
import { useState } from "react";

const Home = () => {
  // Search query state
  const [searchQuery, setSearchQuery] = useState("");
  // Track which item is being copied (for disabling button)
  const [copyingId, setCopyingId] = useState<string | null>(null);
  // Track recently copied item for feedback
  const [copiedId, setCopiedId] = useState<string | null>(null);

  // Get clipboard context
  const {
    items,
    isMonitoring,
    error,
    pausedCopying,
    removeItem,
    togglePin,
    copyToClipboard,
    searchItems,
    clearHistory,
    setError,
    stopMonitoring,
    startMonitoring,
    setPausedCopying,
  } = useClipboard();

  // Get displayed items
  const displayedItems = searchQuery.trim() ? searchItems(searchQuery) : items;

  // Handle copy action
  const handleCopy = async (
    content: string,
    type: ItemType["type"] = "text",
    itemId: string,
  ) => {
    try {
      // Prevent multiple clicks - set copying state
      setCopyingId(itemId);
      setPausedCopying(true); // Pause monitoring to prevent loops

      // Perform the copy
      await copyToClipboard(content, type);

      // Auto-clear feedback after 2 seconds
      setTimeout(() => {
        setCopiedId(null);
      }, 2500);
    } catch (err) {
      setError("Failed to copy to clipboard");
    } finally {
      // Clear loading state
      setCopyingId(null);
    }
  };

  // Handle delete item with database persistence
  const handleDelete = async (id: string) => {
    try {
      const confirmed = await confirm(
        "Are you sure you want to delete this item?",
        "Delete Item",
      );
      if (!confirmed) return;

      removeItem(id);
      const success = await databaseService.deleteItem(id);
      if (!success) {
        setError("Failed to delete item from database");
      }
    } catch (error) {
      console.error("Delete error:", error);
      setError("Failed to delete item from database");
    }
  };

  // Handle pin toggle with database persistence
  const handleTogglePin = async (id: string, isPinned: boolean) => {
    togglePin(id);
    await databaseService.updatePined(id, !isPinned);
  };

  // Handle clear all
  const handleClearAll = async () => {
    const confirmed = await confirm(
      "Are you sure you want to clear all clipboard history?",
      "Clear Clipboard",
    );
    if (confirmed) {
      clearHistory();
      databaseService.clearAll();
    }
  };

  return (
    <div className="flex flex-col h-full bg-white">
      {/* Header Section */}
      <div className="border-b border-slate-200 p-4">
        <div className="flex gap-3 mb-4">
          <div className="w-8 h-8 rounded-lg bg-blue-100 flex items-center justify-center text-lg">
            📋
          </div>
          <div className="flex-1">
            <h1 className="text-2xl font-bold text-slate-900">
              Clipboard History
            </h1>
            <div className="flex items-center gap-2 mt-1">
              <div
                className={cn(
                  "w-2 h-2 rounded-full",
                  isMonitoring ? "bg-green-500" : "bg-slate-300",
                )}
              />
              <span className="text-xs w-22 text-slate-600">
                {isMonitoring ? "Monitoring" : "Not monitoring"}
              </span>
              <span className="text-xs w-22 text-slate-600">
                {pausedCopying ? " (Paused)" : "Neutral"}
              </span>
              <span className="text-xs text-slate-500 ml-2">
                {displayedItems.length} items
              </span>
            </div>
          </div>

          <Button
            variant="secondary"
            size="sm"
            onClick={isMonitoring ? stopMonitoring : startMonitoring}
            className="text-slate-600 hover:text-red-600"
          >
            {isMonitoring ? "Stop" : "Start"}
          </Button>
        </div>

        <div className="flex gap-3 w-full">
          {/* Search Bar */}
          <div className="flex gap-2 flex-1">
            <div className="flex-1 relative">
              <Search
                className="absolute left-3 top-2.5 text-slate-400"
                size={18}
              />
              <Input
                placeholder="Search clipboard history..."
                value={searchQuery}
                onChange={(e) => setSearchQuery(e.target.value)}
                className={cn(
                  "pl-10 bg-slate-50 border-slate-200",
                  "focus:bg-white focus:border-blue-300",
                )}
              />
            </div>
          </div>

          {/* Clear button */}
          {displayedItems.length > 0 && (
            <Button
              variant="outline"
              size="sm"
              onClick={handleClearAll}
              className="text-slate-600 hover:text-red-600 ml-auto"
            >
              Clear
            </Button>
          )}
        </div>

        {/* Error Message */}
        {error && (
          <div className="mt-3 flex items-center gap-2 p-2 bg-red-50 border border-red-200 rounded text-sm text-red-700">
            <AlertCircle size={16} />
            <span>{error}</span>
          </div>
        )}
      </div>

      {/* Content Area */}
      <div className="flex-1 overflow-y-auto bg-white">
        {displayedItems.length === 0 ? (
          <div className="flex items-center justify-center h-full">
            <div className="text-center">
              <div className="w-16 h-16 mx-auto mb-4 flex items-center justify-center text-4xl">
                {displayedItems.length === 0 ? "📋" : "🔍"}
              </div>
              <p className="text-slate-500 text-sm">
                {displayedItems.length === 0
                  ? "Copy something to start tracking your clipboard"
                  : "No items match your search"}
              </p>
              {displayedItems.length === 0 && (
                <div className="mt-2 flex items-center justify-center gap-2 text-xs text-slate-400">
                  <Activity size={14} />
                  <span>Listening for clipboard changes...</span>
                </div>
              )}
            </div>
          </div>
        ) : (
          <div className="divide-y divide-slate-200">
            {displayedItems.map((item) => (
              <div
                key={item.id}
                className="hover:bg-slate-50 transition-colors p-4"
              >
                {/* Item Content */}
                <div className="mb-3">
                  {item.type === "image_base64" && item.imageBase64 ? (
                    <div className="flex items-center gap-2">
                      <img
                        src={`data:image/png;base64,${item.imageBase64}`}
                        alt="clipboard content"
                        className="object-cover rounded"
                      />
                    </div>
                  ) : (
                    <p
                      className={cn(
                        "text-sm text-slate-700 bg-zinc-50 p-2 rounded",
                        "max-h-20 overflow-hidden whitespace-pre-wrap",
                        item.type === "html" ? "font-mono text-xs" : "",
                      )}
                    >
                      {item.content}
                    </p>
                  )}
                </div>

                {/* Action Buttons */}
                <div className="flex items-center justify-end gap-2">
                  <Button
                    variant="outline"
                    size="sm"
                    onClick={() => handleCopy(item.content, item.type, item.id)}
                    disabled={!!copyingId}
                    className={cn(
                      copiedId === item.id
                        ? "text-green-600 border-green-200 bg-green-50"
                        : "text-blue-600 border-blue-200 hover:bg-blue-50",
                    )}
                  >
                    {copiedId === item.id ? (
                      <CheckIcon size={16} />
                    ) : (
                      <Copy size={16} className="mr-1" />
                    )}
                    {copiedId === item.id ? "Copied!" : "Copy"}
                  </Button>

                  <Button
                    variant="ghost"
                    size="sm"
                    onClick={() => handleTogglePin(item.id, item.isPinned)}
                    className={cn(
                      "h-8 w-8 p-0",
                      item.isPinned
                        ? "text-blue-600 hover:bg-blue-50"
                        : "text-slate-400 hover:text-slate-600",
                    )}
                    title={item.isPinned ? "Unpin" : "Pin"}
                  >
                    {item.isPinned ? <Pin size={16} /> : <PinOff size={16} />}
                  </Button>

                  <Button
                    variant="outline"
                    size="sm"
                    onClick={() => handleDelete(item.id)}
                    className="text-slate-600 hover:text-red-600 hover:bg-red-50"
                    disabled={item.isPinned}
                    title={item.isPinned ? "Unpin to delete" : "Delete"}
                  >
                    <Trash2 size={16} />
                  </Button>
                </div>
              </div>
            ))}
          </div>
        )}
      </div>
    </div>
  );
};

export default Home;
