# SVBONY AI Assistant 使用说明

## 简介
SVBONY AI Assistant 是一款用于监控 SVBONY SVHub Omni2P 设备的 AI 按钮按下事件，并执行如打开 URL、运行命令或显示通知等配置操作的工具。本工具支持在 Windows、macOS系统上运行，并且可以配置为开机自动启动。

## 功能特性
- 监控 SVBONY SVHub Omni2P 设备的 AI 按钮按下事件。
- 支持多种操作，如打开 URL、运行命令、发送按键和显示通知。
- 支持在后台或前台运行。
- 支持开机自动启动。
- 详细的配置和日志记录。

## 安装与使用

### 从预编译二进制文件安装
如果你不想自己编译代码，可以直接下载预编译的二进制文件，然后按照以下步骤操作：
1. 下载适合你操作系统的二进制文件。
2. 解压下载的文件。
3. 打开终端或命令提示符，导航到解压后的目录。
4. 运行 `svbony-ai-assistant` 命令启动程序。

### 从源代码编译
如果你想从源代码开始编译程序，可以按照以下步骤操作：

#### 环境准备
确保你已经安装了 Rust 开发环境。如果还没有安装，可以从 [Rust 官方网站](https://www.rust-lang.org/tools/install) 下载并安装。

#### 克隆代码仓库
```bash
git clone <仓库地址>
cd svbony-ai-assistant
```

#### 编译程序
```bash
cargo build --release
```
编译完成后，可执行文件将位于 `target/release` 目录下。

### 运行程序
编译完成后，你可以使用以下命令运行程序：
```bash
./target/release/svbony-ai-assistant
```

## 命令行参数
| 参数 | 描述 |
| ---- | ---- |
| `-f, --foreground` | 以前台模式运行，将所有日志消息显示在控制台，并保持应用程序与终端连接，适用于调试或手动监控。 |
| `-b, --background` | 以后台模式作为守护进程运行，应用程序将从终端分离并在后台静默运行，这是从 GUI 启动时的默认行为。 |
| `--enable-autostart` | 配置应用程序在系统启动时自动启动，将为你的操作系统创建必要的自动启动条目。 |
| `--disable-autostart` | 从自动启动中移除应用程序，系统启动时应用程序将不会自动启动。 |
| `-c, --show-config` | 显示当前配置文件的路径和内容，然后退出，不启动监控服务。 |
| `-v, --verbose` | 启用详细日志记录，显示调试消息和设备通信的详细信息。 |
| `-q, --quiet` | 以安静模式运行，抑制除错误消息之外的所有日志输出。 |
| `-V, --version` | 显示版本信息。 |

### 示例用法
```bash
# 以前台模式和详细日志记录运行
./target/release/svbony-ai-assistant --foreground --verbose

# 以后台模式运行并启用自动启动
./target/release/svbony-ai-assistant --enable-autostart
./target/release/svbony-ai-assistant

# 显示当前配置
./target/release/svbony-ai-assistant --show-config
```

## 配置文件
配置文件用于定义应用程序的行为和操作。配置文件可以位于以下位置：
- **Windows**：`%APPDATA%\SVBONY-AI-Assistant\config.json`
- **macOS**：`~/Library/Application Support/SVBONY-AI-Assistant/config.json`
- 或者与可执行文件位于同一目录下的 `config.json`

如果配置文件不存在，将使用默认配置并在上述位置创建配置文件。

### 配置文件示例
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

## 自动启动配置
你可以使用 `--enable-autostart` 和 `--disable-autostart` 参数来启用或禁用应用程序的自动启动功能。例如：
```bash
# 启用自动启动
./target/release/svbony-ai-assistant --enable-autostart

# 禁用自动启动
./target/release/svbony-ai-assistant --disable-autostart
```

## 故障排除
- **日志记录**：可以使用 `--verbose` 参数启用详细日志记录，以便更好地排查问题。
- **单实例检查**：如果应用程序无法启动，可能是因为已经有另一个实例正在运行。可以使用 `--foreground` 参数启动多个实例进行调试。
- **配置文件问题**：如果配置文件存在问题，可以尝试删除配置文件，然后重新启动应用程序，应用程序将使用默认配置并重新创建配置文件。

## 贡献
如果你想为 SVBONY AI Assistant 项目做出贡献，请遵循以下步骤：
1. 克隆代码仓库。
2. 创建一个新的分支。
3. 进行修改和测试。
4. 提交拉取请求。

## 许可证
本项目采用 MIT 许可证，详情请参阅 [LICENSE](LICENSE) 文件。