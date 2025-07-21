use clap::{Arg, ArgAction, Command};
use anyhow::{Context, Result};
use log::{info};

use crate::config::Config;
use crate::autostart::AutostartManager;
use crate::background::BackgroundService;

/// Command line interface for SVBONY AI Assistant
#[derive(Debug, Default)]
pub struct CliArgs {
    pub foreground: bool,
    pub background: bool,
    pub help_requested: bool,
    pub enable_autostart: Option<bool>,
    pub disable_autostart: bool,
    pub show_config: bool,
    pub regenerate_config: bool,
    pub version: bool,
    pub verbose: bool,
    pub quiet: bool,
}

impl CliArgs {
    /// Parse command line arguments
    pub fn parse() -> Result<Self> {
        let matches = Command::new("svbony-ai-assistant")
            .version(env!("CARGO_PKG_VERSION"))
            .author(env!("CARGO_PKG_AUTHORS"))
            .about("SVBONY AI Assistant - Monitor AI button and execute configured actions")
            .long_about("SVBONY AI Assistant monitors your SVBONY SVHub Omni2P device for AI button presses and executes configured actions like opening URLs, running commands, or showing notifications.")
            .arg(
                Arg::new("foreground")
                    .short('f')
                    .long("foreground")
                    .help("Run in foreground mode with console output")
                    .long_help("Run the application in foreground mode. This will display all log messages in the console and keep the application attached to the terminal. Useful for debugging or manual monitoring.")
                    .action(ArgAction::SetTrue)
            )
            .arg(
                Arg::new("background")
                    .short('b')
                    .long("background")
                    .help("Run in background mode (daemon)")
                    .long_help("Run the application in background mode as a daemon process. The application will detach from the terminal and run silently in the background. This is the default behavior when launched from a GUI.")
                    .action(ArgAction::SetTrue)
            )
            .arg(
                Arg::new("enable-autostart")
                    .long("enable-autostart")
                    .help("Enable automatic startup on system boot")
                    .long_help("Configure the application to start automatically when the system boots. This will create the necessary autostart entries for your operating system.")
                    .action(ArgAction::SetTrue)
            )
            .arg(
                Arg::new("disable-autostart")
                    .long("disable-autostart")
                    .help("Disable automatic startup on system boot")
                    .long_help("Remove the application from automatic startup. The application will not start automatically when the system boots.")
                    .action(ArgAction::SetTrue)
            )
            .arg(
                Arg::new("show-config")
                    .short('c')
                    .long("show-config")
                    .help("Show current configuration and exit")
                    .long_help("Display the current configuration file path and contents, then exit without starting the monitoring service.")
                    .action(ArgAction::SetTrue)
            )
            .arg(
                Arg::new("regenerate-config")
                    .short('r')
                    .long("regenerate-config")
                    .help("Regenerate default configuration files, overwriting any existing files")
                    .long_help("Create new default configuration files in both system default directory and executable directory, overwriting any existing files with the same name.")
                    .action(ArgAction::SetTrue)
            )
            .arg(
                Arg::new("verbose")
                    .short('v')
                    .long("verbose")
                    .help("Enable verbose logging")
                    .long_help("Enable verbose logging output. This will show debug messages and detailed information about device communication.")
                    .action(ArgAction::SetTrue)
            )
            .arg(
                Arg::new("quiet")
                    .short('q')
                    .long("quiet")
                    .help("Suppress all output except errors")
                    .long_help("Run in quiet mode, suppressing all log output except for error messages.")
                    .action(ArgAction::SetTrue)
            )
            .arg(
                Arg::new("version")
                    .short('V')
                    .long("version")
                    .help("Show version information")
                    .action(ArgAction::SetTrue)
            )
            .get_matches();

        let mut args = CliArgs::default();

        args.foreground = matches.get_flag("foreground");
        args.background = matches.get_flag("background");
        args.enable_autostart = if matches.get_flag("enable-autostart") { Some(true) } else { None };
        args.disable_autostart = matches.get_flag("disable-autostart");
        args.show_config = matches.get_flag("show-config");
        args.regenerate_config = matches.get_flag("regenerate-config");
        args.version = matches.get_flag("version");
        args.verbose = matches.get_flag("verbose");
        args.quiet = matches.get_flag("quiet");

        // Validate conflicting arguments
        if args.foreground && args.background {
            return Err(anyhow::anyhow!("Cannot specify both --foreground and --background"));
        }

        if args.verbose && args.quiet {
            return Err(anyhow::anyhow!("Cannot specify both --verbose and --quiet"));
        }

        if args.enable_autostart.is_some() && args.disable_autostart {
            return Err(anyhow::anyhow!("Cannot specify both --enable-autostart and --disable-autostart"));
        }

        Ok(args)
    }

