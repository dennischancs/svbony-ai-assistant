use anyhow::{Context, Result};
use log::{debug, error, info, warn};
use libc;

use std::path::PathBuf;
use std::fs;
#[cfg(any(target_os = "macos", target_os = "linux"))]
use nix::unistd::Pid;
#[cfg(any(target_os = "macos", target_os = "linux"))]
use nix::sys::signal::{kill, Signal};

pub struct BackgroundService {
    is_background: bool,
    pid_file: Option<PathBuf>,
}

impl BackgroundService {
    pub fn new(background: bool) -> Result<Self> {
        let pid_file = if background {
            Some(Self::get_pid_file_path()?)
        } else {
            None
        };

        Ok(BackgroundService {
            is_background: background,
            pid_file,
        })
    }

    pub fn is_background(&self) -> bool {
        self.is_background
    }

    /// Check if another instance is already running
    pub fn check_single_instance(&self) -> Result<bool> {
        if let Some(pid_file) = &self.pid_file {
            if pid_file.exists() {
                match fs::read_to_string(pid_file) {
                    Ok(pid_str) => {
                        if let Ok(pid) = pid_str.trim().parse::<u32>() {
                            if self.is_process_running(pid) {
                                warn!("Another instance is already running with PID: {}", pid);
                                return Ok(false);
                            } else {
                                debug!("Stale PID file found, removing it");
                                let _ = fs::remove_file(pid_file);
                            }
                        }
                    }
                    Err(e) => {
                        debug!("Failed to read PID file: {}", e);
                        let _ = fs::remove_file(pid_file);
                    }
                }
            }
        }
        Ok(true)
    }

    /// Start background service
    pub fn start(&self) -> Result<()> {
        if !self.is_background {
            return Ok(());
        }

        info!("Starting background service");

        // Create PID file
        if let Some(pid_file) = &self.pid_file {
            let current_pid = std::process::id();
            
            // Create directory if it doesn't exist
            if let Some(parent) = pid_file.parent() {
                fs::create_dir_all(parent)?;
            }
            
            fs::write(pid_file, current_pid.to_string())
                .context("Failed to create PID file")?;
            
            info!("Created PID file at: {:?} with PID: {}", pid_file, current_pid);
        }

        // Setup logging for background mode
        self.setup_background_logging()?;

        Ok(())
    }

    /// Stop background service
    pub fn stop(&self) -> Result<()> {
        if let Some(pid_file) = &self.pid_file {
            if pid_file.exists() {
                fs::remove_file(pid_file)
                    .context("Failed to remove PID file")?;
                info!("Removed PID file");
            }
        }
        Ok(())
    }

    /// Setup logging for background mode
    fn setup_background_logging(&self) -> Result<()> {
        if !self.is_background {
            return Ok(());
        }

        let log_dir = Self::get_log_directory()?;
        if !log_dir.exists() {
            fs::create_dir_all(&log_dir)?;
        }

        let log_file = log_dir.join("svbony-ai-assistant.log");
        info!("Background mode: logs will be written to {:?}", log_file);

        Ok(())
    }

    /// Check if a process with given PID is running
    fn is_process_running(&self, pid: u32) -> bool {
        #[cfg(target_os = "windows")]
        {
            self.is_process_running_windows(pid)
        }
        #[cfg(any(target_os = "macos", target_os = "linux"))]
        {
            self.is_process_running_unix(pid)
        }
    }

    #[cfg(target_os = "windows")]
    fn is_process_running_windows(&self, pid: u32) -> bool {
        use winapi::um::processthreadsapi::OpenProcess;
        use winapi::um::winnt::PROCESS_QUERY_INFORMATION;
        use winapi::um::handleapi::CloseHandle;
        use std::ptr;

        unsafe {
            let handle = OpenProcess(PROCESS_QUERY_INFORMATION, 0, pid);
            if handle != ptr::null_mut() {
                CloseHandle(handle);
                true
            } else {
                false
            }
        }
    }

    #[cfg(any(target_os = "macos", target_os = "linux"))]
    fn is_process_running_unix(&self, pid: u32) -> bool {
        // More reliable implementation using nix crate
        kill(Pid::from_raw(pid as i32), None).is_ok()
    }

    /// Get PID file path
    fn get_pid_file_path() -> Result<PathBuf> {
        let runtime_dir = Self::get_runtime_directory()?;
        Ok(runtime_dir.join("svbony-ai-assistant.pid"))
    }

