use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub actions: Vec<ActionConfig>,
    pub settings: AppSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionConfig {
    pub name: String,
    pub action_type: ActionType,
    pub parameters: ActionParameters,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionType {
    OpenUrl,
    RunCommand,
    SendKeys,
    ShowNotification,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionParameters {
    pub url: Option<String>,
    pub command: Option<String>,
    pub args: Option<Vec<String>>,
    pub keys: Option<String>,
    pub message: Option<String>,
    pub title: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub auto_start: bool,
    pub minimize_to_tray: bool,
    pub log_level: String,
    pub check_updates: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            actions: vec![
                ActionConfig {
                    name: "Open app.notta.ai".to_string(),
                    action_type: ActionType::OpenUrl,
                    parameters: ActionParameters {
                        url: Some("https://app.notta.ai".to_string()),
                        command: None,
                        args: None,
                        keys: None,
                        message: None,
                        title: None,
                    },
                    enabled: true,
                },
                ActionConfig {
                    name: "Show AI Assistant Notification".to_string(),
                    action_type: ActionType::ShowNotification,
                    parameters: ActionParameters {
                        url: None,
                        command: None,
                        args: None,
                        keys: None,
                        message: Some("AI Assistant activated!".to_string()),
                        title: Some("SVBONY AI Assistant".to_string()),
                    },
                    enabled: true,
                },
            ],
            settings: AppSettings::default(),
        }
    }
}

impl Default for AppSettings {
    fn default() -> Self {
        AppSettings {
            auto_start: true,
            minimize_to_tray: true,
            log_level: "info".to_string(),
            check_updates: true,
        }
    }
}

impl Config {
    pub fn get_config_path() -> Result<PathBuf> {
        let config_dir = dirs::config_dir()
            .context("Failed to get user config directory")?
            .join("SVBONY-AI-Assistant");

        if !config_dir.exists() {
            fs::create_dir_all(&config_dir)
                .context("Failed to create config directory")?;
        }

        Ok(config_dir.join("config.json"))
    }

    pub fn load_or_create_default() -> Result<Self> {
        // 尝试获取 exe 所在目录的 config.json 路径
        let exe_path = std::env::current_exe()?;
        let exe_dir = exe_path.parent().unwrap_or_else(|| exe_path.as_path());
        let local_config_path = exe_dir.join("config.json");

        let config_path = Self::get_config_path()?;

        if local_config_path.exists() {
            return Self::load_from_file(&local_config_path);
        } else if config_path.exists() {
            return Self::load_from_file(&config_path);
        } else {
            let default_config = Self::default();
            // 在默认目录生成配置文件
            default_config.save_to_file(&config_path)?;
            // 在程序同目录生成配置文件
            default_config.save_to_file(&local_config_path)?;
            return Ok(default_config);
        }
    }

    pub fn load_from_file(path: &PathBuf) -> Result<Self> {
        let content = fs::read_to_string(path)
            .context("Failed to read config file")?;

        let config: Config = serde_json::from_str(&content)
            .context("Failed to parse config file")?;

        Ok(config)
    }

    pub fn save_to_file(&self, path: &PathBuf) -> Result<()> {
        let content = serde_json::to_string_pretty(self)
            .context("Failed to serialize config")?;

        fs::write(path, content)
            .context("Failed to write config file")?;

        Ok(())
    }

    pub fn save(&self) -> Result<()> {
        let config_path = Self::get_config_path()?;
        self.save_to_file(&config_path)
    }

    pub fn add_action(&mut self, action: ActionConfig) -> Result<()> {
        self.actions.push(action);
        self.save()
    }

    pub fn remove_action(&mut self, index: usize) -> Result<()> {
        if index < self.actions.len() {
            self.actions.remove(index);
            self.save()
        } else {
            Err(anyhow::anyhow!("Action index out of bounds"))
        }
    }

    pub fn update_action(&mut self, index: usize, action: ActionConfig) -> Result<()> {
        if index < self.actions.len() {
            self.actions[index] = action;
            self.save()
        } else {
            Err(anyhow::anyhow!("Action index out of bounds"))
        }
    }

    pub fn toggle_action(&mut self, index: usize) -> Result<()> {
        if index < self.actions.len() {
            self.actions[index].enabled = !self.actions[index].enabled;
            self.save()
        } else {
            Err(anyhow::anyhow!("Action index out of bounds"))
        }
    }
}