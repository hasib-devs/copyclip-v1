import React, { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Button } from "@/components/ui/button";
import { Card, CardContent } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { Alert, AlertDescription } from "@/components/ui/alert";
import { AlertCircle } from "lucide-react";

interface ButtonMapping {
  button: string;
  action: string;
  description: string;
}

const GAMEPAD_BUTTONS = [
  { id: "South", label: "A / Cross", group: "Face" },
  { id: "East", label: "B / Circle", group: "Face" },
  { id: "West", label: "X / Square", group: "Face" },
  { id: "North", label: "Y / Triangle", group: "Face" },
  { id: "LB", label: "LB / L1", group: "Shoulders" },
  { id: "RB", label: "RB / R1", group: "Shoulders" },
  { id: "LT", label: "LT / L2", group: "Triggers" },
  { id: "RT", label: "RT / R2", group: "Triggers" },
  { id: "LeftStick", label: "Left Stick Click", group: "Sticks" },
  { id: "RightStick", label: "Right Stick Click", group: "Sticks" },
  { id: "Select", label: "Select / Back", group: "Menu" },
  { id: "Start", label: "Start", group: "Menu" },
];

const AVAILABLE_ACTIONS = [
  { id: "LeftClick", label: "Left Click", category: "Mouse" },
  { id: "RightClick", label: "Right Click", category: "Mouse" },
  { id: "MiddleClick", label: "Middle Click", category: "Mouse" },
  { id: "DoubleClick", label: "Double Click", category: "Mouse" },
  { id: "ScrollUp", label: "Scroll Up", category: "Scroll" },
  { id: "ScrollDown", label: "Scroll Down", category: "Scroll" },
  { id: "ScrollLeft", label: "Scroll Left", category: "Scroll" },
  { id: "ScrollRight", label: "Scroll Right", category: "Scroll" },
  { id: "Key_Return", label: "Return / Enter", category: "Keyboard" },
  { id: "Key_Escape", label: "Escape", category: "Keyboard" },
  { id: "Key_Tab", label: "Tab", category: "Keyboard" },
  { id: "Key_Space", label: "Space", category: "Keyboard" },
  { id: "Cmd_Left", label: "Cmd + Left", category: "Shortcuts" },
  { id: "Cmd_Right", label: "Cmd + Right", category: "Shortcuts" },
  { id: "Cmd_Up", label: "Cmd + Up", category: "Shortcuts" },
  { id: "Cmd_Down", label: "Cmd + Down", category: "Shortcuts" },
  { id: "SwitchModeNormal", label: "Switch to Normal Mode", category: "Mode" },
  { id: "SwitchModeMotion", label: "Switch to Motion Mode", category: "Mode" },
  { id: "SwitchModeHotkey", label: "Switch to Hotkey Mode", category: "Mode" },
];

