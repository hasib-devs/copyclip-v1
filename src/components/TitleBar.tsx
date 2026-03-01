import { cn } from "@/lib/utils";

import { Minus, X } from "lucide-react";
import { getCurrentWindow } from "@tauri-apps/api/window";

const TitleBar = () => {
  const appWindow = getCurrentWindow();
  const handleMinimize = async () => {
    await appWindow.minimize();
  };

  const handleClose = async () => {
    await appWindow.close();
  };

  return (
    <div
      data-tauri-drag-region
      className={cn(
        "flex items-center justify-between",
        "h-10 px-4",
        "bg-white border-b border-slate-200",
        "select-none",
      )}
    >
      <div className="flex items-center gap-2">
        <div
          className={cn(
            "w-4 h-4 rounded-sm",
            "bg-linear-to-br from-blue-600 to-blue-400",
            "flex items-center justify-center",
          )}
        >
          <span className="text-xs font-bold text-white">C</span>
        </div>
        <span className="text-xs font-semibold text-slate-900">YinVim</span>
      </div>

      {/* Window Control Buttons */}
      <div className="flex items-center gap-2">
        <button
          onClick={handleMinimize}
          className={cn(
            "p-1 rounded transition-colors",
            "hover:bg-slate-100 text-slate-600",
          )}
          title="Minimize"
        >
          <Minus size={14} />
        </button>
        <button
          onClick={handleClose}
          className={cn(
            "p-1 rounded transition-colors",
            "hover:bg-red-100 text-slate-600 hover:text-red-600",
          )}
          title="Close"
        >
          <X size={14} />
        </button>
      </div>
    </div>
  );
};

export default TitleBar;
