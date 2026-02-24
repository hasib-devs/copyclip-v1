import Navigation from "@/components/Navigation";
import { TooltipProvider } from "@/components/ui/tooltip";
import { ClipboardProvider } from "@/contexts/ClipboardContext";
import Emoji from "@/pages/emoji";
import Home from "@/pages/home";
import Settings from "@/pages/settings";
import Snippets from "@/pages/snippets";
import Statistics from "@/pages/statistics";
import { BrowserRouter, Route, Routes } from "react-router";
import "./App.css";

function App() {
  return (
    <ClipboardProvider>
      <TooltipProvider>
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
    </ClipboardProvider>
  );
}

export default App;
