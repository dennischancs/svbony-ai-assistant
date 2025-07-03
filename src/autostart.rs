use anyhow::Result;
use log::info;
use std::fs;
use std::path::PathBuf;
use anyhow::Context;

pub struct AutostartManager;

impl AutostartManager {
    pub fn new() -> Self {
        AutostartManager
    }

    /// Check if autostart is enabled
    pub fn is_enabled(&self) -> Result<bool> {
        #[cfg(target_os = "windows")]
        {
            self.is_enabled_windows()
        }
        #[cfg(target_os = "macos")]
        {
            self.is_enabled_macos()
        }
        #[cfg(target_os = "linux")]
        {
            self.is_enabled_linux()
        }
    }

    /// Enable autostart
    pub fn enable(&self) -> Result<()> {
        info!("Enabling autostart");
        
        #[cfg(target_os = "windows")]
        {
            self.enable_windows()
        }
        #[cfg(target_os = "macos")]
        {
            self.enable_macos()
        }
        #[cfg(target_os = "linux")]
        {
            self.enable_linux()
        }
    }

    /// Disable autostart
    pub fn disable(&self) -> Result<()> {
        info!("Disabling autostart");
        
        #[cfg(target_os = "windows")]
        {
            self.disable_windows()
        }
        #[cfg(target_os = "macos")]
        {
            self.disable_macos()
        }
        #[cfg(target_os = "linux")]
        {
            self.disable_linux()
        }
    }

    /// Setup autostart on first run if enabled in config
    pub fn setup_first_run(&self, enabled: bool) -> Result<()> {
        if enabled && !self.is_enabled()? {
            info!("Setting up autostart on first run");
            self.enable()?;
        } else if !enabled && self.is_enabled()? {
            info!("Disabling autostart as requested");
            self.disable()?;
        }
        Ok(())
    }

    #[cfg(target_os = "windows")]
    fn is_enabled_windows(&self) -> Result<bool> {
        use winapi::um::winreg::{RegOpenKeyExA, RegQueryValueExA, RegCloseKey, HKEY_CURRENT_USER};
        use winapi::um::winnt::{KEY_READ, REG_SZ};
        use std::ffi::CString;
        use std::ptr;

        let subkey = CString::new("Software\\Microsoft\\Windows\\CurrentVersion\\Run")?;
        let value_name = CString::new("SVBONY-AI-Assistant")?;
        
        let mut hkey = ptr::null_mut();
        
        unsafe {
            let result = RegOpenKeyExA(
                HKEY_CURRENT_USER,
                subkey.as_ptr(),
                0,
                KEY_READ,
                &mut hkey
            );
            
            if result != 0 {
                return Ok(false);
            }
            
            let mut data_type = 0u32;
            let mut data_size = 0u32;
            
            let query_result = RegQueryValueExA(
                hkey,
                value_name.as_ptr(),
                ptr::null_mut(),
                &mut data_type,
                ptr::null_mut(),
                &mut data_size
            );
            
            RegCloseKey(hkey);
            
            Ok(query_result == 0 && data_type == REG_SZ)
        }
    }

    #[cfg(target_os = "windows")]
    fn enable_windows(&self) -> Result<()> {
        use winapi::um::winreg::{RegOpenKeyExA, RegSetValueExA, RegCloseKey, HKEY_CURRENT_USER};
        use winapi::um::winnt::{KEY_WRITE, REG_SZ};
        use std::ffi::CString;
        use std::ptr;

        let exe_path = std::env::current_exe()?;
        let exe_path_str = exe_path.to_string_lossy();
        let startup_command = format!("\"{}\" --background", exe_path_str);
        
        let subkey = CString::new("Software\\Microsoft\\Windows\\CurrentVersion\\Run")?;
        let value_name = CString::new("SVBONY-AI-Assistant")?;
        let value_data = CString::new(startup_command)?;
        
        let mut hkey = ptr::null_mut();
        
        unsafe {
            let result = RegOpenKeyExA(
                HKEY_CURRENT_USER,
                subkey.as_ptr(),
                0,
                KEY_WRITE,
                &mut hkey
            );
            
            if result != 0 {
                return Err(anyhow::anyhow!("Failed to open registry key"));
            }
            
            let set_result = RegSetValueExA(
                hkey,
                value_name.as_ptr(),
                0,
                REG_SZ,
                value_data.as_ptr() as *const u8,
                value_data.as_bytes().len() as u32 + 1
            );
            
            RegCloseKey(hkey);
            
            if set_result != 0 {
                return Err(anyhow::anyhow!("Failed to set registry value"));
            }
        }
        
        Ok(())
    }

    #[cfg(target_os = "windows")]
    fn disable_windows(&self) -> Result<()> {
        use winapi::um::winreg::{RegOpenKeyExA, RegDeleteValueA, RegCloseKey, HKEY_CURRENT_USER};
        use winapi::um::winnt::KEY_WRITE;
        use std::ffi::CString;
        use std::ptr;

        let subkey = CString::new("Software\\Microsoft\\Windows\\CurrentVersion\\Run")?;
        let value_name = CString::new("SVBONY-AI-Assistant")?;
        
        let mut hkey = ptr::null_mut();
        
        unsafe {
            let result = RegOpenKeyExA(
                HKEY_CURRENT_USER,
                subkey.as_ptr(),
                0,
                KEY_WRITE,
                &mut hkey
            );
            
            if result != 0 {
                return Ok(()); // Key doesn't exist, already disabled
            }
            
            RegDeleteValueA(hkey, value_name.as_ptr());
            RegCloseKey(hkey);
        }
        
        Ok(())
    }

    #[cfg(target_os = "macos")]
    fn is_enabled_macos(&self) -> Result<bool> {
        let plist_path = self.get_macos_plist_path()?;
        Ok(plist_path.exists())
    }

