import React, { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Badge } from "@/components/ui/badge";

type GamepadMode = "Normal" | "Motion" | "Hotkey" | "Unknown";

const ModeIndicator: React.FC = () => {
  const [currentMode, setCurrentMode] = useState<GamepadMode>("Unknown");
  const [isLoading, setIsLoading] = useState(true);

  // Fetch current mode on mount
  useEffect(() => {
    const fetchMode = async () => {
      try {
        setIsLoading(true);
        const mode = await invoke<string>("get_gamepad_mode");
        // Parse the debug format from backend
        const parsedMode = mode.includes("Normal")
          ? "Normal"
          : mode.includes("Motion")
            ? "Motion"
            : mode.includes("Hotkey")
              ? "Hotkey"
              : "Unknown";
        setCurrentMode(parsedMode as GamepadMode);
      } catch (err) {
        console.error("Failed to fetch gamepad mode:", err);
        setCurrentMode("Unknown");
      } finally {
        setIsLoading(false);
      }
    };

    fetchMode();

    // Poll mode every 500ms for real-time updates
    const interval = setInterval(fetchMode, 500);
    return () => clearInterval(interval);
  }, []);

  const getModeColor = (mode: GamepadMode) => {
    switch (mode) {
      case "Normal":
        return "bg-blue-100 text-blue-800";
      case "Motion":
        return "bg-purple-100 text-purple-800";
      case "Hotkey":
        return "bg-green-100 text-green-800";
      default:
        return "bg-gray-100 text-gray-800";
    }
  };

  const getModeDescription = (mode: GamepadMode) => {
    switch (mode) {
      case "Normal":
        return "Normal mode - Mouse and scroll active";
      case "Motion":
        return "Motion mode - Motion control active";
      case "Hotkey":
        return "Hotkey mode - Custom hotkeys active";
      default:
        return "Mode unknown";
    }
  };

  return (
    <div className="flex items-center gap-2 px-3 py-2 rounded-lg bg-white border border-slate-200 transition-all duration-300">
      {isLoading ? (
        <div className="flex items-center gap-2">
          <div className="w-2 h-2 rounded-full bg-gray-400 animate-pulse" />
          <span className="text-sm text-gray-600">Loading...</span>
        </div>
      ) : (
        <>
          <Badge
            className={`${getModeColor(currentMode)} font-semibold`}
            variant="outline"
          >
            {currentMode}
          </Badge>
          <span className="text-xs text-gray-600 truncate max-w-sm">
            {getModeDescription(currentMode)}
          </span>
        </>
      )}
    </div>
  );
};

export default ModeIndicator;
