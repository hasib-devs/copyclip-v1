import Navigation from "@/components/Navigation";
import { TooltipProvider } from "@/components/ui/tooltip";
import { cn } from "@/lib/utils";
import Emoji from "@/pages/emoji";
import Home from "@/pages/home";
import Settings from "@/pages/settings";
import Tasks from "@/pages/tasks";
import Statistics from "@/pages/statistics";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { Minus, X } from "lucide-react";
import { BrowserRouter, Route, Routes } from "react-router";
import "./App.css";

function App() {
  const appWindow = getCurrentWindow();

  const handleMinimize = async () => {
    await appWindow.minimize();
  };

  const handleClose = async () => {
    await appWindow.close();
  };

  return (
    <TooltipProvider>
      <BrowserRouter>
        <div className="flex flex-col h-screen bg-white text-slate-900 overflow-hidden">
          {/* Custom Window Title Bar */}
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
              <span className="text-xs font-semibold text-slate-900">
                Copyclip
              </span>
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

          {/* Navigation Bar */}
          <Navigation />

          {/* Main Content Area */}
          <main className="flex-1 overflow-hidden bg-white">
            <Routes>
              <Route path="/" element={<Home />} />
              <Route path="/emoji" element={<Emoji />} />
              <Route path="/tasks" element={<Tasks />} />
              <Route path="/stats" element={<Statistics />} />
              <Route path="/settings" element={<Settings />} />
            </Routes>
          </main>
        </div>
      </BrowserRouter>
    </TooltipProvider>
  );
}

export default App;
