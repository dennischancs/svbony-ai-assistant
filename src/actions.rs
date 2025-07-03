use anyhow::{Context, Result};
use log::{debug, error, info, warn};
use std::process::Stdio;
use tokio::process::Command;

use crate::config::{ActionConfig, ActionType};

pub struct ActionExecutor {
    // Future: Could hold state for more complex actions
}

impl ActionExecutor {
    pub fn new() -> Self {
        ActionExecutor {}
    }

    pub async fn execute_action(&self, action: &ActionConfig) -> Result<()> {
        if !action.enabled {
            debug!("Action '{}' is disabled, skipping", action.name);
            return Ok(());
        }

        info!("Executing action: {}", action.name);

        match action.action_type {
            ActionType::OpenUrl => {
                self.open_url(action).await
            }
            ActionType::RunCommand => {
                self.run_command(action).await
            }
            ActionType::SendKeys => {
                self.send_keys(action).await
            }
            ActionType::ShowNotification => {
                self.show_notification(action).await
            }
        }
    }

    async fn open_url(&self, action: &ActionConfig) -> Result<()> {
        let url = action.parameters.url.as_ref()
            .context("URL parameter is required for OpenUrl action")?;

        info!("Opening URL: {}", url);

        // Use the 'open' crate for cross-platform URL opening
        match open::that(url) {
            Ok(_) => {
                info!("Successfully opened URL: {}", url);
                Ok(())
            }
            Err(e) => {
                error!("Failed to open URL {}: {}", url, e);
                Err(anyhow::anyhow!("Failed to open URL: {}", e))
            }
        }
    }

    async fn run_command(&self, action: &ActionConfig) -> Result<()> {
        let command = action.parameters.command.as_ref()
            .context("Command parameter is required for RunCommand action")?;

        let args = action.parameters.args.as_ref().cloned().unwrap_or_default();

        info!("Running command: {} {:?}", command, args);

        let mut cmd = Command::new(command);
        cmd.args(&args);
        cmd.stdout(Stdio::null());
        cmd.stderr(Stdio::null());

        // Windows specific: Hide console window
        #[cfg(target_os = "windows")]
        {
            
            const CREATE_NO_WINDOW: u32 = 0x08000000;
            const DETACHED_PROCESS: u32 = 0x00000008;
            cmd.creation_flags(CREATE_NO_WINDOW | DETACHED_PROCESS);
        }

        match cmd.spawn() {
            Ok(mut child) => {
                // Don't wait for the process to complete for most commands
                // This allows opening applications without blocking
                tokio::spawn(async move {
                    match child.wait().await {
                        Ok(status) => {
                            if status.success() {
                                debug!("Command completed successfully");
                            } else {
                                warn!("Command exited with status: {}", status);
                            }
                        }
                        Err(e) => {
                            error!("Error waiting for command: {}", e);
                        }
                    }
                });

                Ok(())
            }
            Err(e) => {
                error!("Failed to run command {}: {}", command, e);
                Err(anyhow::anyhow!("Failed to run command: {}", e))
            }
        }
    }

    async fn send_keys(&self, action: &ActionConfig) -> Result<()> {
        let keys = action.parameters.keys.as_ref()
            .context("Keys parameter is required for SendKeys action")?;

        info!("Sending keys: {}", keys);

        // This is a placeholder implementation
        // In a real implementation, you would use platform-specific APIs
        // or a cross-platform crate like `enigo` for key simulation
        warn!("SendKeys action is not yet implemented");
        
        // For now, just log what keys would be sent
        info!("Would send keys: {}", keys);
        
        Ok(())
    }

    async fn show_notification(&self, action: &ActionConfig) -> Result<()> {
        let default_title = "SVBONY AI Assistant".to_string();
        let title = action.parameters.title.as_ref()
            .unwrap_or(&default_title);
        let message = action.parameters.message.as_ref()
            .context("Message parameter is required for ShowNotification action")?;

        info!("Showing notification: {} - {}", title, message);

        // Cross-platform notification implementation
        self.show_system_notification(title, message).await
    }