    /// Determine if we should run in background mode
    pub fn should_run_in_background(&self) -> bool {
        if self.foreground {
            false
        } else {
            // Default to background mode unless explicitly running in foreground
            true
        }
    }

    /// Check if the application was launched from a terminal
    fn is_launched_from_terminal(&self) -> bool {
        #[cfg(unix)]
        {
            // On Unix systems, check if stdout is a TTY
            use std::os::unix::io::AsRawFd;
            unsafe {
                libc::isatty(std::io::stdout().as_raw_fd()) == 1
            }
        }
        #[cfg(windows)]
        {
            // On Windows, check if we have a console
            use winapi::um::wincon::GetConsoleWindow;
            unsafe {
                GetConsoleWindow() != std::ptr::null_mut()
            }
        }
    }

    /// Setup logging based on CLI arguments
    pub fn setup_logging(&self) -> Result<()> {
        let log_level = if self.verbose {
            "debug"
        } else if self.quiet {
            "error"
        } else {
            "info"
        };

        let mut builder = env_logger::Builder::from_env(
            env_logger::Env::default().default_filter_or(log_level)
        );

        // In background mode, we might want to configure file logging
        if self.should_run_in_background() {
            // For background mode, we'll set up file logging later in BackgroundService
            builder.target(env_logger::Target::Stdout);
        } else {
            // For foreground mode, log to stderr so stdout can be used for other output
            builder.target(env_logger::Target::Stderr);
        }

        builder.init();
        Ok(())
    }

    /// Handle special commands that don't require the main application loop
    pub async fn handle_special_commands(&self) -> Result<bool> {
        if self.version {
            self.show_version();
            return Ok(true);
        }

        if self.show_config {
            self.show_configuration().await?;
            return Ok(true);
        }
        
        if self.regenerate_config {
            self.regenerate_configuration().await?;
            return Ok(true);
        }

        if let Some(enable) = self.enable_autostart {
            self.configure_autostart(enable).await?;
            return Ok(true);
        }

        if self.disable_autostart {
            self.configure_autostart(false).await?;
            return Ok(true);
        }

        Ok(false)
    }

    /// Show version information
    fn show_version(&self) {
        println!("SVBONY AI Assistant v{}", env!("CARGO_PKG_VERSION"));
        println!("Built with Rust {}", std::env::var("RUSTC_VERSION").unwrap_or_else(|_| "unknown".to_string()));
        println!("Target: {}", std::env::var("TARGET").unwrap_or_else(|_| "unknown".to_string()));
        
        #[cfg(debug_assertions)]
        println!("Build: Debug");
        #[cfg(not(debug_assertions))]
        println!("Build: Release");

        println!();
        println!("Authors: {}", env!("CARGO_PKG_AUTHORS"));
        println!("License: {}", env!("CARGO_PKG_LICENSE"));
        println!("Description: {}", env!("CARGO_PKG_DESCRIPTION"));
    }

    /// Show current configuration
    async fn show_configuration(&self) -> Result<()> {
        println!("SVBONY AI Assistant Configuration");
        println!("================================");
        println!();

        // Show config file path
        let config_path = Config::get_config_path()?;
        println!("Configuration file: {}", config_path.display());
        
        // Check if config file exists
        if !config_path.exists() {
            println!("Status: Configuration file does not exist (will be created with defaults)");
            println!();
            
            let default_config = Config::default();
            self.print_config(&default_config);
        } else {
            println!("Status: Configuration file exists");
            println!();
            
            match Config::load_or_create_default() {
                Ok(config) => {
                    self.print_config(&config);
                }
                Err(e) => {
                    println!("Error loading configuration: {}", e);
                    return Err(e);
                }
            }
        }

        // Show autostart status
        println!();
        println!("Autostart Configuration");
        println!("======================");
        
        let autostart_manager = AutostartManager::new();
        match autostart_manager.is_enabled() {
            Ok(enabled) => {
                println!("Autostart: {}", if enabled { "Enabled" } else { "Disabled" });
            }
            Err(e) => {
                println!("Autostart: Error checking status - {}", e);
            }
        }

        Ok(())
    }

