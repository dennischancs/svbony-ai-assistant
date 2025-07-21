use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use log::{info, debug, warn};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub actions: Vec<ActionConfig>,
    pub settings: AppSettings,
    pub version: String,
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
            version: env!("CARGO_PKG_VERSION").to_string(),
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
    
    pub fn get_exe_config_path() -> Result<PathBuf> {
        let exe_path = std::env::current_exe()?;
        let exe_dir = exe_path.parent().unwrap_or_else(|| exe_path.as_path());
        Ok(exe_dir.join("config.json"))
    }
    
    /// 备份并重置配置文件为出厂设置
    /// 每个目录的配置文件各自备份为config.json.old，然后用默认配置覆盖
    pub fn backup_and_reset_to_factory() -> Result<Self> {
        // 获取系统配置路径
        let system_config_path = Self::get_config_path()?;
        let system_backup_path = system_config_path.with_file_name("config.json.old");
        
        // 获取可执行文件目录配置路径
        let exe_config_path = Self::get_exe_config_path()?;
        let exe_backup_path = exe_config_path.with_file_name("config.json.old");
        
        // 创建默认配置
        let default_config = Self::default();
        
        // 处理系统配置目录文件
        if system_config_path.exists() {
            info!("系统配置文件存在，备份到 {:?}", system_backup_path);
            if let Err(e) = fs::copy(&system_config_path, &system_backup_path) {
                warn!("备份系统配置文件失败: {}", e);
            }
            info!("使用出厂设置覆盖系统配置文件");
            default_config.save_to_file(&system_config_path)?;
        } else {
            info!("系统配置文件不存在，创建默认配置");
            default_config.save_to_file(&system_config_path)?;
        }
        
        // 处理可执行文件目录配置文件
        if exe_config_path.exists() {
            info!("可执行文件目录配置文件存在，备份到 {:?}", exe_backup_path);
            if let Err(e) = fs::copy(&exe_config_path, &exe_backup_path) {
                warn!("备份可执行文件目录配置文件失败: {}", e);
            }
            info!("使用出厂设置覆盖可执行文件目录配置文件");
            default_config.save_to_file(&exe_config_path)?;
        } else {
            info!("可执行文件目录配置文件不存在，创建默认配置");
            default_config.save_to_file(&exe_config_path)?;
        }
        
        info!("所有配置文件已备份并重置为出厂设置");
        Ok(default_config)
    }

    pub fn load_or_create_default() -> Result<Self> {
        // 优先尝试加载系统配置目录的配置文件
        let system_config_path = Self::get_config_path()?;
        let exe_config_path = Self::get_exe_config_path()?;
        
        if system_config_path.exists() {
            // 尝试加载系统配置文件
            match Self::load_from_file(&system_config_path) {
                Ok(config) => {
                    // 配置文件版本匹配，可以直接使用
                    info!("使用系统配置目录的配置文件: {:?}", system_config_path);
                    return Ok(config);
                },
                Err(e) => {
                    // 配置文件版本不匹配或解析错误
                    info!("系统配置文件加载失败: {}", e);
                    info!("将备份并使用出厂设置");
                    return Self::backup_and_reset_to_factory();
                }
            }
        }
        
        if exe_config_path.exists() {
            // 尝试加载可执行文件目录配置文件
            match Self::load_from_file(&exe_config_path) {
                Ok(config) => {
                    // 配置文件版本匹配，可以直接使用
                    info!("使用可执行文件目录的配置文件: {:?}", exe_config_path);
                    return Ok(config);
                },
                Err(e) => {
                    // 配置文件版本不匹配或解析错误
                    info!("可执行文件目录配置文件加载失败: {}", e);
                    info!("将备份并使用出厂设置");
                    return Self::backup_and_reset_to_factory();
                }
            }
        }
        
        // 两个配置文件都不存在或无法加载，创建默认配置
        info!("配置文件不存在，创建默认配置");
        let default_config = Self::default();
        
        // 在两个目录都创建默认配置文件
        default_config.save_to_file(&system_config_path)?;
        default_config.save_to_file(&exe_config_path)?;
        
        return Ok(default_config);
    }

    pub fn load_from_file(path: &PathBuf) -> Result<Self> {
        let content = fs::read_to_string(path)
            .context("Failed to read config file")?;

        // 尝试解析配置文件
        match serde_json::from_str::<Config>(&content) {
            Ok(config) => {
                // 检查版本字段是否为空或与当前程序版本不匹配
                if config.version.is_empty() || config.version != env!("CARGO_PKG_VERSION") {
                    return Err(anyhow::anyhow!("配置文件版本不匹配或缺失"));
                }
                Ok(config)
            },
            Err(e) => {
                // 无法解析配置文件，返回错误
                Err(anyhow::anyhow!("无法解析配置文件: {}", e))
            }
        }
    }

    pub fn save_to_file(&self, path: &PathBuf) -> Result<()> {
        let content = serde_json::to_string_pretty(self)
            .context("Failed to serialize config")?;

        fs::write(path, content)
            .context("Failed to write config file")?;

        Ok(())
    }

    pub fn save(&self) -> Result<()> {
        // 保存到系统配置目录
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