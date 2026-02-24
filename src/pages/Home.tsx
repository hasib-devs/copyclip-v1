import { useEffect, useState } from "react";
import {
  Search,
  Copy,
  Trash2,
  Pin,
  PinOff,
  AlertCircle,
  Activity,
} from "lucide-react";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import { cn } from "@/lib/utils";
import { useClipboard } from "@/hooks/useClipboard";
import { databaseService } from "@/services/databaseService";
import { ItemType } from "@/types/clipboard";

const Home = () => {
  const [searchQuery, setSearchQuery] = useState("");
  const [displayPage, setDisplayPage] = useState(0);
  const pageSize = 10;

  // Get clipboard context
  const {
    items,
    isMonitoring,
    error,
    removeItem,
    togglePin,
    copyToClipboard,
    searchItems,
    clearHistory,
    setError,
  } = useClipboard();

  // Get displayed items
  const displayedItems = searchQuery.trim() ? searchItems(searchQuery) : items;
  const paginatedItems = displayedItems.slice(
    displayPage * pageSize,
    (displayPage + 1) * pageSize,
  );

  console.log({ paginatedItems });

  const totalPages = Math.ceil(displayedItems.length / pageSize);

  // Handle copy action
  const handleCopy = async (
    content: string,
    type: ItemType["type"] = "text",
  ) => {
    try {
      await copyToClipboard(content, type);
    } catch (err) {
      setError("Failed to copy to clipboard");
    }
  };

  // Handle delete item with database persistence
  const handleDelete = async (id: string) => {
    try {
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
    await databaseService.updateItem(id, !isPinned);
  };

  // Handle clear all
  const handleClearAll = () => {
    if (
      window.confirm("Are you sure you want to clear all clipboard history?")
    ) {
      clearHistory();
      databaseService.clearAll();
      setDisplayPage(0);
    }
  };

  // Reset page when search changes
  useEffect(() => {
    setDisplayPage(0);
  }, [searchQuery]);

  return (
    <div className="flex flex-col h-full bg-white">
      {/* Header Section */}
      <div className="border-b border-slate-200 p-4">
        <div className="flex items-center gap-3 mb-4">
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
              <span className="text-xs text-slate-600">
                {isMonitoring ? "Monitoring" : "Not monitoring"}
              </span>
              <span className="text-xs text-slate-500 ml-2">
                {items.length} items
              </span>
            </div>
          </div>
          {items.length > 0 && (
            <Button
              variant="outline"
              size="sm"
              onClick={handleClearAll}
              className="text-slate-600 hover:text-red-600"
            >
              Clear
            </Button>
          )}
        </div>

        {/* Search Bar */}
        <div className="flex gap-2">
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
        {paginatedItems.length === 0 ? (
          <div className="flex items-center justify-center h-full">
            <div className="text-center">
              <div className="w-16 h-16 mx-auto mb-4 flex items-center justify-center text-4xl">
                {items.length === 0 ? "📋" : "🔍"}
              </div>
              <p className="text-slate-500 text-sm">
                {items.length === 0
                  ? "Copy something to start tracking your clipboard"
                  : "No items match your search"}
              </p>
              {items.length === 0 && (
                <div className="mt-2 flex items-center justify-center gap-2 text-xs text-slate-400">
                  <Activity size={14} />
                  <span>Listening for clipboard changes...</span>
                </div>
              )}
            </div>
          </div>
        ) : (
          <div className="divide-y divide-slate-200">
            {paginatedItems.map((item) => (
              <div
                key={item.id}
                className="hover:bg-slate-50 transition-colors p-4"
              >
                {/* Item Header */}
                <div className="flex items-start justify-between mb-2">
                  <div className="flex items-center gap-2 flex-1 min-w-0">
                    <span className="text-xs font-medium text-slate-500 bg-slate-100 px-2 py-1 rounded whitespace-nowrap">
                      {item.type}
                    </span>
                    <span className="text-xs text-slate-400">
                      {new Date(item.timestamp).toLocaleTimeString()}
                    </span>
                  </div>
                  <div className="flex items-center gap-1 ml-2">
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
                  </div>
                </div>

                {/* Item Content */}
                <div className="mb-3">
                  {item.type === "image" && item.imageBase64 ? (
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
                        "text-sm text-slate-700 whitespace-pre-wrap",
                        "max-h-20 overflow-hidden",
                        item.type === "html" ? "font-mono text-xs" : "",
                      )}
                    >
                      {item.content.substring(0, 200)}
                      {item.content.length > 200 && "..."}
                    </p>
                  )}
                </div>

                {/* Action Buttons */}
                <div className="flex items-center justify-end gap-2">
                  <Button
                    variant="outline"
                    size="sm"
                    onClick={() => handleCopy(item.content, item.type)}
                    className="text-blue-600 border-blue-200 hover:bg-blue-50"
                  >
                    <Copy size={16} className="mr-1" />
                    Copy
                  </Button>
                  <Button
                    variant="outline"
                    size="sm"
                    onClick={() => handleDelete(item.id)}
                    className="text-slate-600 hover:text-red-600 hover:bg-red-50"
                  >
                    <Trash2 size={16} />
                  </Button>
                </div>
              </div>
            ))}
          </div>
        )}
      </div>

      {/* Pagination */}
      {totalPages > 1 && (
        <div className="border-t border-slate-200 p-4 flex items-center justify-between bg-slate-50">
          <span className="text-xs text-slate-600">
            Page {displayPage + 1} of {totalPages}
          </span>
          <div className="flex gap-2">
            <Button
              variant="outline"
              size="sm"
              onClick={() => setDisplayPage(Math.max(0, displayPage - 1))}
              disabled={displayPage === 0}
            >
              Previous
            </Button>
            <Button
              variant="outline"
              size="sm"
              onClick={() =>
                setDisplayPage(Math.min(totalPages - 1, displayPage + 1))
              }
              disabled={displayPage === totalPages - 1}
            >
              Next
            </Button>
          </div>
        </div>
      )}
    </div>
  );
};

export default Home;
