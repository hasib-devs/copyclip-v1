import { invoke } from "@tauri-apps/api/core";
import { AlertCircle, Gamepad2, Zap } from "lucide-react";
import { useEffect, useState } from "react";
import { Alert, AlertDescription } from "./ui/alert";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "./ui/card";
import { Label } from "./ui/label";
import { Slider } from "./ui/slider";
import { Switch } from "./ui/switch";

interface ControllerState {
  connected: boolean;
  left_stick_x: number;
  left_stick_y: number;
  left_trigger: number;
  right_trigger: number;
}

interface ControllerSettings {
  sensitivity: number;
  dead_zone: number;
  acceleration: number;
  enabled: boolean;
}

export function ControllerConfig() {
  const [settings, setSettings] = useState<ControllerSettings>({
    sensitivity: 1.0,
    dead_zone: 0.1,
    acceleration: 1.0,
    enabled: false,
  });

  const [state, _setState] = useState<ControllerState>({
    connected: false,
    left_stick_x: 0,
    left_stick_y: 0,
    left_trigger: 0,
    right_trigger: 0,
  });

  const [isListening, setIsListening] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const loadSettings = async () => {
    try {
      console.info("[ControllerConfig] Loading controller settings...");
      const loadedSettings = await invoke<ControllerSettings>(
        "get_controller_settings",
      );
      setSettings(loadedSettings);
    } catch (err) {
      console.error("Failed to load settings:", err);
      setError("Failed to load controller settings");
    }
  };

  const startControllerListener = async () => {
    try {
      console.info("[ControllerConfig] Starting controller listener...");
      setError(null);
      await invoke("start_controller");
      setIsListening(true);
    } catch (err) {
      const errorMsg = err instanceof Error ? err.message : String(err);
      setError(errorMsg);
      console.error("Failed to start controller:", err);
    }
  };

  const handleStartController = async () => {
    try {
      if (isListening) {
        console.warn("[ControllerConfig] Controller listener already active");
        await handleStopController();
        setError(null);
      }

      await startControllerListener();
    } catch (error) {
      console.error("Error starting controller listener:", error);
      setError("Failed to start controller listener");
    }
  };

  const handleStopController = async () => {
    try {
      console.info("[ControllerConfig] Stopping controller listener...");
      setError(null);
      await invoke("stop_controller");
      setIsListening(false);
    } catch (err) {
      const errorMsg = err instanceof Error ? err.message : String(err);
      setError(errorMsg);
      console.error("Failed to stop controller:", err);
    }
  };

  const handleToggleEnabled = async (enabled: boolean) => {
    try {
      console.info(
        `[ControllerConfig] Toggling controller enabled: ${enabled}`,
      );
      setError(null);
      const updatedSettings = { ...settings, enabled };
      await invoke("update_controller_settings", {
        sensitivity: updatedSettings.sensitivity,
        dead_zone: updatedSettings.dead_zone,
        acceleration: updatedSettings.acceleration,
        enabled: updatedSettings.enabled,
      });
      setSettings(updatedSettings);

      if (enabled && !isListening) {
        await handleStartController();
      } else if (!enabled && isListening) {
        await handleStopController();
      }
    } catch (err) {
      const errorMsg = err instanceof Error ? err.message : String(err);
      setError(errorMsg);
      console.error("Failed to toggle enabled:", err);
    }
  };

  /**
   * Update a specific controller setting and persist it to the backend
   * @param setting  The setting key to update (sensitivity, dead_zone, acceleration)
   * @param value   The new value for the setting
   */
  const updateSetting = async (
    setting: keyof ControllerSettings,
    value: number,
  ) => {
    try {
      console.info(
        `[ControllerConfig] Updating setting ${setting} to ${value}`,
      );
      setError(null);
      const updatedSettings = { ...settings, [setting]: value };
      await invoke("update_controller_settings", {
        sensitivity: updatedSettings.sensitivity,
        dead_zone: updatedSettings.dead_zone,
        acceleration: updatedSettings.acceleration,
        enabled: updatedSettings.enabled,
      });
      setSettings(updatedSettings);
    } catch (err) {
      const errorMsg = err instanceof Error ? err.message : String(err);
      setError(errorMsg);
      console.error("Failed to update setting:", err);
    }
  };

  // Load settings on mount
  useEffect(() => {
    loadSettings();
  }, []);

  // Start polling controller state when enabled
  useEffect(() => {
    if (!isListening) {
      handleStartController().catch((err) => {
        const errorMsg = err instanceof Error ? err.message : String(err);
        setError(errorMsg);
        console.error("Failed to start controller:", err);
      });
    }
  }, [isListening]);

  return (
    <div className="w-full max-w-2xl mx-auto p-4 space-y-6">
      <Card>
        <CardHeader>
          <div className="flex items-center gap-2">
            <Gamepad2 className="w-5 h-5" />
            <div>
              <CardTitle>PS5 Controller Configuration</CardTitle>
              <CardDescription>
                Control your mouse cursor with PS5 controller
              </CardDescription>
            </div>
          </div>
        </CardHeader>
        <CardContent className="space-y-6">
          {error && (
            <Alert variant="destructive">
              <AlertCircle className="h-4 w-4" />
              <AlertDescription>{error}</AlertDescription>
            </Alert>
          )}

          {/* Controller Status */}
          <div className="grid grid-cols-2 gap-4 p-4 bg-muted rounded-lg">
            <div>
              <Label className="text-sm text-muted-foreground">Status</Label>
              <p className="text-sm font-semibold">
                {state.connected ? (
                  <span className="text-green-600">🟢 Connected</span>
                ) : (
                  <span className="text-gray-500">⚪ Not Connected</span>
                )}
              </p>
            </div>
            <div>
              <Label className="text-sm text-muted-foreground">Listening</Label>
              <p className="text-sm font-semibold">
                {isListening ? (
                  <span className="text-blue-600">🔴 Active</span>
                ) : (
                  <span className="text-gray-500">⚫ Inactive</span>
                )}
              </p>
            </div>
          </div>

          {/* Enable/Disable Toggle */}
          <div className="flex items-center justify-between p-4 border rounded-lg">
            <div>
              <Label className="text-base">Enable Controller</Label>
              <p className="text-sm text-muted-foreground">
                Activate PS5 controller input
              </p>
            </div>
            <Switch
              checked={settings.enabled}
              onCheckedChange={handleToggleEnabled}
              disabled={!state.connected}
            />
          </div>

          {/* Real-time Stick Position */}
          {isListening && state.connected && (
            <div className="space-y-2">
              <Label>Left Stick Input</Label>
              <div className="grid grid-cols-2 gap-4 p-4 bg-muted rounded-lg text-sm">
                <div>X: {state.left_stick_x.toFixed(2)}</div>
                <div>Y: {state.left_stick_y.toFixed(2)}</div>
                <div>L-Trigger: {state.left_trigger.toFixed(2)}</div>
                <div>R-Trigger: {state.right_trigger.toFixed(2)}</div>
              </div>
            </div>
          )}

          {/* Sensitivity Slider */}
          <div className="space-y-2">
            <div className="flex justify-between items-center">
              <Label>Sensitivity</Label>
              <span className="text-sm font-semibold">
                {settings.sensitivity.toFixed(2)}x
              </span>
            </div>
            <Slider
              value={[settings.sensitivity]}
              onValueChange={(value) => updateSetting("sensitivity", value[0])}
              min={0.5}
              max={3.0}
              step={0.1}
              className="w-full"
            />
            <p className="text-xs text-muted-foreground">
              Higher values = faster cursor movement
            </p>
          </div>

          {/* Dead Zone Slider */}
          <div className="space-y-2">
            <div className="flex justify-between items-center">
              <Label>Dead Zone</Label>
              <span className="text-sm font-semibold">
                {settings.dead_zone.toFixed(2)}
              </span>
            </div>
            <Slider
              value={[settings.dead_zone]}
              onValueChange={(value) => updateSetting("dead_zone", value[0])}
              min={0.0}
              max={0.3}
              step={0.01}
              className="w-full"
            />
            <p className="text-xs text-muted-foreground">
              Minimum stick movement to register input (prevents drift)
            </p>
          </div>

          {/* Acceleration Slider */}
          <div className="space-y-2">
            <div className="flex justify-between items-center">
              <Label>Acceleration</Label>
              <span className="text-sm font-semibold">
                {settings.acceleration.toFixed(2)}x
              </span>
            </div>
            <Slider
              value={[settings.acceleration]}
              onValueChange={(value) => updateSetting("acceleration", value[0])}
              min={0.8}
              max={2.0}
              step={0.1}
              className="w-full"
            />
            <p className="text-xs text-muted-foreground">
              Multiplier combined with sensitivity for fine control
            </p>
          </div>

          {/* Control Instructions */}
          <div className="p-4 bg-blue-50 dark:bg-blue-950 rounded-lg text-sm space-y-2">
            <div className="flex gap-2">
              <Zap className="w-4 h-4 shrink-0 text-blue-600 dark:text-blue-400 mt-0.5" />
              <div>
                <p className="font-semibold text-blue-900 dark:text-blue-100">
                  Controls
                </p>
                <ul className="list-disc list-inside text-blue-800 dark:text-blue-200 space-y-1">
                  <li>Left Stick: Move cursor</li>
                  <li>Right Trigger (RT): Left click</li>
                  <li>Left Trigger (LT): Right click</li>
                </ul>
              </div>
            </div>
          </div>
        </CardContent>
      </Card>
    </div>
  );
}
