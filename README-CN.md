# SVBONY AI Assistant 使用指南

🇺🇸 [English](https://github.com/dennischancs/svbony-ai-assistant/blob/main/README.md) | 🇫🇷 [Français](https://github.com/dennischancs/svbony-ai-assistant/blob/main/README-FR.md) | 🇩🇪 [Deutsch](https://github.com/dennischancs/svbony-ai-assistant/blob/main/README-DE.md) | 🇮🇹 [Italiano](https://github.com/dennischancs/svbony-ai-assistant/blob/main/README-IT.md) | 🇪🇸 [Español](https://github.com/dennischancs/svbony-ai-assistant/blob/main/README-ES.md) | 🇷🇺 [Русский](https://github.com/dennischancs/svbony-ai-assistant/blob/main/README-RU.md) | 🇵🇹 [Português](https://github.com/dennischancs/svbony-ai-assistant/blob/main/README-PT.md) | 🇯🇵 [日本語](https://github.com/dennischancs/svbony-ai-assistant/blob/main/README-JP.md) | 🇨🇳 [简体中文](https://github.com/dennischancs/svbony-ai-assistant/blob/main/README-CN.md)

### 快速开始

1. [从 GitHub Releases 下载适合你平台的二进制文件](https://github.com/dennischancs/svbony-ai-assistant/releases/latest)，如：macOS (x86_64, aarch64/Apple Silicon)、Windows (x86_64)
2. 解压缩文件
3. 运行可执行文件

## 支持的设备
- SVBONY SVHub Omni2P（PID: 0x5053）
- SVBONY SVHub M6（PID: 0x364d）

## 简介
SVBONY AI Assistant 是一款用于监听支持的 SVBONY 设备 AI 按键事件，并根据配置执行相应操作（如打开网址、运行命令、发送按键（占位）、显示通知）的工具。支持 Windows 和 macOS 系统，并可配置为开机自启。

## 功能特性
- 监听 SVBONY SVHub Omni2P 和 M6 设备的 AI 按键事件。
- 支持多种动作：打开网址、运行命令、发送按键（占位）、显示通知。
- 支持后台（守护进程）或前台（带日志）运行。
- 支持自动开机启动（可配置，首次运行如启用会自动设置）。
- 后台模式下单实例检测。
- 支持系统信号优雅退出（Ctrl+C、SIGTERM）。
- 详细的配置和日志。
- 跨平台通知支持（Windows Toast、macOS osascript）。

## 安装与使用

### 使用预编译二进制文件
如不想自行编译代码，可直接下载预编译二进制文件并按以下步骤操作：
1. 下载适合操作系统的二进制文件。
2. 解压下载的文件。
3. 打开终端或命令提示符，进入解压目录。
4. 运行 `svbony-ai-assistant` 启动程序。

### 源码编译
如需自行编译，可按以下步骤：

#### 环境准备
确保已安装 Rust 开发环境。未安装可从 [Rust 官网](https://www.rust-lang.org/tools/install) 下载并安装。

#### 克隆代码仓库
```bash
git clone https://github.com/dennischancs/svbony-ai-assistant.git
cd svbony-ai-assistant
```

#### 编译程序
```bash
cargo build --release
```
编译完成后，可执行文件位于 `target/release` 目录。

### 运行程序
编译后可通过如下命令运行：
```bash
./target/release/svbony-ai-assistant
```

## 命令行参数
| 参数 | 说明 |
| ---- | ---- |
| `-f, --foreground` | 前台模式运行，日志输出到控制台，适合调试或手动监控。 |
| `-b, --background` | 后台（守护进程）模式运行，程序会脱离终端静默运行。GUI 启动时为默认行为。 |
| `--enable-autostart` | 配置程序开机自启，会为操作系统创建自启动项。 |
| `--disable-autostart` | 取消开机自启。 |
| `-c, --show-config` | 显示当前配置文件路径及内容，随后退出。 |
| `-r, --regenerate-config` | 恢复出厂默认配置，原有配置会备份为 config.json.old。 |
| `-v, --verbose` | 启用详细日志，显示调试信息和设备通信细节。 |
| `-q, --quiet` | 静默模式，仅输出错误日志。 |
| `-V, --version` | 显示版本信息。 |

### 使用示例
```bash
# 前台模式并启用详细日志
./target/release/svbony-ai-assistant --foreground --verbose

# 后台模式并启用开机自启
./target/release/svbony-ai-assistant --enable-autostart
./target/release/svbony-ai-assistant

# 显示当前配置
./target/release/svbony-ai-assistant --show-config

# 恢复默认配置
./target/release/svbony-ai-assistant --regenerate-config
```

## 动作类型
- `OpenUrl`：在默认浏览器中打开网址。
- `RunCommand`：运行系统命令（可带参数）。
- `SendKeys`：（占位）模拟按键（尚未实现）。
- `ShowNotification`：显示系统通知（带标题和内容）。

## 配置文件
配置文件用于定义程序行为和动作。配置文件可能位于：
- **Windows**: `%APPDATA%\SVBONY-AI-Assistant\config.json`
- **macOS**: `~/Library/Application Support/SVBONY-AI-Assistant/config.json`
- 或与可执行文件同目录下的 `config.json`

如配置文件不存在，程序会自动生成默认配置。

### 配置文件示例
```json
{
  "actions": [
    {
      "name": "Open app.notta.ai",
      "action_type": "OpenUrl", // 可选：OpenUrl, RunCommand, SendKeys, ShowNotification
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
  },
  "version": "0.1.0"
}
```

### 版本兼容性
配置文件包含一个与应用程序版本匹配的 `version` 字段。升级应用时：
- 若配置文件版本与程序不符，程序会自动：
  1. 备份原有配置为 `config.json.old`
  2. 生成新的默认配置文件
- 这样可确保配置与程序兼容，旧设置可在备份文件中查找。

## 自动启动说明
- 若配置中 `auto_start` 启用，首次运行时程序会自动尝试设置开机自启。
- 也可通过 `--enable-autostart` 和 `--disable-autostart` 手动控制。

## 通知功能
- **Windows**：使用 Toast 通知（PowerShell），如失败则气泡通知。
- **macOS**：使用 `osascript` 系统通知。

## 故障排查
- **日志**：可用 `--verbose` 启用详细日志，便于排查。
- **单实例检测**：后台模式下如启动失败，可能已有实例在运行。可用 `--foreground` 启动多个实例调试。
- **配置文件问题**：如配置异常，可删除配置文件后重启，程序会自动生成默认配置。

## 贡献
如需贡献代码，请按以下流程：
1. 克隆代码仓库。
2. 新建分支。
3. 修改并测试。
4. 提交 Pull Request。

## 许可证
本项目采用 MIT 许可证，详见 [LICENSE](LICENSE) 文件。