    #[cfg(target_os = "macos")]
    fn enable_macos(&self) -> Result<()> {
        let exe_path = std::env::current_exe()?;
        let plist_path = self.get_macos_plist_path()?;
        
        let plist_content = format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>com.svbony.ai-assistant</string>
    <key>ProgramArguments</key>
    <array>
        <string>{}</string>
        <string>--background</string>
    </array>
    <key>RunAtLoad</key>
    <true/>
    <key>KeepAlive</key>
    <false/>
</dict>
</plist>"#,
            exe_path.to_string_lossy()
        );

        // Create LaunchAgents directory if it doesn't exist
        if let Some(parent) = plist_path.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(&plist_path, plist_content)?;
        
        // Load the plist using the correct launchctl syntax for modern macOS
        let output = std::process::Command::new("launchctl")
            .args(&["bootstrap", "gui/501", plist_path.to_str().unwrap()])
            .output();

        match output {
            Ok(result) => {
                if !result.status.success() {
                    let stderr = String::from_utf8_lossy(&result.stderr);
                    // If it's already loaded, that's OK
                    if !stderr.contains("already loaded") {
                        return Err(anyhow::anyhow!("Failed to load plist: {}", stderr));
                    }
                }
            }
            Err(_) => {
                // Fallback to legacy launchctl load command
                let output = std::process::Command::new("launchctl")
                    .args(&["load", plist_path.to_str().unwrap()])
                    .output()?;
                
                if !output.status.success() {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    if !stderr.contains("already loaded") {
                        return Err(anyhow::anyhow!("Failed to load plist (legacy): {}", stderr));
                    }
                }
            }
        }
        
        Ok(())
    }

    #[cfg(target_os = "macos")]
    fn disable_macos(&self) -> Result<()> {
        let plist_path = self.get_macos_plist_path()?;
        
        if plist_path.exists() {
            let label = "com.svbony.ai-assistant";
            
            // Try modern launchctl bootout command first
            let output = std::process::Command::new("launchctl")
                .args(&["bootout", "gui/501", label])
                .output();

            match output {
                Ok(result) => {
                    if !result.status.success() {
                        let stderr = String::from_utf8_lossy(&result.stderr);
                        // If service is not loaded, that's OK
                        if !stderr.contains("No such process") && !stderr.contains("not exist") {
                            // Try with plist path instead of label
                            let output2 = std::process::Command::new("launchctl")
                                .args(&["bootout", "gui/501", plist_path.to_str().unwrap()])
                                .output()?;
                            
                            if !output2.status.success() {
                                let stderr2 = String::from_utf8_lossy(&output2.stderr);
                                if !stderr2.contains("No such process") && !stderr2.contains("not exist") {
                                    return Err(anyhow::anyhow!("Failed to unload plist: {}", stderr2));
                                }
                            }
                        }
                    }
                }
                Err(_) => {
                    // Fallback to legacy launchctl unload command
                    let output = std::process::Command::new("launchctl")
                        .args(&["unload", plist_path.to_str().unwrap()])
                        .output()?;
                    
                    if !output.status.success() {
                        let stderr = String::from_utf8_lossy(&output.stderr);
                        // If service is not loaded, that's OK
                        if !stderr.contains("No such process") && !stderr.contains("not exist") {
                            return Err(anyhow::anyhow!("Failed to unload plist (legacy): {}", stderr));
                        }
                    }
                }
            }
            
            // Remove the plist file
            fs::remove_file(&plist_path)?;
        }
        
        Ok(())
    }

    #[cfg(target_os = "macos")]
    fn get_macos_plist_path(&self) -> Result<PathBuf> {
        let home_dir = dirs::home_dir()
            .context("Failed to get home directory")?;
        
        Ok(home_dir
            .join("Library")
            .join("LaunchAgents")
            .join("com.svbony.ai-assistant.plist"))
    }

    #[cfg(target_os = "linux")]
    fn is_enabled_linux(&self) -> Result<bool> {
        let desktop_file_path = self.get_linux_desktop_file_path()?;
        Ok(desktop_file_path.exists())
    }

    #[cfg(target_os = "linux")]
    fn enable_linux(&self) -> Result<()> {
        let exe_path = std::env::current_exe()?;
        let desktop_file_path = self.get_linux_desktop_file_path()?;
        
        let desktop_content = format!(
            r#"[Desktop Entry]
Type=Application
Name=SVBONY AI Assistant
Comment=SVBONY AI Assistant - Monitor AI button and execute configured actions
Exec={} --background
Icon=application-default-icon
Hidden=false
NoDisplay=false
X-GNOME-Autostart-enabled=true
StartupNotify=false"#,
            exe_path.to_string_lossy()
        );

        // Create autostart directory if it doesn't exist
        if let Some(parent) = desktop_file_path.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(&desktop_file_path, desktop_content)?;
        
        Ok(())
    }

    #[cfg(target_os = "linux")]
    fn disable_linux(&self) -> Result<()> {
        let desktop_file_path = self.get_linux_desktop_file_path()?;
        
        if desktop_file_path.exists() {
            fs::remove_file(&desktop_file_path)?;
        }
        
        Ok(())
    }

    #[cfg(target_os = "linux")]
    fn get_linux_desktop_file_path(&self) -> Result<PathBuf> {
        let config_dir = dirs::config_dir()
            .context("Failed to get config directory")?;
        
        Ok(config_dir
            .join("autostart")
            .join("svbony-ai-assistant.desktop"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_autostart_manager_creation() {
        let manager = AutostartManager::new();
        // Just test that we can create the manager
        assert!(true);
    }

    // Note: Integration tests for autostart functionality would require
    // system-level permissions and are better tested manually
}