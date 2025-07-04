# SVBONY AI Assistant Usage Guide

🇺🇸 [English](https://github.com/dennischancs/svbony-ai-assistant/blob/main/README.md) | 🇫🇷 [Français](https://github.com/dennischancs/svbony-ai-assistant/blob/main/README-FR.md) | 🇩🇪 [Deutsch](https://github.com/dennischancs/svbony-ai-assistant/blob/main/README-DE.md) | 🇮🇹 [Italiano](https://github.com/dennischancs/svbony-ai-assistant/blob/main/README-IT.md) | 🇪🇸 [Español](https://github.com/dennischancs/svbony-ai-assistant/blob/main/README-ES.md) | 🇷🇺 [Русский](https://github.com/dennischancs/svbony-ai-assistant/blob/main/README-RU.md) | 🇵🇹 [Português](https://github.com/dennischancs/svbony-ai-assistant/blob/main/README-PT.md) | 🇯🇵 [日本語](https://github.com/dennischancs/svbony-ai-assistant/blob/main/README-JP.md) | 🇨🇳 [简体中文](https://github.com/dennischancs/svbony-ai-assistant/blob/main/README-CN.md)

### Quick Start

1. [Download the appropriate binary from GitHub Releases](https://github.com/dennischancs/svbony-ai-assistant/releases/latest) for your platform, eg: macOS (x86_64, aarch64/Apple Silicon), Windows (x86_64)
2. Extract the archive
3. Run the binary

## Introduction
The SVBONY AI Assistant is a tool designed to monitor the AI button press events on the SVBONY SVHub Omni2P device and execute configured actions such as opening URLs, running commands, or displaying notifications. This tool supports Windows, macOS systems and can be configured to start automatically on system boot.

## Features
- Monitor AI button press events on the SVBONY SVHub Omni2P device.
- Support multiple actions, including opening URLs, running commands, sending keystrokes, and displaying notifications.
- Support running in the background or foreground.
- Support automatic startup on system boot.
- Detailed configuration and logging.

## Installation and Usage

### Installation from Pre-compiled Binaries
If you don't want to compile the code yourself, you can directly download the pre-compiled binaries and follow these steps:
1. Download the binary file suitable for your operating system.
2. Extract the downloaded file.
3. Open a terminal or command prompt and navigate to the extracted directory.
4. Run the `svbony-ai-assistant` command to start the program.

### Compilation from Source Code
If you want to compile the program from source code, you can follow these steps:

#### Environment Preparation
Ensure that you have installed the Rust development environment. If not, you can download and install it from the [Rust official website](https://www.rust-lang.org/tools/install).

#### Clone the Code Repository
```bash
git clone https://github.com/dennischancs/svbony-ai-assistant.git
cd svbony-ai-assistant
```

#### Compile the Program
```bash
cargo build --release
```
After compilation, the executable file will be located in the `target/release` directory.

### Run the Program
After compilation, you can run the program using the following command:
```bash
./target/release/svbony-ai-assistant
```

## Command Line Arguments
| Argument | Description |
| ---- | ---- |
| `-f, --foreground` | Run in foreground mode, displaying all log messages in the console and keeping the application attached to the terminal. Suitable for debugging or manual monitoring. |
| `-b, --background` | Run in background mode as a daemon process. The application will detach from the terminal and run silently in the background. This is the default behavior when launched from a GUI. |
| `--enable-autostart` | Configure the application to start automatically when the system boots. This will create the necessary autostart entries for your operating system. |
| `--disable-autostart` | Remove the application from automatic startup. The application will not start automatically when the system boots. |
| `-c, --show-config` | Display the current configuration file path and contents, then exit without starting the monitoring service. |
| `-v, --verbose` | Enable verbose logging output. This will show debug messages and detailed information about device communication. |
| `-q, --quiet` | Run in quiet mode, suppressing all log output except for error messages. |
| `-V, --version` | Display version information. |

### Example Usage
```bash
# Run in foreground mode with verbose logging
./target/release/svbony-ai-assistant --foreground --verbose

# Run in background mode and enable automatic startup
./target/release/svbony-ai-assistant --enable-autostart
./target/release/svbony-ai-assistant

# Display the current configuration
./target/release/svbony-ai-assistant --show-config
```

## Configuration File
The configuration file is used to define the behavior and actions of the application. The configuration file can be located in the following locations:
- **Windows**: `%APPDATA%\SVBONY-AI-Assistant\config.json`
- **macOS**: `~/Library/Application Support/SVBONY-AI-Assistant/config.json`
- Or `config.json` in the same directory as the executable file

If the configuration file does not exist, the application will use the default configuration and create a configuration file in the above location.

### Configuration File Example
```json
{
  "actions": [
    {
      "name": "Open app.notta.ai",
      "action_type": "OpenUrl",
      "parameters": {
        "url": "https://app.notta.ai",
        "command": null,
        "args": null,
        "keys": null,
        "message": null,
        "title": null
      },
      "enabled": true
    },
    {
      "name": "Show AI Assistant Notification",
      "action_type": "ShowNotification",
      "parameters": {
        "url": null,
        "command": null,
        "args": null,
        "keys": null,
        "message": "AI Assistant activated!",
        "title": "SVBONY AI Assistant"
      },
      "enabled": true
    }
  ],
  "settings": {
    "auto_start": true,
    "minimize_to_tray": true,
    "log_level": "info",
    "check_updates": true
  }
}
```

## Automatic Startup Configuration
You can use the `--enable-autostart` and `--disable-autostart` arguments to enable or disable the automatic startup function of the application. For example:
```bash
# Enable automatic startup
./target/release/svbony-ai-assistant --enable-autostart

# Disable automatic startup
./target/release/svbony-ai-assistant --disable-autostart
```

## Troubleshooting
- **Logging**: You can use the `--verbose` argument to enable verbose logging for better troubleshooting.
- **Single Instance Check**: If the application fails to start, it may be because another instance is already running. You can use the `--foreground` argument to start multiple instances for debugging.
- **Configuration File Issues**: If there are issues with the configuration file, you can try deleting the configuration file and restarting the application. The application will use the default configuration and recreate the configuration file.

## Contribution
If you want to contribute to the SVBONY AI Assistant project, please follow these steps:
1. Clone the code repository.
2. Create a new branch.
3. Make modifications and tests.
4. Submit a pull request.

## License
This project is licensed under the MIT License. For details, please refer to the [LICENSE](LICENSE) file.