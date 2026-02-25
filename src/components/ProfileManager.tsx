import React, { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Button } from "@/components/ui/button";
import { Card, CardContent } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { Alert, AlertDescription } from "@/components/ui/alert";
import { AlertCircle, Plus, Trash2, Download } from "lucide-react";

interface GamepadProfile {
  name: string;
  description: string;
  sensitivity: number;
  dead_zone: number;
  acceleration: number;
  button_map: Record<string, string>;
  axis_map: Record<string, string>;
  enabled_features: {
    mouse_control: boolean;
    keyboard_emulation: boolean;
    vibration: boolean;
    adaptive_triggers: boolean;
    scroll_control: boolean;
  };
  scroll_settings: {
    enabled: boolean;
    vertical_speed: number;
    horizontal_speed: number;
    reverse_vertical: boolean;
    reverse_horizontal: boolean;
  };
  dpad_mapping: {
    up: { single: string };
    down: { single: string };
    left: { single: string };
    right: { single: string };
  };
}

const ProfileManager: React.FC = () => {
  const [profiles, setProfiles] = useState<GamepadProfile[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [message, setMessage] = useState<{
    type: "success" | "error";
    text: string;
  } | null>(null);
  const [showCreateForm, setShowCreateForm] = useState(false);
  const [newProfileName, setNewProfileName] = useState("");
  const [newProfileDescription, setNewProfileDescription] = useState("");
  const [isCreating, setIsCreating] = useState(false);
  const [activeProfile, setActiveProfile] = useState<string | null>(null);

  // Load profiles on mount
  useEffect(() => {
    loadProfiles();
  }, []);

  const loadProfiles = async () => {
    try {
      setIsLoading(true);
      const profilesList = await invoke<GamepadProfile[]>(
        "get_gamepad_profiles",
      );
      setProfiles(profilesList);
      // Find active profile
      const active = profilesList.find((p) => p.name === "Default"); // TODO: Check actual active flag
      if (active) setActiveProfile(active.name);
    } catch (err) {
      console.error("Failed to load profiles:", err);
      setMessage({ type: "error", text: "Failed to load profiles" });
    } finally {
      setIsLoading(false);
    }
  };

  const handleCreateProfile = async () => {
    if (!newProfileName.trim()) {
      setMessage({ type: "error", text: "Profile name is required" });
      return;
    }

    try {
      setIsCreating(true);
      const newProfile: GamepadProfile = {
        name: newProfileName,
        description: newProfileDescription,
        sensitivity: 1.0,
        dead_zone: 0.1,
        acceleration: 1.0,
        button_map: {},
        axis_map: {},
        enabled_features: {
          mouse_control: true,
          keyboard_emulation: false,
          vibration: true,
          adaptive_triggers: false,
          scroll_control: true,
        },
        scroll_settings: {
          enabled: true,
          vertical_speed: 1.0,
          horizontal_speed: 1.0,
          reverse_vertical: false,
          reverse_horizontal: false,
        },
        dpad_mapping: {
          up: { single: "Up" },
          down: { single: "Down" },
          left: { single: "Left" },
          right: { single: "Right" },
        },
      };

      await invoke("save_gamepad_profile", { profile: newProfile });
      setNewProfileName("");
      setNewProfileDescription("");
      setShowCreateForm(false);
      setMessage({ type: "success", text: "Profile created successfully!" });
      setTimeout(() => setMessage(null), 3000);
      loadProfiles();
    } catch (err) {
      console.error("Failed to create profile:", err);
      setMessage({ type: "error", text: "Failed to create profile" });
    } finally {
      setIsCreating(false);
    }
  };

  const handleDeleteProfile = async (profileName: string) => {
    if (!confirm(`Delete profile "${profileName}"?`)) return;

    try {
      await invoke("delete_gamepad_profile", { profile_name: profileName });
      setMessage({ type: "success", text: "Profile deleted successfully!" });
      setTimeout(() => setMessage(null), 3000);
      loadProfiles();
    } catch (err) {
      console.error("Failed to delete profile:", err);
      setMessage({ type: "error", text: "Failed to delete profile" });
    }
  };

  const handleSetActive = async (profileName: string) => {
    try {
      await invoke("set_active_gamepad_profile", { profile_name: profileName });
      setActiveProfile(profileName);
      setMessage({
        type: "success",
        text: `Switched to "${profileName}" profile`,
      });
      setTimeout(() => setMessage(null), 3000);
      loadProfiles();
    } catch (err) {
      console.error("Failed to set active profile:", err);
      setMessage({ type: "error", text: "Failed to switch profile" });
    }
  };

  return (
    <div className="space-y-6 p-6 max-w-3xl">
      <div>
        <h2 className="text-2xl font-bold mb-2">Profile Manager</h2>
        <p className="text-gray-600">
          Create and manage different gamepad input profiles
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
          <p className="text-gray-500">Loading profiles...</p>
        </div>
      ) : (
        <>
          {/* Create Profile Form */}
          {showCreateForm && (
            <Card>
              <CardContent className="pt-6">
                <h3 className="font-semibold mb-4">Create New Profile</h3>
                <div className="space-y-4">
                  <div>
                    <label className="text-sm font-medium">Profile Name</label>
                    <input
                      type="text"
                      placeholder="e.g., Gaming, Work, Browsing"
                      value={newProfileName}
                      onChange={(e) => setNewProfileName(e.target.value)}
                      className="w-full px-3 py-2 border border-slate-200 rounded-lg text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
                    />
                  </div>
                  <div>
                    <label className="text-sm font-medium">
                      Description (Optional)
                    </label>
                    <textarea
                      placeholder="Describe what this profile is for..."
                      value={newProfileDescription}
                      onChange={(e) => setNewProfileDescription(e.target.value)}
                      rows={3}
                      className="w-full px-3 py-2 border border-slate-200 rounded-lg text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
                    />
                  </div>
                  <div className="flex gap-2 justify-end pt-2">
                    <Button
                      variant="outline"
                      onClick={() => setShowCreateForm(false)}
                    >
                      Cancel
                    </Button>
                    <Button
                      onClick={handleCreateProfile}
                      disabled={isCreating || !newProfileName.trim()}
                      className="bg-blue-600 hover:bg-blue-700"
                    >
                      {isCreating ? "Creating..." : "Create Profile"}
                    </Button>
                  </div>
                </div>
              </CardContent>
            </Card>
          )}

          {/* Create Button */}
          {!showCreateForm && (
            <Button
              onClick={() => setShowCreateForm(true)}
              className="w-full bg-blue-600 hover:bg-blue-700"
            >
              <Plus className="w-4 h-4 mr-2" />
              New Profile
            </Button>
          )}

          {/* Profiles Grid */}
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            {profiles.length === 0 ? (
              <Card className="md:col-span-2">
                <CardContent className="pt-6 text-center">
                  <p className="text-gray-500 mb-4">No profiles created yet</p>
                  <Button
                    onClick={() => setShowCreateForm(true)}
                    className="bg-blue-600 hover:bg-blue-700"
                  >
                    Create Your First Profile
                  </Button>
                </CardContent>
              </Card>
            ) : (
              profiles.map((profile) => (
                <Card key={profile.name}>
                  <CardContent className="pt-6">
                    <div className="space-y-3">
                      <div className="flex items-start justify-between">
                        <div>
                          <h3 className="font-semibold text-lg">
                            {profile.name}
                          </h3>
                          {profile.description && (
                            <p className="text-xs text-gray-500 mt-1">
                              {profile.description}
                            </p>
                          )}
                        </div>
                        {activeProfile === profile.name && (
                          <Badge className="bg-green-100 text-green-800">
                            Active
                          </Badge>
                        )}
                      </div>

                      <p className="text-xs text-gray-400">
                        Sensitivity: {profile.sensitivity.toFixed(1)}x
                      </p>

                      <div className="flex gap-2 pt-2">
                        {activeProfile !== profile.name && (
                          <Button
                            size="sm"
                            variant="outline"
                            onClick={() => handleSetActive(profile.name)}
                            className="flex-1"
                          >
                            Activate
                          </Button>
                        )}
                        <Button
                          size="sm"
                          variant="outline"
                          title="Export profile"
                        >
                          <Download className="w-4 h-4" />
                        </Button>
                        {profile.name !== "Default" && (
                          <Button
                            size="sm"
                            variant="outline"
                            onClick={() => handleDeleteProfile(profile.name)}
                            className="text-red-600 hover:text-red-700 border-red-200"
                          >
                            <Trash2 className="w-4 h-4" />
                          </Button>
                        )}
                      </div>
                    </div>
                  </CardContent>
                </Card>
              ))
            )}
          </div>

          {/* Quick Actions */}
          <Card>
            <CardContent className="pt-6">
              <h3 className="font-semibold mb-3">Import/Export</h3>
              <div className="flex gap-2">
                <Button variant="outline" className="flex-1">
                  📥 Import Profile
                </Button>
                <Button variant="outline" className="flex-1">
                  📤 Export Active
                </Button>
              </div>
            </CardContent>
          </Card>
        </>
      )}
    </div>
  );
};

export default ProfileManager;
