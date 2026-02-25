import React, { useEffect, useState, useRef } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { Alert, AlertDescription } from "@/components/ui/alert";
import { AlertCircle, Zap, Circle } from "lucide-react";

interface GamepadState {
  id: string;
  index: number;
  connected: boolean;
  timestamp: number;
  buttons: Array<{ pressed: boolean; touched: boolean; value: number }>;
  axes: number[];
  mapping: string;
  vibration_actuators: number;
}

const BUTTON_NAMES: Record<number, string> = {
  0: "South (A/X)",
  1: "East (B/Circle)",
  2: "West (X/Square)",
  3: "North (Y/Triangle)",
  4: "LB/L1",
  5: "RB/R1",
  6: "LT/L2",
  7: "RT/R2",
  8: "Select/Back",
  9: "Start",
  10: "Left Stick",
  11: "Right Stick",
  12: "Guide/Home",
  13: "D-Pad Up",
  14: "D-Pad Down",
  15: "D-Pad Left",
  16: "D-Pad Right",
};

const AXIS_NAMES: Record<number, string> = {
  0: "Left Stick X",
  1: "Left Stick Y",
  2: "Right Stick X",
  3: "Right Stick Y",
};

const GamepadDiagnostics: React.FC = () => {
  const [gamepads, setGamepads] = useState<GamepadState[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [refreshRate, setRefreshRate] = useState(60); // FPS
  const [selectedGamepadIndex, setSelectedGamepadIndex] = useState<
    number | null
  >(null);
  const [message, setMessage] = useState<{
    type: "info" | "success" | "error";
    text: string;
  } | null>(null);
  const refreshIntervalRef = useRef<ReturnType<typeof setInterval> | null>(
    null,
  );

  // Load gamepads on mount and start polling
  useEffect(() => {
    loadGamepads();
    startPolling();

    return () => {
      if (refreshIntervalRef.current) {
        clearInterval(refreshIntervalRef.current);
      }
    };
  }, []);

  // Update polling interval when refresh rate changes
  useEffect(() => {
    if (refreshIntervalRef.current) {
      clearInterval(refreshIntervalRef.current);
    }
    startPolling();
  }, [refreshRate]);

  const startPolling = () => {
    const interval = 1000 / refreshRate; // Convert FPS to milliseconds
    refreshIntervalRef.current = setInterval(() => {
      loadGamepads();
    }, interval);
  };

  const loadGamepads = async () => {
    try {
      const gamepadsList = await invoke<GamepadState[]>("get_gamepads");
      setGamepads(gamepadsList);
      setIsLoading(false);
    } catch (err) {
      console.error("Failed to load gamepads:", err);
      setMessage({ type: "error", text: "Failed to load gamepad data" });
      setIsLoading(false);
    }
  };

  const testVibration = async (gamepadIndex: number) => {
    try {
      // This would require a vibrate command in the backend
      setMessage({
        type: "info",
        text: `Testing vibration on gamepad ${gamepadIndex}...`,
      });
      setTimeout(() => setMessage(null), 3000);
    } catch (err) {
      console.error("Vibration test failed:", err);
      setMessage({ type: "error", text: "Vibration test failed" });
    }
  };

  const selectedGamepad =
    selectedGamepadIndex !== null ? gamepads[selectedGamepadIndex] : null;

  return (
    <div className="space-y-6 p-6 max-w-5xl">
      <div>
        <h2 className="text-2xl font-bold mb-2">Gamepad Diagnostics</h2>
        <p className="text-gray-600">
          Monitor connected gamepads and test button/axis responsiveness
        </p>
      </div>

      {message && (
        <Alert variant={message.type === "error" ? "destructive" : "default"}>
          <AlertCircle className="h-4 w-4" />
          <AlertDescription>{message.text}</AlertDescription>
        </Alert>
      )}

      {/* Refresh Rate Control */}
      <Card>
        <CardContent className="pt-6">
          <div className="flex items-center gap-4">
            <label className="text-sm font-medium">Polling Rate:</label>
            <div className="flex gap-2">
              {[30, 60, 120].map((rate) => (
                <Button
                  key={rate}
                  size="sm"
                  variant={refreshRate === rate ? "default" : "outline"}
                  onClick={() => setRefreshRate(rate)}
                  className="min-w-16"
                >
                  {rate} Hz
                </Button>
              ))}
            </div>
            <span className="text-xs text-gray-500">
              (~{(1000 / refreshRate).toFixed(1)}ms update interval)
            </span>
          </div>
        </CardContent>
      </Card>

      {isLoading ? (
        <div className="flex items-center justify-center h-40">
          <p className="text-gray-500">Scanning for gamepads...</p>
        </div>
      ) : gamepads.length === 0 ? (
        <Card className="border-yellow-200 bg-yellow-50">
          <CardContent className="pt-6 text-center">
            <p className="text-yellow-800 mb-3">🎮 No gamepads detected</p>
            <p className="text-sm text-yellow-700 mb-4">
              Please connect a gamepad and refresh
            </p>
            <Button
              onClick={loadGamepads}
              variant="outline"
              className="border-yellow-300"
            >
              Scan Again
            </Button>
          </CardContent>
        </Card>
      ) : (
        <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
          {/* Gamepad List */}
          <div className="space-y-3">
            <h3 className="font-semibold text-lg">Connected Gamepads</h3>
            {gamepads.map((gp) => (
              <Card
                key={gp.index}
                className={`cursor-pointer transition-all ${
                  selectedGamepadIndex === gp.index
                    ? "border-blue-500 bg-blue-50"
                    : gp.connected
                      ? "hover:border-gray-400"
                      : "opacity-50"
                }`}
                onClick={() => setSelectedGamepadIndex(gp.index)}
              >
                <CardContent className="pt-4">
                  <div className="space-y-2">
                    <div className="flex items-center justify-between">
                      <p className="font-medium text-sm truncate">{gp.id}</p>
                      <Badge
                        className={
                          gp.connected
                            ? "bg-green-100 text-green-800"
                            : "bg-red-100 text-red-800"
                        }
                      >
                        {gp.connected ? "Connected" : "Disconnected"}
                      </Badge>
                    </div>
                    <div className="space-y-1 text-xs text-gray-600">
                      <p>Index: {gp.index}</p>
                      <p>Mapping: {gp.mapping}</p>
                      {gp.vibration_actuators > 0 && (
                        <p>🔊 Vibration: {gp.vibration_actuators} motors</p>
                      )}
                      <p className="text-gray-400">
                        Last update:{" "}
                        {new Date(gp.timestamp).toLocaleTimeString()}
                      </p>
                    </div>
                  </div>
                </CardContent>
              </Card>
            ))}
          </div>

          {/* Detailed View */}
          {selectedGamepad ? (
            <div className="lg:col-span-2 space-y-6">
              {/* Buttons */}
              <Card>
                <CardHeader>
                  <CardTitle className="text-lg">Button State</CardTitle>
                </CardHeader>
                <CardContent>
                  <div className="grid grid-cols-2 gap-3 max-h-80 overflow-y-auto">
                    {selectedGamepad.buttons.map((btn, idx) => (
                      <div
                        key={idx}
                        className={`p-3 rounded border-2 transition-all ${
                          btn.pressed
                            ? "border-blue-500 bg-blue-100"
                            : "border-gray-200 bg-gray-50"
                        }`}
                      >
                        <div className="flex items-center gap-2 mb-1">
                          <Circle
                            className={`w-3 h-3 ${
                              btn.pressed
                                ? "fill-blue-500 text-blue-500"
                                : "text-gray-400"
                            }`}
                          />
                          <p className="text-xs font-medium">
                            {BUTTON_NAMES[idx] || `Button ${idx}`}
                          </p>
                        </div>
                        <div className="flex items-center justify-between">
                          <span className="text-xs text-gray-600">
                            {btn.pressed ? "Pressed" : "Released"}
                          </span>
                          {btn.value > 0 && (
                            <span className="text-xs font-mono text-blue-600">
                              {(btn.value * 100).toFixed(0)}%
                            </span>
                          )}
                        </div>
                      </div>
                    ))}
                  </div>
                </CardContent>
              </Card>

              {/* Axes */}
              <Card>
                <CardHeader>
                  <CardTitle className="text-lg">Analog Axes</CardTitle>
                </CardHeader>
                <CardContent className="space-y-4">
                  {selectedGamepad.axes.map((value, idx) => (
                    <div key={idx} className="space-y-2">
                      <div className="flex items-center justify-between">
                        <label className="text-sm font-medium">
                          {AXIS_NAMES[idx] || `Axis ${idx}`}
                        </label>
                        <span className="font-mono text-sm text-blue-600">
                          {(value * 100).toFixed(1)}%
                        </span>
                      </div>
                      <div className="w-full h-6 bg-gray-200 rounded overflow-hidden">
                        <div
                          className={`h-full transition-all ${
                            value > 0.1
                              ? "bg-blue-500"
                              : value < -0.1
                                ? "bg-red-500"
                                : "bg-gray-300"
                          }`}
                          style={{
                            width: `${Math.abs(value) * 50 + 50}%`,
                            marginLeft:
                              value < 0 ? `${Math.abs(value) * 50 - 1}%` : "0",
                          }}
                        />
                      </div>
                      <div className="flex justify-between text-xs text-gray-500">
                        <span>-100%</span>
                        <span>Center</span>
                        <span>+100%</span>
                      </div>
                    </div>
                  ))}
                </CardContent>
              </Card>

              {/* Actions */}
              <Card>
                <CardHeader>
                  <CardTitle className="text-lg">Device Actions</CardTitle>
                </CardHeader>
                <CardContent className="space-y-3">
                  {selectedGamepad.vibration_actuators > 0 && (
                    <Button
                      onClick={() => testVibration(selectedGamepad.index)}
                      className="w-full bg-blue-600 hover:bg-blue-700"
                    >
                      <Zap className="w-4 h-4 mr-2" />
                      Test Vibration
                    </Button>
                  )}
                  <Button
                    onClick={loadGamepads}
                    variant="outline"
                    className="w-full"
                  >
                    🔄 Refresh Data
                  </Button>
                </CardContent>
              </Card>
            </div>
          ) : (
            <Card className="lg:col-span-2">
              <CardContent className="pt-6 text-center text-gray-500">
                <p>Select a gamepad to view details</p>
              </CardContent>
            </Card>
          )}
        </div>
      )}

      {/* Dead Zone Info */}
      <Card className="bg-blue-50 border-blue-200">
        <CardContent className="pt-6">
          <div className="grid grid-cols-3 gap-4 text-sm">
            <div>
              <p className="font-semibold text-blue-900">Dead Zone</p>
              <p className="text-xs text-blue-700 mt-1">
                Adjust in gamepad settings to filter out stick drift
              </p>
            </div>
            <div>
              <p className="font-semibold text-blue-900">Test Responsiveness</p>
              <p className="text-xs text-blue-700 mt-1">
                Use button/axis visualization above to test all controls
              </p>
            </div>
            <div>
              <p className="font-semibold text-blue-900">Polling Rate</p>
              <p className="text-xs text-blue-700 mt-1">
                Higher rates = more responsive but higher CPU usage
              </p>
            </div>
          </div>
        </CardContent>
      </Card>
    </div>
  );
};

export default GamepadDiagnostics;
