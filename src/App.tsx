import Navigation from "@/components/Navigation";
import { TooltipProvider } from "@/components/ui/tooltip";
import { useClipboardMonitor } from "@/hooks/useClipboardMonitor";
import { useLoadClipboardHistory } from "@/hooks/useLoadClipboardHistory";
import Emoji from "@/pages/emoji";
import Home from "@/pages/home";
import Settings from "@/pages/settings";
import Snippets from "@/pages/snippets";
import Statistics from "@/pages/statistics";
import { BrowserRouter, Route, Routes } from "react-router";
import "./App.css";
import { useGamepadContext } from "./contexts/gamepad-context";
import { Toaster } from "./components/ui/sonner";
import {
  useGamepadMonitor,
  useLoadGamepadProfiles,
} from "./hooks/useGamepadMonitor";

function App() {
  const { state, dispatch } = useGamepadContext();

  // Monitor gamepad input when listening
  useGamepadMonitor(state.isListening, dispatch);

  // Load profiles on mount
  useLoadGamepadProfiles(dispatch);

  // Load initial clipboard history from database
  useLoadClipboardHistory();

  // Set up clipboard monitoring
  useClipboardMonitor(true);

  return (
    <TooltipProvider>
      <Toaster />

      <BrowserRouter>
        <div className="flex flex-col h-screen bg-white text-slate-900">
          {/* Navigation Bar */}
          <Navigation />

          {/* Main Content Area */}
          <main className="flex-1 overflow-auto bg-white h-[calc(100vh-57px)]">
            <Routes>
              <Route path="/" element={<Home />} />
              <Route path="/emoji" element={<Emoji />} />
              <Route path="/snippets" element={<Snippets />} />
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