    #[cfg(target_os = "windows")]
    async fn show_system_notification(&self, title: &str, message: &str) -> Result<()> {
        // Use Windows Toast notifications with hidden console
        let mut cmd = Command::new("powershell");
        cmd.args(&[
            "-WindowStyle", "Hidden",
            "-NoProfile",
            "-ExecutionPolicy", "Bypass",
            "-Command",
            &format!(
                "[Windows.UI.Notifications.ToastNotificationManager, Windows.UI.Notifications, ContentType = WindowsRuntime] > $null; \
                 [Windows.UI.Notifications.ToastNotification, Windows.UI.Notifications, ContentType = WindowsRuntime] > $null; \
                 [Windows.Data.Xml.Dom.XmlDocument, Windows.Data.Xml.Dom.XmlDocument, ContentType = WindowsRuntime] > $null; \
                 $APP_ID = 'SVBONY.AI.Assistant'; \
                 $template = @'\
<toast>\
    <visual>\
        <binding template=\"ToastGeneric\">\
            <text>{}</text>\
            <text>{}</text>\
        </binding>\
    </visual>\
</toast>\
'@; \
                 $xml = New-Object Windows.Data.Xml.Dom.XmlDocument; \
                 $xml.LoadXml($template); \
                 $toast = New-Object Windows.UI.Notifications.ToastNotification $xml; \
                 [Windows.UI.Notifications.ToastNotificationManager]::CreateToastNotifier($APP_ID).Show($toast)",
                title, message
            )
        ]);

        // Hide console window
        cmd.stdout(Stdio::null());
        cmd.stderr(Stdio::null());
        
        // Windows specific: Hide console window
        
        const CREATE_NO_WINDOW: u32 = 0x08000000;
        const DETACHED_PROCESS: u32 = 0x00000008;
        cmd.creation_flags(CREATE_NO_WINDOW | DETACHED_PROCESS);

        match cmd.spawn() {
            Ok(mut child) => {
                // Don't wait too long for notification
                tokio::spawn(async move {
                    let _ = child.wait().await;
                });
                info!("Toast notification triggered successfully");
                Ok(())
            }
            Err(e) => {
                error!("Failed to show toast notification: {}", e);
                // Fallback to simple balloon notification
                self.show_fallback_notification_windows(title, message).await
            }
        }
    }

    #[cfg(target_os = "windows")]
    async fn show_fallback_notification_windows(&self, title: &str, message: &str) -> Result<()> {
        let mut cmd = Command::new("powershell");
        cmd.args(&[
            "-WindowStyle", "Hidden",
            "-NoProfile",
            "-ExecutionPolicy", "Bypass",
            "-Command",
            &format!(
                "Add-Type -AssemblyName System.Windows.Forms; \
                 $notify = New-Object System.Windows.Forms.NotifyIcon; \
                 $notify.Icon = [System.Drawing.SystemIcons]::Information; \
                 $notify.Visible = $true; \
                 $notify.ShowBalloonTip(3000, '{}', '{}', [System.Windows.Forms.ToolTipIcon]::Info); \
                 Start-Sleep -Seconds 4; \
                 $notify.Dispose()",
                title, message
            )
        ]);

        cmd.stdout(Stdio::null());
        cmd.stderr(Stdio::null());
        
        
        const CREATE_NO_WINDOW: u32 = 0x08000000;
        const DETACHED_PROCESS: u32 = 0x00000008;
        cmd.creation_flags(CREATE_NO_WINDOW | DETACHED_PROCESS);

        match cmd.spawn() {
            Ok(mut child) => {
                tokio::spawn(async move {
                    let _ = child.wait().await;
                });
                info!("Fallback notification shown successfully");
                Ok(())
            }
            Err(e) => {
                error!("Failed to show fallback notification: {}", e);
                Err(anyhow::anyhow!("Failed to show notification: {}", e))
            }
        }
    }

    #[cfg(target_os = "macos")]
    async fn show_system_notification(&self, title: &str, message: &str) -> Result<()> {
        let mut cmd = Command::new("osascript");
        cmd.args(&[
            "-e",
            &format!("display notification \"{}\" with title \"{}\"", message, title)
        ]);

        cmd.stdout(Stdio::null());
        cmd.stderr(Stdio::null());

        match cmd.spawn() {
            Ok(mut child) => {
                tokio::spawn(async move {
                    let _ = child.wait().await;
                });
                info!("Notification shown successfully");
                Ok(())
            }
            Err(e) => {
                error!("Failed to show notification: {}", e);
                Err(anyhow::anyhow!("Failed to show notification: {}", e))
            }
        }
    }

    #[cfg(target_os = "linux")]
    async fn show_system_notification(&self, title: &str, message: &str) -> Result<()> {
        let mut cmd = Command::new("notify-send");
        cmd.args(&[title, message]);
        cmd.stdout(Stdio::null());
        cmd.stderr(Stdio::null());

        match cmd.spawn() {
            Ok(mut child) => {
                tokio::spawn(async move {
                    let _ = child.wait().await;
                });
                info!("Notification shown successfully");
                Ok(())
            }
            Err(e) => {
                // Fallback: try zenity
                let mut zenity_cmd = Command::new("zenity");
                zenity_cmd.args(&[
                    "--info",
                    "--title", title,
                    "--text", message
                ]);
                zenity_cmd.stdout(Stdio::null());
                zenity_cmd.stderr(Stdio::null());

                match zenity_cmd.spawn() {
                    Ok(mut child) => {
                        tokio::spawn(async move {
                            let _ = child.wait().await;
                        });
                        info!("Notification shown successfully via zenity");
                        Ok(())
                    }
                    Err(zenity_e) => {
                        error!("Failed to show notification via notify-send or zenity: {}, {}", e, zenity_e);
                        Err(anyhow::anyhow!("Failed to show notification: {}", e))
                    }
                }
            }
        }
    }
}