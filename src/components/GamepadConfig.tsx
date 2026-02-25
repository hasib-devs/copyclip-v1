import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "./ui/card";
import { Button } from "./ui/button";
import { Switch } from "./ui/switch";
import { Label } from "./ui/label";
import { Alert, AlertDescription } from "./ui/alert";
import { AlertCircle, Joystick, Zap } from "lucide-react";
import { useGamepadContext } from "@/assets/contexts/gamepad-context";
import { useGamepad } from "@/hooks/useGamepad";
import {
  useGamepadMonitor,
  useLoadGamepadProfiles,
} from "@/hooks/useGamepadMonitor";

export function GamepadConfig() {
  const { state, dispatch } = useGamepadContext();
  const gamepadTools = useGamepad(state, dispatch);

  // Monitor gamepad input when listening
  useGamepadMonitor(state.isListening, dispatch);

  // Load profiles on mount
  useLoadGamepadProfiles(dispatch);

  const activeGamepad = gamepadTools.activeGamepad;

  return (
    <div className="w-full max-w-3xl mx-auto p-4 space-y-6">
      <Card>
        <CardHeader>
          <div className="flex items-center gap-2">
            <Joystick className="w-5 h-5" />
            <div>
              <CardTitle>Gamepad Configuration</CardTitle>
              <CardDescription>
                Control your mouse with any gamepad (PS5, Xbox, Nintendo, etc.)
              </CardDescription>
            </div>
          </div>
        </CardHeader>
        <CardContent className="space-y-6">
          {state.error && (
            <Alert variant="destructive">
              <AlertCircle className="h-4 w-4" />
              <AlertDescription>{state.error}</AlertDescription>
            </Alert>
          )}

          {/* Enable/Disable Toggle */}
          <div className="flex items-center justify-between p-4 border rounded-lg">
            <div>
              <Label className="text-base">Enable Gamepad Input</Label>
              <p className="text-sm text-muted-foreground">
                Activate gamepad listener for all connected devices
              </p>
            </div>
            <div className="flex items-center gap-3">
              <Switch
                checked={state.isListening}
                onCheckedChange={gamepadTools.toggleListening}
                disabled={state.isLoading}
              />
              {state.isLoading && (
                <div className="w-4 h-4 border-2 border-primary border-t-transparent rounded-full animate-spin" />
              )}
            </div>
          </div>

          {/* Connected Gamepads */}
          {state.gamepads.length > 0 ? (
            <div className="space-y-3">
              <div className="flex justify-between items-center">
                <Label className="text-base font-semibold">
                  Connected Gamepads (
                  {state.gamepads.filter((g) => g.connected).length})
                </Label>
                <Button
                  size="sm"
                  variant="outline"
                  onClick={gamepadTools.refreshGamepads}
                  disabled={!state.isListening}
                >
                  Refresh
                </Button>
              </div>
              <div className="grid gap-2">
                {state.gamepads.map((gamepad, idx) => (
                  <button
                    key={idx}
                    onClick={() => gamepadTools.setActiveGamepad(idx)}
                    className={`p-3 rounded-lg text-left border transition-all ${
                      state.activeGamepadIndex === idx
                        ? "border-blue-500 bg-blue-50 dark:bg-blue-950"
                        : "border-gray-200 hover:border-gray-300"
                    }`}
                  >
                    <div className="flex items-center justify-between">
                      <div>
                        <p className="font-medium">{gamepad.id}</p>
                        <p className="text-sm text-muted-foreground">
                          {gamepad.mapping} • {gamepad.vibration_actuators}{" "}
                          vibration motors
                        </p>
                      </div>
                      <span
                        className={`text-sm font-semibold ${
                          gamepad.connected ? "text-green-600" : "text-gray-500"
                        }`}
                      >
                        {gamepad.connected ? "🟢 Connected" : "⚫ Offline"}
                      </span>
                    </div>
                  </button>
                ))}
              </div>
            </div>
          ) : (
            <div className="p-4 bg-yellow-50 dark:bg-yellow-950 rounded-lg text-sm">
              <p className="text-yellow-900 dark:text-yellow-100">
                No gamepads connected. Connect a gamepad via USB or Bluetooth.
              </p>
            </div>
          )}

          {/* Active Gamepad Details */}
          {state.isListening && activeGamepad && activeGamepad.connected && (
            <div className="space-y-3">
              <Label className="text-base font-semibold">
                Real-time Input State
              </Label>
              <div className="grid grid-cols-2 gap-3 p-4 bg-muted rounded-lg text-sm font-mono">
                <div>
                  Left Stick X:{" "}
                  <span className="font-bold">
                    {activeGamepad.axes[0]?.toFixed(2) || "0.00"}
                  </span>
                </div>
                <div>
                  Left Stick Y:{" "}
                  <span className="font-bold">
                    {activeGamepad.axes[1]?.toFixed(2) || "0.00"}
                  </span>
                </div>
                <div>
                  Right Stick X:{" "}
                  <span className="font-bold">
                    {activeGamepad.axes[2]?.toFixed(2) || "0.00"}
                  </span>
                </div>
                <div>
                  Right Stick Y:{" "}
                  <span className="font-bold">
                    {activeGamepad.axes[3]?.toFixed(2) || "0.00"}
                  </span>
                </div>
                <div className="col-span-2 border-t pt-2 mt-2 font-normal">
                  Buttons Pressed:{" "}
                  <span className="font-bold">
                    {activeGamepad.buttons.filter((b) => b.pressed).length} /{" "}
                    {activeGamepad.buttons.length}
                  </span>
                </div>
              </div>
            </div>
          )}

          {/* Profiles Section */}
          {state.profiles.length > 0 && (
            <div className="space-y-3 border-t pt-4">
              <Label className="text-base font-semibold">Saved Profiles</Label>
              <div className="grid gap-2">
                {state.profiles.map((profile) => (
                  <div
                    key={profile.name}
                    className="p-3 rounded-lg border hover:border-blue-300 transition-colors"
                  >
                    <div className="flex justify-between items-start">
                      <div className="flex-1">
                        <p className="font-medium">{profile.name}</p>
                        <p className="text-xs text-muted-foreground mt-1">
                          {profile.description}
                        </p>
                        <div className="flex gap-4 text-xs mt-2 text-muted-foreground">
                          <span>Sensitivity: {profile.sensitivity}x</span>
                          <span>Dead Zone: {profile.dead_zone}</span>
                          <span>Acceleration: {profile.acceleration}x</span>
                        </div>
                      </div>
                      <div className="flex gap-2">
                        {state.activeProfile?.name === profile.name && (
                          <span className="px-2 py-1 text-xs bg-green-100 text-green-800 rounded">
                            Active
                          </span>
                        )}
                        <Button
                          size="sm"
                          variant="ghost"
                          onClick={() =>
                            gamepadTools.deleteProfile(profile.name)
                          }
                        >
                          Delete
                        </Button>
                      </div>
                    </div>
                  </div>
                ))}
              </div>
            </div>
          )}

          {/* Control Instructions */}
          <div className="p-4 bg-blue-50 dark:bg-blue-950 rounded-lg text-sm space-y-2">
            <div className="flex gap-2">
              <Zap className="w-4 h-4 shrink-0 text-blue-600 dark:text-blue-400 mt-0.5" />
              <div>
                <p className="font-semibold text-blue-900 dark:text-blue-100">
                  Default Controls (Any Gamepad)
                </p>
                <ul className="list-disc list-inside text-blue-800 dark:text-blue-200 space-y-1 mt-1">
                  <li>Left Stick / D-Pad: Move cursor</li>
                  <li>Right Trigger (RT/R2): Left click</li>
                  <li>Left Trigger (LT/L2): Right click</li>
                  <li>
                    Works with PS5, PS4, Xbox, Nintendo and other standard
                    gamepads
                  </li>
                </ul>
              </div>
            </div>
          </div>

          {/* Browser Gamepad API Info */}
          <div className="p-3 bg-gray-50 dark:bg-gray-900 rounded text-xs text-muted-foreground border">
            <p>
              <strong>Technical:</strong> Implements the HTML Gamepad API
              standard. All gamepads are mapped to the{" "}
              <code className="bg-black/10 px-1 rounded">standard</code> mapping
              for consistent behavior.
            </p>
          </div>
        </CardContent>
      </Card>
    </div>
  );
}
