# SVBONY AI Assistant 使用ガイド

🇺🇸 [English](https://github.com/dennischancs/svbony-ai-assistant/blob/main/README.md) | 🇫🇷 [Français](https://github.com/dennischancs/svbony-ai-assistant/blob/main/README-FR.md) | 🇩🇪 [Deutsch](https://github.com/dennischancs/svbony-ai-assistant/blob/main/README-DE.md) | 🇮🇹 [Italiano](https://github.com/dennischancs/svbony-ai-assistant/blob/main/README-IT.md) | 🇪🇸 [Español](https://github.com/dennischancs/svbony-ai-assistant/blob/main/README-ES.md) | 🇷🇺 [Русский](https://github.com/dennischancs/svbony-ai-assistant/blob/main/README-RU.md) | 🇵🇹 [Português](https://github.com/dennischancs/svbony-ai-assistant/blob/main/README-PT.md) | 🇯🇵 [日本語](https://github.com/dennischancs/svbony-ai-assistant/blob/main/README-JP.md) | 🇨🇳 [简体中文](https://github.com/dennischancs/svbony-ai-assistant/blob/main/README-CN.md)

#### クイックスタート

1. [GitHub リリースページ](https://github.com/dennischancs/svbony-ai-assistant/releases/latest) からプラットフォームに合わせたバイナリファイルをダウンロードしてください。例：macOS（x86_64、aarch64/Apple Silicon）、Windows（x86_64）
2. アーカイブを解凍します
3. バイナリファイルを実行します

## 概要
SVBONY AI Assistantは、SVBONY SVHub Omni2P デバイスのAIボタン押下イベントを監視し、URL の開く、コマンドの実行、通知の表示などの設定されたアクションを実行するツールです。このツールは Windows、macOS システムをサポートし、システム起動時に自動的に開始するように設定できます。

## 機能
- SVBONY SVHub Omni2P デバイスのAIボタン押下イベントを監視
- URL の開く、コマンドの実行、キーストロークの送信、通知の表示など複数のアクションをサポート
- バックグラウンドまたはフォアグラウンドでの実行をサポート
- システム起動時の自動起動をサポート
- 詳細な設定とログ記録

## インストールと使用方法

### コンパイル済みバイナリからのインストール
コードを自分でコンパイルしたくない場合は、コンパイル済みバイナリを直接ダウンロードして、以下の手順に従ってください：
1. お使いのオペレーティングシステムに適したバイナリファイルをダウンロードします。
2. ダウンロードしたファイルを解凍します。
3. ターミナルまたはコマンドプロンプトを開き、解凍したディレクトリに移動します。
4. `svbony-ai-assistant` コマンドを実行してプログラムを起動します。

### ソースコードからのコンパイル
ソースコードからプログラムをコンパイルしたい場合は、以下の手順に従ってください：

#### 環境準備
Rust開発環境がインストールされていることを確認してください。インストールされていない場合は、[Rust公式サイト](https://www.rust-lang.org/tools/install)からダウンロードしてインストールできます。

#### コードリポジトリのクローン
```bash
git clone https://github.com/dennischancs/svbony-ai-assistant.git
cd svbony-ai-assistant
```

#### プログラムのコンパイル
```bash
cargo build --release
```
コンパイル後、実行可能ファイルは `target/release` ディレクトリに配置されます。

### プログラムの実行
コンパイル後、以下のコマンドを使用してプログラムを実行できます：
```bash
./target/release/svbony-ai-assistant
```

## コマンドライン引数
| 引数 | 説明 |
| ---- | ---- |
| `-f, --foreground` | フォアグラウンドモードで実行し、すべてのログメッセージをコンソールに表示し、アプリケーションをターミナルに接続したままにします。デバッグや手動監視に適しています。 |
| `-b, --background` | デーモンプロセスとしてバックグラウンドモードで実行します。アプリケーションはターミナルから切り離され、バックグラウンドで静かに実行されます。これは GUI から起動した場合のデフォルトの動作です。 |
| `--enable-autostart` | システム起動時にアプリケーションが自動的に起動するように設定します。これにより、お使いのオペレーティングシステムに必要な自動起動エントリが作成されます。 |
| `--disable-autostart` | アプリケーションを自動起動から削除します。システム起動時にアプリケーションは自動的に起動しません。 |
| `-c, --show-config` | 現在の設定ファイルのパスと内容を表示し、監視サービスを起動せずに終了します。 |
| `-v, --verbose` | 詳細なログ出力を有効にします。これにより、デバッグメッセージとデバイス通信に関する詳細情報が表示されます。 |
| `-q, --quiet` | 静音モードで実行し、エラーメッセージを除くすべてのログ出力を抑制します。 |
| `-V, --version` | バージョン情報を表示します。 |

### 使用例
```bash
# 詳細ログを有効にしてフォアグラウンドモードで実行
./target/release/svbony-ai-assistant --foreground --verbose

# バックグラウンドモードで実行し、自動起動を有効にする
./target/release/svbony-ai-assistant --enable-autostart
./target/release/svbony-ai-assistant

# 現在の設定を表示
./target/release/svbony-ai-assistant --show-config
```

## 設定ファイル
設定ファイルは、アプリケーションの動作とアクションを定義するために使用されます。設定ファイルは以下の場所に配置できます：
- **Windows**: `%APPDATA%\SVBONY-AI-Assistant\config.json`
- **macOS**: `~/Library/Application Support/SVBONY-AI-Assistant/config.json`
- または実行可能ファイルと同じディレクトリの `config.json`

設定ファイルが存在しない場合、アプリケーションはデフォルト設定を使用し、上記の場所に設定ファイルを作成します。

### 設定ファイルの例
```json
{
  "actions": [
    {
      "name": "app.notta.ai を開く",
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
      "name": "AI Assistant 通知を表示",
      "action_type": "ShowNotification",
      "parameters": {
        "url": null,
        "command": null,
        "args": null,
        "keys": null,
        "message": "AI Assistant が有効化されました！",
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

## 自動起動設定
`--enable-autostart` と `--disable-autostart` 引数を使用して、アプリケーションの自動起動機能を有効または無効にできます。例：
```bash
# 自動起動を有効にする
./target/release/svbony-ai-assistant --enable-autostart

# 自動起動を無効にする
./target/release/svbony-ai-assistant --disable-autostart
```

## トラブルシューティング
- **ログ記録**: `--verbose` 引数を使用して詳細なログ記録を有効にし、より良いトラブルシューティングを行えます。
- **単一インスタンスチェック**: アプリケーションの起動に失敗する場合、別のインスタンスが既に実行されている可能性があります。デバッグのために複数のインスタンスを起動するには、`--foreground` 引数を使用できます。
- **設定ファイルの問題**: 設定ファイルに問題がある場合は、設定ファイルを削除してアプリケーションを再起動してみてください。アプリケーションはデフォルト設定を使用し、設定ファイルを再作成します。

## 貢献
SVBONY AI Assistant プロジェクトに貢献したい場合は、以下の手順に従ってください：
1. コードリポジトリをクローンします。
2. 新しいブランチを作成します。
3. 修正とテストを行います。
4. プルリクエストを送信します。

## ライセンス
このプロジェクトは MIT ライセンスの下でライセンスされています。詳細については、[LICENSE](LICENSE) ファイルを参照してください。