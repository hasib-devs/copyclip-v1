import "./App.css";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { X, Minus } from "lucide-react";

function App() {
  const appWindow = getCurrentWindow();

  const handleMinimize = async () => {
    await appWindow.minimize();
  };

  const handleClose = async () => {
    await appWindow.close();
  };

  return (
    <div className="flex flex-col h-screen bg-background text-foreground overflow-hidden">
      {/* Custom Window Title Bar */}
      <div
        data-tauri-drag-region
        className="flex items-center justify-between h-12 px-4 bg-slate-900 border-b border-slate-800 select-none"
      >
        <div className="flex items-center gap-2">
          <div className="w-5 h-5 rounded-full bg-linear-to-br from-blue-500 to-cyan-500 flex items-center justify-center">
            <span className="text-xs font-bold text-white">C</span>
          </div>
          <span className="text-sm font-semibold text-white">Copyclip</span>
        </div>

        {/* Window Control Buttons */}
        <div className="flex items-center gap-2">
          <button
            onClick={handleMinimize}
            className="p-1 hover:bg-slate-800 rounded transition-colors"
            title="Minimize"
          >
            <Minus size={16} className="text-slate-300" />
          </button>
          <button
            onClick={handleClose}
            className="p-1 hover:bg-red-500 rounded transition-colors"
            title="Close"
          >
            <X size={16} className="text-slate-300 hover:text-white" />
          </button>
        </div>
      </div>

      {/* Main Content Area */}
      <main className="flex-1 overflow-auto pb-safe">
        <div className="h-full flex items-center justify-center">
          <div className="text-center">
            <h1 className="text-2xl font-bold mb-2">Welcome to Copyclip</h1>
            <p className="text-slate-400">Your clipboard history manager</p>
          </div>
        </div>
      </main>
    </div>
  );
}

export default App;
