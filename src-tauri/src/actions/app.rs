/// Application actions: app switching, launching, window management
/// Different implementations for macOS, Windows, Linux

use log::{debug, error, info};

/// Open application launcher (Spotlight on macOS, etc.)
pub fn open_app_launcher() -> Result<(), String> {
    debug!("[App] Open launcher");

    #[cfg(target_os = "macos")]
    {
        // macOS: Open Spotlight
        info!("[macOS] Opening Spotlight");
        match std::process::Command::new("osascript")
            .arg("-e")
            .arg("tell application \"System Events\" to keystroke space using command down")
            .output()
        {
            Ok(output) if output.status.success() => Ok(()),
            _ => Err("Failed to open Spotlight".to_string()),
        }
    }

    #[cfg(target_os = "windows")]
    {
        // Windows: Open Start Menu
        info!("[Windows] Opening Start Menu");
        match std::process::Command::new("cmd")
            .args(&["/C", "powershell -Command Add-Type -AssemblyName System.Windows.Forms; [System.Windows.Forms.SendKeys]::SendWait('^')"])
            .output()
        {
            Ok(output) if output.status.success() => Ok(()),
            _ => Err("Failed to open Start Menu".to_string()),
        }
    }

    #[cfg(target_os = "linux")]
    {
        // Linux: Open Activities (GNOME) or application menu
        info!("[Linux] Opening Activities");
        match std::process::Command::new("xdotool")
            .args(&["key", "Super_L"])
            .output()
        {
            Ok(output) if output.status.success() => Ok(()),
            _ => Err("Failed to open Activities".to_string()),
        }
    }
}

/// Switch to previous application (Alt+Tab backwards)
pub fn switch_to_previous_app() -> Result<(), String> {
    debug!("[App] Switch previous");

    #[cfg(target_os = "macos")]
    {
        // macOS: Cmd+Tab backwards is Cmd+Shift+Tab
        info!("[macOS] Alt+Tab backward (Cmd+Shift+Tab equivalent)");
        match std::process::Command::new("osascript")
            .arg("-e")
            .arg("tell application \"System Events\" to keystroke tab using {command down, shift down}")
            .output()
        {
            Ok(output) if output.status.success() => Ok(()),
            _ => {
                error!("Failed to switch previous app");
                Err("App switch failed".to_string())
            }
        }
    }

    #[cfg(target_os = "windows")]
    {
        // Windows: Alt+Shift+Tab
        info!("[Windows] Alt+Shift+Tab");
        match std::process::Command::new("cmd")
            .args(&["/C", "powershell -Command Add-Type -AssemblyName System.Windows.Forms; [System.Windows.Forms.SendKeys]::SendWait('%+%(TAB)')"])
            .output()
        {
            Ok(output) if output.status.success() => Ok(()),
            _ => Err("App switch failed".to_string()),
        }
    }

    #[cfg(target_os = "linux")]
    {
        // Linux: Alt+Shift+Tab
        info!("[Linux] Alt+Shift+Tab");
        match std::process::Command::new("xdotool")
            .args(&["key", "alt+shift+Tab"])
            .output()
        {
            Ok(output) if output.status.success() => Ok(()),
            _ => Err("App switch failed".to_string()),
        }
    }
}

/// Switch to next application (Alt+Tab forwards)
pub fn switch_to_next_app() -> Result<(), String> {
    debug!("[App] Switch next");

    #[cfg(target_os = "macos")]
    {
        // macOS: Cmd+Tab
        info!("[macOS] Cmd+Tab");
        match std::process::Command::new("osascript")
            .arg("-e")
            .arg("tell application \"System Events\" to keystroke tab using command down")
            .output()
        {
            Ok(output) if output.status.success() => Ok(()),
            _ => Err("App switch failed".to_string()),
        }
    }

    #[cfg(target_os = "windows")]
    {
        // Windows: Alt+Tab
        info!("[Windows] Alt+Tab");
        match std::process::Command::new("cmd")
            .args(&["/C", "powershell -Command Add-Type -AssemblyName System.Windows.Forms; [System.Windows.Forms.SendKeys]::SendWait('%{TAB}')"])
            .output()
        {
            Ok(output) if output.status.success() => Ok(()),
            _ => Err("App switch failed".to_string()),
        }
    }

    #[cfg(target_os = "linux")]
    {
        // Linux: Alt+Tab
        info!("[Linux] Alt+Tab");
        match std::process::Command::new("xdotool")
            .args(&["key", "alt+Tab"])
            .output()
        {
            Ok(output) if output.status.success() => Ok(()),
            _ => Err("App switch failed".to_string()),
        }
    }
}

/// Show application switcher (hold Alt+Tab)
pub fn show_app_switcher() -> Result<(), String> {
    debug!("[App] Show switcher");

    // This is typically handled by the OS when Alt+Tab is pressed and held
    // We just initiate the Alt+Tab sequence
    switch_to_next_app()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_functions_no_panic() {
        // Just ensure these functions don't panic when called
        // Actual success depends on OS and available tools
        let _ = open_app_launcher();
        let _ = switch_to_next_app();
        let _ = switch_to_previous_app();
    }
}
