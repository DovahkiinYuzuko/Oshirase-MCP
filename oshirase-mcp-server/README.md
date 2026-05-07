# Oshirase-MCP Server Setup

[日本語](#日本語)|[English](#english)

## 日本語

### 詳細セットアップ手順

#### 1. ビルドと配布
このプロジェクトは **cargo-dist** を使用して、Windows, macOS, Linux 向けのバイナリを自動配布しています。ソースからビルドする場合は、以下のコマンドを実行してください。
```bash
cargo build --release
```

#### 2. 設定
- **Gemini CLI**: インストール時に設定（Webhook URLなど）を直接入力できます。手動で設定ファイルを編集する必要はありません。
- **その他のツール**: 初回起動後、以下のパスに設定ファイルが自動生成されます：
  - **Windows**: `%APPDATA%\mcp-notifier\config.toml`
  - **macOS/Linux**: `~/.config/mcp-notifier/config.toml`

Discord通知を使用する場合は、`webhook_url` を記入してください。
```toml
[discord]
webhook_url = "あなたのWebhook URL"

[general]
default_title = "Oshirase"
```

#### 3. 各AIエージェントへの登録例
サーバーを起動する際は `--mcp` フラグが必要です。

- **Gemini CLI**: `gemini extensions link .` (プロジェクトルートで実行)
- **Claude Code**: `claude mcp add oshirase -- /path/to/oshirase-mcp --mcp`
- **Cursor**: `mcp.json` にコマンドパスと `--mcp` 引数を追加。

## English

### Detailed Setup Instructions

#### 1. Build (From Source)
Ensure Rust is installed, then run the following command to compile the server:
```bash
cargo build --release
```
*Note: If you are using the distributed version, pre-compiled binaries for each OS are located in the `bin/` directory at the root. The extension uses `bin/run.js` to automatically select and launch the correct binary for your environment.*

#### 2. Configuration (config.toml)
After the first execution, a configuration file will be automatically created at:
- **Windows**: `%APPDATA%\mcp-notifier\config.toml`
- **macOS/Linux**: `~/.config/mcp-notifier/config.toml` (or the standard config path for your OS)

If you plan to use Discord notifications, provide your `webhook_url`:
```toml
[discord]
webhook_url = "YOUR_WEBHOOK_URL"

[general]
default_title = "Oshirase"
```

#### 3. Registering with AI Agents
The `--mcp` flag is required to start the server in MCP mode.

- **Gemini CLI**: Run `gemini extensions link .` in the project root.
- **Claude Code**: `claude mcp add oshirase -- /path/to/oshirase-mcp --mcp`
- **Cursor**: Add the command path and the `--mcp` argument to your `mcp.json` or settings.
