import { useState } from "react";
import { Switch } from "@/components/ui/switch";
import { Label } from "@/components/ui/label";
import {
  Accordion,
  AccordionContent,
  AccordionItem,
  AccordionTrigger,
} from "@/components/ui/accordion";
import { ControllerConfig } from "@/components/ControllerConfig";
import { cn } from "@/lib/utils";

const Settings = () => {
  const [theme, setTheme] = useState("system");
  const [startWithSystem, setStartWithSystem] = useState(false);
  const [minimizeToTray, setMinimizeToTray] = useState(true);
  const [showNotifications, setShowNotifications] = useState(true);

  return (
    <div className="flex flex-col h-full bg-white">
      {/* Header */}
      <div className="border-b border-slate-200 p-4">
        <div className="flex items-center gap-3">
          <div className="w-8 h-8 rounded-lg bg-slate-200 flex items-center justify-center text-lg">
            ⚙️
          </div>
          <div>
            <h1 className="text-2xl font-bold text-slate-900">Settings</h1>
            <p className="text-sm text-slate-500">
              Customize your clipboard manager
            </p>
          </div>
        </div>
      </div>

      {/* Settings Sections */}
      <div className="flex-1 overflow-y-auto">
        <Accordion
          type="single"
          collapsible
          defaultValue="general"
          className="w-full"
        >
          {/* General Section */}
          <AccordionItem value="general" className="border-b border-slate-200">
            <AccordionTrigger className="px-4 py-3 hover:bg-slate-50">
              <div className="flex items-center gap-3">
                <span>⚙️</span>
                <span className="font-medium text-slate-900">General</span>
              </div>
            </AccordionTrigger>
            <AccordionContent className="px-4 py-4 bg-slate-50">
              <div className="space-y-4">
                {/* Theme */}
                <div>
                  <Label className="block text-sm font-medium text-slate-900 mb-2">
                    Theme
                  </Label>
                  <div className="flex gap-2">
                    {["Light", "Dark", "System"].map((t) => (
                      <button
                        key={t}
                        onClick={() => setTheme(t.toLowerCase())}
                        className={cn(
                          "px-3 py-1.5 rounded text-sm transition-colors",
                          theme === t.toLowerCase()
                            ? "bg-slate-900 text-white"
                            : "bg-white border border-slate-200 text-slate-600 hover:border-slate-300",
                        )}
                      >
                        {t === "Light" && "☀️"} {t === "Dark" && "🌙"}{" "}
                        {t === "System" && "⚙️"} {t}
                      </button>
                    ))}
                  </div>
                </div>

                {/* Toggle Settings */}
                <div className="space-y-3 pt-4 border-t border-slate-200">
                  <div className="flex items-center justify-between">
                    <div>
                      <Label className="text-sm font-medium text-slate-900">
                        Start with system
                      </Label>
                      <p className="text-xs text-slate-500 mt-0.5">
                        Launch automatically when you log in
                      </p>
                    </div>
                    <Switch
                      checked={startWithSystem}
                      onCheckedChange={setStartWithSystem}
                    />
                  </div>

                  <div className="flex items-center justify-between">
                    <div>
                      <Label className="text-sm font-medium text-slate-900">
                        Minimize to tray
                      </Label>
                      <p className="text-xs text-slate-500 mt-0.5">
                        Keep running in the background when closed
                      </p>
                    </div>
                    <Switch
                      checked={minimizeToTray}
                      onCheckedChange={setMinimizeToTray}
                    />
                  </div>

                  <div className="flex items-center justify-between">
                    <div>
                      <Label className="text-sm font-medium text-slate-900">
                        Show notifications
                      </Label>
                      <p className="text-xs text-slate-500 mt-0.5">
                        Display notifications for important events
                      </p>
                    </div>
                    <Switch
                      checked={showNotifications}
                      onCheckedChange={setShowNotifications}
                    />
                  </div>
                </div>
              </div>
            </AccordionContent>
          </AccordionItem>

          {/* Clipboard Section */}
          <AccordionItem
            value="clipboard"
            className="border-b border-slate-200"
          >
            <AccordionTrigger className="px-4 py-3 hover:bg-slate-50">
              <div className="flex items-center gap-3">
                <span>📋</span>
                <span className="font-medium text-slate-900">Clipboard</span>
              </div>
            </AccordionTrigger>
            <AccordionContent className="px-4 py-4 bg-slate-50">
              <div className="space-y-3 text-sm text-slate-600">
                <p>Clipboard settings coming soon...</p>
              </div>
            </AccordionContent>
          </AccordionItem>

          {/* Keyboard Shortcuts Section */}
          <AccordionItem
            value="shortcuts"
            className="border-b border-slate-200"
          >
            <AccordionTrigger className="px-4 py-3 hover:bg-slate-50">
              <div className="flex items-center gap-3">
                <span>⌨️</span>
                <span className="font-medium text-slate-900">
                  Keyboard Shortcuts
                </span>
              </div>
            </AccordionTrigger>
            <AccordionContent className="px-4 py-4 bg-slate-50">
              <div className="space-y-3 text-sm text-slate-600">
                <p>Keyboard shortcuts configuration coming soon...</p>
              </div>
            </AccordionContent>
          </AccordionItem>

          {/* PS5 Controller Section */}
          <AccordionItem
            value="controller"
            className="border-b border-slate-200"
          >
            <AccordionTrigger className="px-4 py-3 hover:bg-slate-50">
              <div className="flex items-center gap-3">
                <span>🎮</span>
                <span className="font-medium text-slate-900">
                  PS5 Controller
                </span>
              </div>
            </AccordionTrigger>
            <AccordionContent className="px-4 py-4 bg-slate-50">
              <ControllerConfig />
            </AccordionContent>
          </AccordionItem>
        </Accordion>
      </div>
    </div>
  );
};

export default Settings;
