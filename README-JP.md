# SVBONY AI Assistant 使用ガイド

🇺🇸 [English](https://github.com/dennischancs/svbony-ai-assistant/blob/main/README.md) | 🇫🇷 [Français](https://github.com/dennischancs/svbony-ai-assistant/blob/main/README-FR.md) | 🇩🇪 [Deutsch](https://github.com/dennischancs/svbony-ai-assistant/blob/main/README-DE.md) | 🇮🇹 [Italiano](https://github.com/dennischancs/svbony-ai-assistant/blob/main/README-IT.md) | 🇪🇸 [Español](https://github.com/dennischancs/svbony-ai-assistant/blob/main/README-ES.md) | 🇷🇺 [Русский](https://github.com/dennischancs/svbony-ai-assistant/blob/main/README-RU.md) | 🇵🇹 [Português](https://github.com/dennischancs/svbony-ai-assistant/blob/main/README-PT.md) | 🇯🇵 [日本語](https://github.com/dennischancs/svbony-ai-assistant/blob/main/README-JP.md) | 🇨🇳 [简体中文](https://github.com/dennischancs/svbony-ai-assistant/blob/main/README-CN.md)

### クイックスタート

1. [GitHub リリースページ](https://github.com/dennischancs/svbony-ai-assistant/releases/latest) からプラットフォームに合わせたバイナリファイルをダウンロードしてください。例：macOS（x86_64、aarch64/Apple Silicon）、Windows（x86_64）
2. アーカイブを解凍します
3. バイナリファイルを実行します

## 対応デバイス
- SVBONY SVHub Omni2P (PID: 0x5053)
- SVBONY SVHub M6 (PID: 0x364d)

## 概要
SVBONY AI Assistantは、対応しているSVBONYデバイスのAIボタン押下イベントを監視し、URL の開く、コマンドの実行、キーストロークの送信（プレースホルダー）、通知の表示などの設定されたアクションを実行するツールです。このツールは Windows、macOSシステムをサポートし、システム起動時に自動的に開始するように設定できます。

## 機能
- SVBONY SVHub Omni2PとM6デバイスのAIボタン押下イベントを監視
- 複数のアクションをサポート：URLを開く、コマンドの実行、キーストロークの送信（プレースホルダー）、通知の表示
- バックグラウンド（デーモン）またはフォアグラウンド（ログ付き）での実行をサポート
- システム起動時の自動起動をサポート（設定可能、有効にした場合は初回実行時に自動設定）
- バックグラウンドモードでのシングルインスタンスチェック
- システムシグナルによる正常終了をサポート（Ctrl+C、SIGTERM）
- 詳細な設定とログ記録
- クロスプラットフォーム通知サポート（Windows Toast、macOS osascript）

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
| `-r, --regenerate-config` | 設定ファイルを工場出荷時のデフォルト設定にリセットします。システム設定が存在する場合、config.json.oldとして実行可能ファイルディレクトリにバックアップされます。すべてのconfig.jsonファイルが工場出荷時のデフォルト設定にリセットされます。 |
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

# デフォルト設定に戻す
./target/release/svbony-ai-assistant --regenerate-config
```

## アクションタイプ
- `OpenUrl`：デフォルトブラウザでURLを開きます。
- `RunCommand`：オプションの引数を含むシステムコマンドを実行します。
- `SendKeys`：（プレースホルダー）キー押下をシミュレートします（まだ実装されていません）。
- `ShowNotification`：タイトルとメッセージを含むシステム通知を表示します。

## 設定ファイル
設定ファイルは、アプリケーションの動作とアクションを定義するために使用されます。設定ファイルは以下の場所に配置できます：
- **Windows**: `%APPDATA%\SVBONY-AI-Assistant\config.json`
- **macOS**: `~/Library/Application Support/SVBONY-AI-Assistant\config.json`
- または実行可能ファイルと同じディレクトリの `config.json`

設定ファイルが存在しない場合、アプリケーションはデフォルト設定を使用し、上記の場所に設定ファイルを作成します。

### 設定ファイルの例
```json
{
  "actions": [
    {
      "name": "app.notta.ai を開く",
      "action_type": "OpenUrl", // オプション: OpenUrl, RunCommand, SendKeys, ShowNotification
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
  },
  "version": "0.1.0"
}
```

### バージョン互換性
設定ファイルにはアプリケーションバージョンと一致する`version`フィールドが含まれています。アプリケーションをアップグレードする際：

- 設定ファイルのバージョンがアプリケーションのバージョンと一致しない場合、アプリケーションは自動的に：
  1. 既存の設定を`config.json.old`にバックアップします
  2. 工場出荷時のデフォルト設定を使用して新しい設定ファイルを作成します
- これにより、設定とアプリケーションバージョンの互換性が確保されます
- 必要に応じて、バックアップファイルから古い設定を参照できます

## 自動起動設定
- 設定で`auto_start`が有効になっている場合、アプリケーションは初回実行時に自動起動の設定を試みます。
- `--enable-autostart`と`--disable-autostart`を使用して手動で自動起動を有効/無効にすることもできます。

## 通知
- **Windows**：Toast通知（PowerShell）を使用し、バルーン通知にフォールバックします。
- **macOS**：システム通知に`osascript`を使用します。

## トラブルシューティング
- **ログ記録**: `--verbose` 引数を使用して詳細なログ記録を有効にし、より良いトラブルシューティングを行えます。
- **単一インスタンスチェック**: バックグラウンドモードでアプリケーションの起動に失敗する場合、別のインスタンスが既に実行されている可能性があります。デバッグのために複数のインスタンスを起動するには、`--foreground` 引数を使用できます。
- **設定ファイルの問題**: 設定ファイルに問題がある場合は、設定ファイルを削除してアプリケーションを再起動してみてください。アプリケーションはデフォルト設定を使用し、設定ファイルを再作成します。

## 貢献
SVBONY AI Assistant プロジェクトに貢献したい場合は、以下の手順に従ってください：
1. コードリポジトリをクローンします。
2. 新しいブランチを作成します。
3. 修正とテストを行います。
4. プルリクエストを送信します。

## ライセンス
このプロジェクトは MIT ライセンスの下でライセンスされています。詳細については、[LICENSE](LICENSE) ファイルを参照してください。