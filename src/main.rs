use anyhow::{Context, Result};
use hidapi::{HidApi, HidDevice};
use log::{debug, error, info, warn};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::time::sleep;
use tokio::signal;

mod config;
mod hid_monitor;
mod actions;
mod cli;
mod background;
mod autostart;

use config::Config;
use hid_monitor::HidMonitor;
use actions::ActionExecutor;
use cli::CliArgs;

use autostart::AutostartManager;

// SVBONY SVHub device identifiers
const VENDOR_ID: u16 = 0xe2b7;
// PID for M6 and Omni2P devices
const M6_PRODUCT_ID: u16 = 0x364d; // SiBiChi
const OMNI2P_PRODUCT_ID: u16 = 0x5053; // YSAIR

// Expected HID data pattern for AI button
const AI_BUTTON_PATTERN: &[u8] = &[
    0x04, 0xb2, 0x00, 0x00, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00
];

#[derive(Debug, Clone)]
pub struct AppState {
    pub config: Arc<Mutex<Config>>,
    pub is_running: Arc<Mutex<bool>>,
    pub hid_monitor: Arc<Mutex<Option<HidMonitor>>>,
    pub cli_args: Arc<CliArgs>,
}

impl AppState {
    pub fn new(cli_args: CliArgs) -> Result<Self> {
        let config = Config::load_or_create_default()?;

        Ok(AppState {
            config: Arc::new(Mutex::new(config)),
            is_running: Arc::new(Mutex::new(true)),
            hid_monitor: Arc::new(Mutex::new(None)),
            cli_args: Arc::new(cli_args),
        })
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Windows specific: Hide console window as early as possible
    #[cfg(target_os = "windows")]
    {
        // Hide console window immediately if running in background
        let args: Vec<String> = std::env::args().collect();
        let is_background = args.contains(&"--background".to_string()) || 
                           args.contains(&"-b".to_string()) ||
                           (!args.contains(&"--foreground".to_string()) && 
                            !args.contains(&"-f".to_string()) && 
                            !args.contains(&"--help".to_string()) &&
                            !args.contains(&"-h".to_string()));
        
        if is_background {
            unsafe {
                use winapi::um::wincon::{GetConsoleWindow, FreeConsole};
                use winapi::um::winuser::{ShowWindow, SW_HIDE};
                
                let console_window = GetConsoleWindow();
                if !console_window.is_null() {
                    ShowWindow(console_window, SW_HIDE);
                    FreeConsole();
                }
            }
        }
    }

    // Parse command line arguments
    let cli_args = CliArgs::parse()
        .context("Failed to parse command line arguments")?;

    // Setup logging based on CLI arguments
    cli_args.setup_logging()
        .context("Failed to setup logging")?;

    // Handle special commands that don't require the main loop
    if cli_args.handle_special_commands().await? {
        return Ok(());
    }

    info!("Starting SVBONY AI Assistant v{}", env!("CARGO_PKG_VERSION"));

    // Create background service
    let background_service = cli_args.create_background_service()
        .context("Failed to create background service")?;

    // Check for single instance if running in background
    if background_service.is_background() {
        if !background_service.check_single_instance()? {
            error!("Another instance of SVBONY AI Assistant is already running.");
            error!("Use --foreground to run multiple instances for debugging.");
            std::process::exit(1);
        }
    }

    // Start background service if needed
    background_service.start()
        .context("Failed to start background service")?;

    // Initialize application state
    let app_state = AppState::new(cli_args)
        .context("Failed to initialize application state")?;
    
    // Log configuration version information
    {
        let config_guard = app_state.config.lock().unwrap();
        info!("配置文件版本: {}", config_guard.version);
    }

    // Setup autostart on first run if enabled in config
    setup_autostart_on_first_run(&app_state).await?;

    // Additional Windows console hiding in background mode
    if background_service.is_background() {
        info!("Running in background mode");
        #[cfg(any(target_os = "macos", target_os = "linux"))]
        {
            if let Err(e) = background::daemonize() {
                warn!("Failed to daemonize: {}, continuing anyway", e);
            }
        }
        #[cfg(target_os = "windows")]
        {
            // Additional console window hiding for Windows
            unsafe {
                use winapi::um::wincon::{GetConsoleWindow, FreeConsole}; use winapi::um::consoleapi::AllocConsole;
                use winapi::um::winuser::{ShowWindow, SW_HIDE};
                use winapi::um::processenv::SetStdHandle;
                use winapi::um::winbase::{STD_OUTPUT_HANDLE, STD_ERROR_HANDLE, STD_INPUT_HANDLE};
                use std::ptr;
                
                let console_window = GetConsoleWindow();
                if !console_window.is_null() {
                    ShowWindow(console_window, SW_HIDE);
                }
                
                // Disconnect from console completely
                FreeConsole();
                
                // Redirect standard handles to null
                SetStdHandle(STD_INPUT_HANDLE, ptr::null_mut());
                SetStdHandle(STD_OUTPUT_HANDLE, ptr::null_mut());
                SetStdHandle(STD_ERROR_HANDLE, ptr::null_mut());
            }
            background::daemonize()?;
        }
    } else {
        info!("Running in foreground mode");
    }

    // Initialize HID monitor with both PIDs
    let hid_monitor = HidMonitor::new_multi_pid(VENDOR_ID, vec![M6_PRODUCT_ID, OMNI2P_PRODUCT_ID])
        .context("Failed to initialize HID monitor")?;

    {
        let mut monitor_guard = app_state.hid_monitor.lock().unwrap();
        *monitor_guard = Some(hid_monitor);
    }

    info!("SVBONY AI Assistant started successfully");
    info!("Monitoring for SVBONY devices (VID: {:04x}, PIDs: [{:04x}, {:04x}])", 
           VENDOR_ID, M6_PRODUCT_ID, OMNI2P_PRODUCT_ID);

    // Show additional info in foreground mode
    if !background_service.is_background() {
        info!("Press Ctrl+C to stop the application");
        info!("Use --help for more options");
    }

    // Setup signal handlers
    let app_state_for_signals = app_state.clone();
    tokio::spawn(async move {
        setup_signal_handlers(app_state_for_signals).await;
    });

    // Main application loop
    let app_state_clone = app_state.clone();
    let monitor_task = tokio::spawn(async move {
        monitor_hid_device(app_state_clone).await;
    });

    // Keep the application running
    loop {
        let is_running = {
            let running_guard = app_state.is_running.lock().unwrap();
            *running_guard
        };

        if !is_running {
            info!("Application shutdown requested");
            break;
        }

        sleep(Duration::from_millis(100)).await;
    }

    // Wait for monitor task to complete
    let _ = monitor_task.await;

    // Cleanup background service
    background_service.stop()
        .context("Failed to stop background service")?;

    info!("SVBONY AI Assistant shut down successfully");
    Ok(())
}

async fn setup_signal_handlers(app_state: AppState) {
    #[cfg(unix)]
    {
        let mut sigterm = signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to setup SIGTERM handler");
        let mut sigint = signal::unix::signal(signal::unix::SignalKind::interrupt())
            .expect("Failed to setup SIGINT handler");

        tokio::select! {
            _ = sigterm.recv() => {
                info!("Received SIGTERM, shutting down gracefully");
            }
            _ = sigint.recv() => {
                info!("Received SIGINT, shutting down gracefully");
            }
        }
    }

    #[cfg(windows)]
    {
        match signal::ctrl_c().await {
            Ok(_) => {
                info!("Received Ctrl+C, shutting down gracefully");
            }
            Err(e) => {
                error!("Failed to setup Ctrl+C handler: {}", e);
                return;
            }
        }
    }

    // Signal shutdown
    {
        let mut running_guard = app_state.is_running.lock().unwrap();
        *running_guard = false;
    }
}

async fn setup_autostart_on_first_run(app_state: &AppState) -> Result<()> {
    let should_setup_autostart = {
        let config_guard = app_state.config.lock().unwrap();
        config_guard.settings.auto_start
    };

    if should_setup_autostart {
        let autostart_manager = AutostartManager::new();
        
        // Only setup if not already enabled
        match autostart_manager.is_enabled() {
            Ok(false) => {
                info!("Setting up autostart (first run with auto_start=true)");
                if let Err(e) = autostart_manager.enable() {
                    warn!("Failed to enable autostart: {}", e);
                } else {
                    info!("Autostart enabled successfully");
                }
            }
            Ok(true) => {
                debug!("Autostart already enabled");
            }
            Err(e) => {
                warn!("Failed to check autostart status: {}", e);
            }
        }
    }

    Ok(())
}

async fn monitor_hid_device(app_state: AppState) {
    let mut last_connection_attempt = std::time::Instant::now();
    let mut device: Option<HidDevice> = None;
    let action_executor = ActionExecutor::new();

    loop {
        let is_running = {
            let running_guard = app_state.is_running.lock().unwrap();
            *running_guard
        };

        if !is_running {
            break;
        }

        // Try to connect to device if not connected
        if device.is_none() && last_connection_attempt.elapsed() > Duration::from_secs(5) {
            match connect_to_device().await {
                Ok(hid_device) => {
                    info!("Connected to SVBONY device");
                    device = Some(hid_device);
                }
                Err(e) => {
                    debug!("Device not found or connection failed: {}", e);
                    last_connection_attempt = std::time::Instant::now();
                }
            }
        }

        // Monitor device if connected
        if let Some(ref mut hid_device) = device {
            match monitor_device_input(hid_device, &app_state, &action_executor).await {
                Ok(_) => {}
                Err(e) => {
                    warn!("Device monitoring error: {}", e);
                    device = None; // Reset connection on error
                    last_connection_attempt = std::time::Instant::now();
                }
            }
        }

        sleep(Duration::from_millis(10)).await;
    }

    info!("HID monitoring stopped");
}

async fn connect_to_device() -> Result<HidDevice> {
    let api = HidApi::new()
        .context("Failed to initialize HID API")?;

    // Try to connect to M6 first
    let device = match api.open(VENDOR_ID, M6_PRODUCT_ID) {
        Ok(device) => {
            info!("Connected to SVBONY M6 device (PID: {:04x})", M6_PRODUCT_ID);
            device
        },
        Err(_) => {
            // If M6 not found, try Omni2P
            match api.open(VENDOR_ID, OMNI2P_PRODUCT_ID) {
                Ok(device) => {
                    info!("Connected to SVBONY Omni2P device (PID: {:04x})", OMNI2P_PRODUCT_ID);
                    device
                },
                Err(e) => {
                    return Err(anyhow::anyhow!("Failed to open any SVBONY device: {}", e));
                }
            }
        }
    };

    // Set non-blocking mode
    device.set_blocking_mode(false)
        .context("Failed to set non-blocking mode")?;

    Ok(device)
}

async fn monitor_device_input(
    device: &mut HidDevice,
    app_state: &AppState,
    action_executor: &ActionExecutor,
) -> Result<()> {
    let mut buffer = [0u8; 1024]; // 1K buffer as specified

    match device.read_timeout(&mut buffer, 10) {
        Ok(size) if size > 0 => {
            // Log HID data in foreground mode or verbose mode
            let should_log_data = {
                let cli_args = &app_state.cli_args;
                !cli_args.should_run_in_background() || cli_args.verbose
            };

            if should_log_data {
                debug!("Received HID data: {} bytes", size);
                debug!("Data: {:02x?}", &buffer[..std::cmp::min(size, 32)]);
            }

            // Check if this matches our AI button pattern
            if size >= AI_BUTTON_PATTERN.len() &&
               buffer[..AI_BUTTON_PATTERN.len()] == *AI_BUTTON_PATTERN {
                info!("AI button pressed detected!");

                // Execute configured actions
                let config = {
                    let config_guard = app_state.config.lock().unwrap();
                    config_guard.clone()
                };

                for action in &config.actions {
                    if let Err(e) = action_executor.execute_action(action).await {
                        error!("Failed to execute action {}: {}", action.name, e);
                    }
                }
            }
        }
        Ok(_) => {
            // No data available, this is normal in non-blocking mode
        }
        Err(e) => {
            return Err(anyhow::anyhow!("HID read error: {}", e));
        }
    }

    Ok(())
}