const KeybindingEditor: React.FC = () => {
  const [mappings, setMappings] = useState<ButtonMapping[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [hasChanges, setHasChanges] = useState(false);
  const [activeGroup, setActiveGroup] = useState<string>("Face");
  const [expandedButton, setExpandedButton] = useState<string | null>(null);
  const [message, setMessage] = useState<{
    type: "success" | "error";
    text: string;
  } | null>(null);

  // Load current mappings on mount
  useEffect(() => {
    const loadMappings = async () => {
      try {
        setIsLoading(true);
        // Fetch keybindings from backend
        const keybindings = await invoke<Array<[string, string]>>(
          "get_gamepad_keybindings",
        );

        const mappings: ButtonMapping[] = keybindings.map(
          ([button, action]) => {
            const buttonInfo = GAMEPAD_BUTTONS.find((b) => b.id === button);
            const actionInfo = AVAILABLE_ACTIONS.find((a) => a.id === action);
            return {
              button,
              action,
              description: `${buttonInfo?.label || button} → ${actionInfo?.label || action}`,
            };
          },
        );
        setMappings(mappings);
      } catch (err) {
        console.error("Failed to load keybindings:", err);
        setMessage({ type: "error", text: "Failed to load keybindings" });
      } finally {
        setIsLoading(false);
      }
    };

    loadMappings();
  }, []);

  const handleActionChange = (buttonId: string, newAction: string) => {
    setMappings((prev) =>
      prev.map((m) => {
        if (m.button === buttonId) {
          const action = AVAILABLE_ACTIONS.find((a) => a.id === newAction);
          return {
            ...m,
            action: newAction,
            description: `${
              GAMEPAD_BUTTONS.find((b) => b.id === buttonId)?.label
            } → ${action?.label}`,
          };
        }
        return m;
      }),
    );
    setHasChanges(true);
  };

  const handleSave = async () => {
    try {
      const keybindings = mappings.map(
        (m) => [m.button, m.action] as [string, string],
      );
      await invoke("save_gamepad_keybindings", { keybindings });
      setHasChanges(false);
      setMessage({ type: "success", text: "Keybindings saved successfully!" });
      setTimeout(() => setMessage(null), 3000);
    } catch (err) {
      console.error("Failed to save keybindings:", err);
      setMessage({ type: "error", text: "Failed to save keybindings" });
    }
  };

  const handleReset = () => {
    setHasChanges(false);
    // Reload mappings
  };

  const buttons = GAMEPAD_BUTTONS.filter((b) => b.group === activeGroup);
  const buttonMappings = mappings.filter((m) =>
    buttons.some((b) => b.id === m.button),
  );

  const groups = Array.from(new Set(GAMEPAD_BUTTONS.map((b) => b.group)));

  return (
    <div className="space-y-6 p-6">
      <div>
        <h2 className="text-2xl font-bold mb-2">Keybinding Editor</h2>
        <p className="text-gray-600">
          Customize how gamepad buttons map to actions
        </p>
      </div>

      {message && (
        <Alert variant={message.type === "error" ? "destructive" : "default"}>
          <AlertCircle className="h-4 w-4" />
          <AlertDescription>{message.text}</AlertDescription>
        </Alert>
      )}

      {isLoading ? (
        <div className="flex items-center justify-center h-40">
          <p className="text-gray-500">Loading keybindings...</p>
        </div>
      ) : (
        <>
          {/* Group Selector */}
          <div className="flex gap-2 flex-wrap">
            {groups.map((group) => (
              <Button
                key={group}
                variant={activeGroup === group ? "default" : "outline"}
                onClick={() => setActiveGroup(group)}
                className="min-w-20"
              >
                {group}
              </Button>
            ))}
          </div>

          {/* Keybinding Grid */}
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            {buttonMappings.map((mapping) => {
              const button = GAMEPAD_BUTTONS.find(
                (b) => b.id === mapping.button,
              );
              const action = AVAILABLE_ACTIONS.find(
                (a) => a.id === mapping.action,
              );
              const isExpanded = expandedButton === mapping.button;

              return (
                <Card key={mapping.button}>
                  <CardContent className="pt-6">
                    <div className="space-y-3">
                      <div className="flex items-center justify-between">
                        <Badge
                          variant="secondary"
                          className="text-sm font-semibold"
                        >
                          {button?.label}
                        </Badge>
                        <Badge variant="outline" className="text-xs">
                          {action?.category}
                        </Badge>
                      </div>

                      <div>
                        <p className="text-sm font-medium text-slate-700 mb-2">
                          {action?.label}
                        </p>
                        <Button
                          variant={isExpanded ? "default" : "outline"}
                          size="sm"
                          onClick={() =>
                            setExpandedButton(
                              isExpanded ? null : mapping.button,
                            )
                          }
                          className="w-full"
                        >
                          {isExpanded ? "Cancel" : "Change Action"}
                        </Button>
                      </div>

                      {isExpanded && (
                        <div className="border-t pt-3 space-y-2 max-h-48 overflow-y-auto">
                          {Array.from(
                            new Set(AVAILABLE_ACTIONS.map((a) => a.category)),
                          ).map((category) => (
                            <div key={category}>
                              <p className="text-xs font-semibold text-gray-500 mb-1 px-2">
                                {category}
                              </p>
                              {AVAILABLE_ACTIONS.filter(
                                (a) => a.category === category,
                              ).map((act) => (
                                <Button
                                  key={act.id}
                                  variant={
                                    act.id === mapping.action
                                      ? "default"
                                      : "ghost"
                                  }
                                  size="sm"
                                  className="w-full justify-start text-left"
                                  onClick={() => {
                                    handleActionChange(mapping.button, act.id);
                                    setExpandedButton(null);
                                  }}
                                >
                                  {act.label}
                                </Button>
                              ))}
                            </div>
                          ))}
                        </div>
                      )}
                    </div>
                  </CardContent>
                </Card>
              );
            })}
          </div>

          {/* Save/Reset Buttons */}
          {hasChanges && (
            <div className="flex gap-2 justify-end pt-4 border-t">
              <Button variant="outline" onClick={handleReset}>
                Cancel
              </Button>
              <Button onClick={handleSave} className="bg-blue-600">
                Save Keybindings
              </Button>
            </div>
          )}
        </>
      )}
    </div>
  );
};

export default KeybindingEditor;