    /// Print configuration details
    fn print_config(&self, config: &Config) {
        println!("Application Settings");
        println!("-------------------");
        println!("Auto start: {}", config.settings.auto_start);
        println!("Minimize to tray: {}", config.settings.minimize_to_tray);
        println!("Log level: {}", config.settings.log_level);
        println!("Check updates: {}", config.settings.check_updates);
        println!();

        println!("Configured Actions ({} total)", config.actions.len());
        println!("------------------");
        for (i, action) in config.actions.iter().enumerate() {
            println!("{}. {} [{}]", 
                i + 1, 
                action.name, 
                if action.enabled { "Enabled" } else { "Disabled" }
            );
            println!("   Type: {:?}", action.action_type);
            
            match action.action_type {
                crate::config::ActionType::OpenUrl => {
                    if let Some(url) = &action.parameters.url {
                        println!("   URL: {}", url);
                    }
                }
                crate::config::ActionType::RunCommand => {
                    if let Some(command) = &action.parameters.command {
                        println!("   Command: {}", command);
                        if let Some(args) = &action.parameters.args {
                            println!("   Arguments: {:?}", args);
                        }
                    }
                }
                crate::config::ActionType::SendKeys => {
                    if let Some(keys) = &action.parameters.keys {
                        println!("   Keys: {}", keys);
                    }
                }
                crate::config::ActionType::ShowNotification => {
                    if let Some(title) = &action.parameters.title {
                        println!("   Title: {}", title);
                    }
                    if let Some(message) = &action.parameters.message {
                        println!("   Message: {}", message);
                    }
                }
            }
            println!();
        }
    }

    /// Configure autostart
    async fn configure_autostart(&self, enable: bool) -> Result<()> {
        let autostart_manager = AutostartManager::new();
        
        if enable {
            info!("Enabling autostart...");
            autostart_manager.enable()
                .context("Failed to enable autostart")?;
            println!("Autostart has been enabled. The application will start automatically on system boot.");
        } else {
            info!("Disabling autostart...");
            autostart_manager.disable()
                .context("Failed to disable autostart")?;
            println!("Autostart has been disabled. The application will not start automatically on system boot.");
        }

        Ok(())
    }

    /// Regenerate default configuration files
    async fn regenerate_configuration(&self) -> Result<()> {
        println!("正在备份和重置配置文件为出厂设置...");
        
        match Config::backup_and_reset_to_factory() {
            Ok(_) => {
                println!("配置文件操作成功:");
                println!("- 如果配置文件存在，已备份为config.json.old");
                println!("- 所有配置文件已重置为出厂设置");
            }
            Err(e) => {
                println!("重置配置文件时出错: {}", e);
                return Err(e);
            }
        }
        
        Ok(())
    }

    /// Create a BackgroundService based on CLI arguments
    pub fn create_background_service(&self) -> Result<BackgroundService> {
        BackgroundService::new(self.should_run_in_background())
    }

    /// Show help for daemon management
    pub fn show_daemon_help(&self) {
        println!("Daemon Management");
        println!("================");
        println!();
        println!("To run in foreground (with console output):");
        println!("  svbony-ai-assistant --foreground");
        println!();
        println!("To run in background (daemon mode):");
        println!("  svbony-ai-assistant --background");
        println!();
        println!("To enable autostart:");
        println!("  svbony-ai-assistant --enable-autostart");
        println!();
        println!("To disable autostart:");
        println!("  svbony-ai-assistant --disable-autostart");
        println!();
        println!("To check configuration:");
        println!("  svbony-ai-assistant --show-config");
        println!();
        println!("For verbose output:");
        println!("  svbony-ai-assistant --verbose --foreground");
    }
}

/// Helper function to check if another instance is running
pub fn check_single_instance() -> Result<bool> {
    // This will be implemented using the BackgroundService functionality
    let background_service = BackgroundService::new(true)?;
    background_service.check_single_instance()
}

/// Show usage examples
pub fn show_usage_examples() {
    println!("Usage Examples");
    println!("==============");
    println!();
    println!("Basic usage (run in background):");
    println!("  svbony-ai-assistant");
    println!();
    println!("Debug mode (run in foreground with verbose logging):");
    println!("  svbony-ai-assistant --foreground --verbose");
    println!();
    println!("Setup autostart and run:");
    println!("  svbony-ai-assistant --enable-autostart");
    println!("  svbony-ai-assistant");
    println!();
    println!("Check current configuration:");
    println!("  svbony-ai-assistant --show-config");
    println!();
    println!("Regenerate default configuration files:");
    println!("  svbony-ai-assistant --regenerate-config");
    println!();
    println!("Run quietly (errors only):");
    println!("  svbony-ai-assistant --quiet");
    println!();
    println!("Configuration file locations:");
    
    #[cfg(target_os = "windows")]
    println!("  Windows: %APPDATA%\\SVBONY-AI-Assistant\\config.json");
    
    #[cfg(target_os = "macos")]
    println!("  macOS: ~/Library/Application Support/SVBONY-AI-Assistant/config.json");
    
    #[cfg(target_os = "linux")]
    println!("  Linux: ~/.config/SVBONY-AI-Assistant/config.json");
    
    println!("  Or: ./config.json (in the same directory as the executable)");
}