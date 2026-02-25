import React, { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Card, CardContent } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Label } from "@/components/ui/label";
import { Alert, AlertDescription } from "@/components/ui/alert";
import { AlertCircle, Zap } from "lucide-react";

interface GamepadSettings {
  sensitivity: number;
  deadZone: number;
  acceleration: number;
  scrollVerticalSpeed: number;
  scrollHorizontalSpeed: number;
  reverseScrollVertical: boolean;
  reverseScrollHorizontal: boolean;
  vibrationEnabled: boolean;
}

const DEFAULT_SETTINGS: GamepadSettings = {
  sensitivity: 1.5,
  deadZone: 0.1,
  acceleration: 1.0,
  scrollVerticalSpeed: 1.0,
  scrollHorizontalSpeed: 1.0,
  reverseScrollVertical: false,
  reverseScrollHorizontal: false,
  vibrationEnabled: true,
};

const GamepadSettingsPanel: React.FC = () => {
  const [settings, setSettings] = useState<GamepadSettings>(DEFAULT_SETTINGS);
  const [hasChanges, setHasChanges] = useState(false);
  const [isSaving, setIsSaving] = useState(false);
  const [message, setMessage] = useState<{
    type: "success" | "error";
    text: string;
  } | null>(null);

  // Load settings on mount
  useEffect(() => {
    const loadSettings = async () => {
      try {
        const settingsJson = await invoke<GamepadSettings>(
          "get_gamepad_settings",
        );
        setSettings(settingsJson);
      } catch (err) {
        console.error("Failed to load settings:", err);
        setMessage({ type: "error", text: "Failed to load settings" });
      }
    };

    loadSettings();
  }, []);

  const handleSettingChange = (
    key: keyof GamepadSettings,
    value: number | boolean,
  ) => {
    setSettings((prev) => ({ ...prev, [key]: value }));
    setHasChanges(true);
    setMessage(null);
  };

  const handleSave = async () => {
    try {
      setIsSaving(true);
      await invoke("save_gamepad_settings", { settings });
      setHasChanges(false);
      setMessage({ type: "success", text: "Settings saved successfully!" });
      setTimeout(() => setMessage(null), 3000);
    } catch (err) {
      console.error("Failed to save settings:", err);
      setMessage({ type: "error", text: "Failed to save settings" });
    } finally {
      setIsSaving(false);
    }
  };

  const handleReset = () => {
    setSettings(DEFAULT_SETTINGS);
    setHasChanges(false);
    setMessage(null);
  };

  const SliderControl = ({
    label,
    value,
    onChange,
    min,
    max,
    step,
    description,
  }: {
    label: string;
    value: number;
    onChange: (v: number) => void;
    min: number;
    max: number;
    step: number;
    description: string;
  }) => (
    <div className="space-y-2">
      <div className="flex items-center justify-between">
        <Label className="text-sm font-medium">{label}</Label>
        <Badge variant="outline" className="text-xs">
          {value.toFixed(2)}
        </Badge>
      </div>
      <input
        type="range"
        min={min}
        max={max}
        step={step}
        value={value}
        onChange={(e) => onChange(parseFloat(e.target.value))}
        className="w-full h-2 bg-slate-200 rounded-lg appearance-none cursor-pointer"
      />
      <p className="text-xs text-gray-500">{description}</p>
    </div>
  );

  const ToggleControl = ({
    label,
    checked,
    onChange,
    description,
  }: {
    label: string;
    checked: boolean;
    onChange: (v: boolean) => void;
    description: string;
  }) => (
    <div className="space-y-2 p-3 bg-slate-50 rounded-lg border border-slate-200">
      <div className="flex items-center justify-between">
        <Label className="text-sm font-medium cursor-pointer">{label}</Label>
        <input
          type="checkbox"
          checked={checked}
          onChange={(e) => onChange(e.target.checked)}
          className="w-4 h-4 rounded cursor-pointer"
        />
      </div>
      <p className="text-xs text-gray-500">{description}</p>
    </div>
  );

  return (
    <div className="space-y-6 p-6 max-w-2xl">
      <div>
        <h2 className="text-2xl font-bold mb-2">Gamepad Settings</h2>
        <p className="text-gray-600">
          Fine-tune your gamepad input behavior and responsiveness
        </p>
      </div>

      {message && (
        <Alert variant={message.type === "error" ? "destructive" : "default"}>
          <AlertCircle className="h-4 w-4" />
          <AlertDescription>{message.text}</AlertDescription>
        </Alert>
      )}

      <div className="space-y-8">
        {/* Mouse Control Settings */}
        <Card>
          <CardContent className="pt-6">
            <h3 className="text-lg font-semibold mb-4 flex items-center gap-2">
              <span>🖱️</span> Mouse Control
            </h3>
            <div className="space-y-4">
              <SliderControl
                label="Sensitivity"
                value={settings.sensitivity}
                onChange={(v) => handleSettingChange("sensitivity", v)}
                min={0.5}
                max={3.0}
                step={0.1}
                description="Adjust cursor movement speed (0.5x - 3.0x)"
              />
              <SliderControl
                label="Dead Zone"
                value={settings.deadZone}
                onChange={(v) => handleSettingChange("deadZone", v)}
                min={0.0}
                max={0.3}
                step={0.01}
                description="Minimum stick movement threshold (prevents drift)"
              />
              <SliderControl
                label="Acceleration"
                value={settings.acceleration}
                onChange={(v) => handleSettingChange("acceleration", v)}
                min={0.8}
                max={2.0}
                step={0.1}
                description="Increase speed as you move stick further (smoothness)"
              />
            </div>
          </CardContent>
        </Card>

        {/* Scroll Settings */}
        <Card>
          <CardContent className="pt-6">
            <h3 className="text-lg font-semibold mb-4 flex items-center gap-2">
              <span>📜</span> Scroll Control
            </h3>
            <div className="space-y-4">
              <SliderControl
                label="Vertical Speed"
                value={settings.scrollVerticalSpeed}
                onChange={(v) => handleSettingChange("scrollVerticalSpeed", v)}
                min={0.5}
                max={5.0}
                step={0.5}
                description="Adjust vertical scroll sensitivity"
              />
              <SliderControl
                label="Horizontal Speed"
                value={settings.scrollHorizontalSpeed}
                onChange={(v) =>
                  handleSettingChange("scrollHorizontalSpeed", v)
                }
                min={0.5}
                max={5.0}
                step={0.5}
                description="Adjust horizontal scroll sensitivity"
              />
              <ToggleControl
                label="Reverse Vertical"
                checked={settings.reverseScrollVertical}
                onChange={(v) =>
                  handleSettingChange("reverseScrollVertical", v)
                }
                description="Invert vertical scroll direction"
              />
              <ToggleControl
                label="Reverse Horizontal"
                checked={settings.reverseScrollHorizontal}
                onChange={(v) =>
                  handleSettingChange("reverseScrollHorizontal", v)
                }
                description="Invert horizontal scroll direction"
              />
            </div>
          </CardContent>
        </Card>

        {/* Advanced Settings */}
        <Card>
          <CardContent className="pt-6">
            <h3 className="text-lg font-semibold mb-4 flex items-center gap-2">
              <Zap className="w-5 h-5" /> Advanced
            </h3>
            <div className="space-y-4">
              <ToggleControl
                label="Vibration Feedback"
                checked={settings.vibrationEnabled}
                onChange={(v) => handleSettingChange("vibrationEnabled", v)}
                description="Enable rumble/haptic feedback on supported controllers"
              />
            </div>
          </CardContent>
        </Card>
      </div>

      {/* Action Buttons */}
      <div className="flex gap-2 justify-end pt-4 border-t">
        <Button
          variant="outline"
          onClick={handleReset}
          disabled={!hasChanges || isSaving}
        >
          Reset to Defaults
        </Button>
        <Button
          onClick={handleSave}
          disabled={!hasChanges || isSaving}
          className="bg-blue-600 hover:bg-blue-700"
        >
          {isSaving ? "Saving..." : "Save Settings"}
        </Button>
      </div>
    </div>
  );
};

export default GamepadSettingsPanel;