    /// Get runtime directory for PID files
    fn get_runtime_directory() -> Result<PathBuf> {
        #[cfg(target_os = "linux")]
        {
            // Try XDG_RUNTIME_DIR first, fallback to /tmp
            if let Ok(xdg_runtime) = std::env::var("XDG_RUNTIME_DIR") {
                Ok(PathBuf::from(xdg_runtime).join("svbony-ai-assistant"))
            } else {
                Ok(PathBuf::from("/tmp/svbony-ai-assistant"))
            }
        }
        #[cfg(target_os = "macos")]
        {
            let home_dir = dirs::home_dir()
                .context("Failed to get home directory")?;
            Ok(home_dir.join(".local/var/run/svbony-ai-assistant"))
        }
        #[cfg(target_os = "windows")]
        {
            let temp_dir = std::env::temp_dir();
            Ok(temp_dir.join("svbony-ai-assistant"))
        }
    }

    /// Get log directory
    fn get_log_directory() -> Result<PathBuf> {
        #[cfg(target_os = "linux")]
        {
            let home_dir = dirs::home_dir()
                .context("Failed to get home directory")?;
            Ok(home_dir.join(".local/share/svbony-ai-assistant/logs"))
        }
        #[cfg(target_os = "macos")]
        {
            let home_dir = dirs::home_dir()
                .context("Failed to get home directory")?;
            Ok(home_dir.join("Library/Logs/SVBONY-AI-Assistant"))
        }
        #[cfg(target_os = "windows")]
        {
            let data_dir = dirs::data_local_dir()
                .context("Failed to get local data directory")?;
            Ok(data_dir.join("SVBONY-AI-Assistant/logs"))
        }
    }

    /// Send signal to running instance
    pub fn send_signal_to_running_instance(&self, signal: &str) -> Result<()> {
        if let Some(pid_file) = &self.pid_file {
            if pid_file.exists() {
                let pid_str = fs::read_to_string(pid_file)?;
                let pid: u32 = pid_str.trim().parse()
                    .context("Invalid PID in file")?;

                info!("Sending {} signal to PID: {}", signal, pid);

                #[cfg(any(target_os = "macos", target_os = "linux"))]
                {
                    std::process::Command::new("kill")
                        .args(&["-TERM", &pid.to_string()])
                        .output()?;
                }

                #[cfg(target_os = "windows")]
                {
                    // On Windows, we could use taskkill, but for now just log
                    warn!("Signal sending not implemented on Windows");
                }
            }
        }
        Ok(())
    }
}

impl Drop for BackgroundService {
    fn drop(&mut self) {
        if let Err(e) = self.stop() {
            error!("Failed to cleanup background service: {}", e);
        }
    }
}

/// Daemonize the process on Unix systems
#[cfg(any(target_os = "macos", target_os = "linux"))]
pub fn daemonize() -> Result<()> {
    info!("Daemonizing process");

    // Don't fork on macOS for GUI applications - it can cause issues with system permissions
    #[cfg(target_os = "macos")]
    {
        // On macOS, just redirect standard streams without forking
        // This is better for GUI applications that need to access system resources
        redirect_standard_streams()?;
        return Ok(());
    }

    #[cfg(target_os = "linux")]
    {
        // Fork the process
        unsafe {
            let pid = libc::fork();
            
            if pid < 0 {
                return Err(anyhow::anyhow!("Fork failed"));
            } else if pid > 0 {
                // Parent process exits
                std::process::exit(0);
            }
        }

        // Child continues here
        // Create new session
        unsafe {
            if libc::setsid() < 0 {
                return Err(anyhow::anyhow!("setsid failed"));
            }
        }

        // Change working directory to root
        std::env::set_current_dir("/")?;

        // Redirect file descriptors
        redirect_standard_streams()?;
    }

    Ok(())
}

/// Redirect standard streams to /dev/null (Unix) or equivalent
#[cfg(any(target_os = "macos", target_os = "linux"))]
fn redirect_standard_streams() -> Result<()> {
    unsafe {
        // Close original descriptors
        libc::close(0); // stdin
        libc::close(1); // stdout
        libc::close(2); // stderr

        // Open /dev/null for stdin
        let dev_null = libc::open("/dev/null\0".as_ptr() as *const i8, libc::O_RDWR);
        if dev_null < 0 {
            return Err(anyhow::anyhow!("Failed to open /dev/null"));
        }
        libc::dup2(dev_null, 0);
        libc::dup2(dev_null, 1);
        libc::dup2(dev_null, 2);
        if dev_null > 2 {
            libc::close(dev_null);
        }
    }

    Ok(())
}

/// Windows service placeholder - would need windows-service crate for full implementation
#[cfg(target_os = "windows")]
pub fn daemonize() -> Result<()> {
    // On Windows, we just run in background without true service installation
    // For a true Windows service, we'd need to implement the windows-service traits
    info!("Running in background mode on Windows");
    Ok(())
}