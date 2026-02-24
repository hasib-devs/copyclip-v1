import Navigation from "@/components/Navigation";
import { TooltipProvider } from "@/components/ui/tooltip";
import { cn } from "@/lib/utils";
import Emoji from "@/pages/emoji";
import Home from "@/pages/home";
import Settings from "@/pages/settings";
import Snippets from "@/pages/snippets";
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
        <div className="flex flex-col h-screen bg-background text-foreground overflow-hidden">
          {/* Custom Window Title Bar */}
          <div
            data-tauri-drag-region
            className={cn(
              "flex items-center justify-between",
              "h-12 px-4",
              "bg-slate-900 border-b border-slate-800",
              "select-none",
            )}
          >
            <div className="flex items-center gap-2">
              <div
                className={cn(
                  "w-5 h-5 rounded-full",
                  "bg-linear-to-br from-blue-500 to-cyan-500",
                  "flex items-center justify-center",
                )}
              >
                <span className="text-xs font-bold text-white">C</span>
              </div>
              <span className="text-sm font-semibold text-white">Copyclip</span>
            </div>

            {/* Window Control Buttons */}
            <div className="flex items-center gap-2">
              <button
                onClick={handleMinimize}
                className={cn(
                  "p-1 rounded transition-colors",
                  "hover:bg-slate-800",
                )}
                title="Minimize"
              >
                <Minus size={16} className="text-slate-300" />
              </button>
              <button
                onClick={handleClose}
                className={cn(
                  "p-1 rounded transition-colors",
                  "hover:bg-red-500",
                )}
                title="Close"
              >
                <X size={16} className="text-slate-300 hover:text-white" />
              </button>
            </div>
          </div>

          {/* Main Content Area */}
          <main className="flex-1 overflow-auto">
            <Routes>
              <Route path="/" element={<Home />} />
              <Route path="/emoji" element={<Emoji />} />
              <Route path="/snippets" element={<Snippets />} />
              <Route path="/stats" element={<Statistics />} />
              <Route path="/settings" element={<Settings />} />
            </Routes>
          </main>

          {/* Navigation Footer */}
          <Navigation />
        </div>
      </BrowserRouter>
    </TooltipProvider>
  );
}

export default App;
