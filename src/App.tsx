import { TooltipProvider } from "@/components/ui/tooltip";
import Settings from "@/pages/settings";
import { BrowserRouter, Route, Routes } from "react-router";
import "./App.css";
import { Toaster } from "./components/ui/sonner";

function App() {
  return (
    <TooltipProvider>
      <Toaster />

      <BrowserRouter>
        <div className="flex flex-col h-screen bg-white text-slate-900">
          {/* Main Content Area */}
          <main className="flex-1 overflow-auto bg-white h-[calc(100vh-57px)]">
            <Routes>
              <Route path="/" element={<Settings />} />
            </Routes>
          </main>
        </div>
      </BrowserRouter>
    </TooltipProvider>
  );
}

export default App;
