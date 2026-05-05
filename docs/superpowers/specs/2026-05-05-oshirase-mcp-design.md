# Oshirase-MCP 設計ドキュメント

## 1. プロジェクト概要
本プロジェクトは、Model Context Protocol (MCP) に準拠した通知管理サーバーである。
OS標準のデスクトップ通知およびDiscordへのリッチな通知（Embed）を提供し、AIエージェントからの呼び出しだけでなく、単体CLIとしても動作する「二刀流」設計とする。

## 2. 動作モード
バイナリ（`oshirase-mcp`）は、起動引数によって以下の2つのモードで動作する。

### 2.1. MCPモード
- **起動方法**: 引数なし、または `--mcp` フラグ。
- **通信**: 標準入出力（Stdio）を用いたMCPプロトコル通信。
- **提供ツール**:
    - `notify_os(title, message)`: OS標準のデスクトップ通知を表示。
    - `notify_discord(content, prompt)`: Discord Webhookを用いてEmbed形式で通知。

### 2.2. CLIモード
- **起動方法**: サブコマンド（`os`, `discord`）を使用。
- **使用例**:
    - `oshirase-mcp os --title "Hello" --message "World"`
    - `oshirase-mcp discord --content "Task Complete" --prompt "Explain Rust..."`

## 3. 設定管理 (ハイブリッド方式)
設定値は以下の優先順位で解決される。

1. **環境変数**:
    - `DISCORD_WEBHOOK_URL`
    - `OS_NOTIFICATION_DEFAULT_TITLE`
2. **設定ファイル (`config.toml`)**:
    - パス: `%AppData%\mcp-notifier\config.toml` (Windows)
    - 初回実行時に設定ファイルが存在しない場合、デフォルト値を含んだテンプレートを自動生成する。

## 4. Discord通知仕様 (Embed)
- **タイトル**: 設定されたデフォルトタイトル（例: "Gemini CLI"）
- **本文**: `content` 引数の内容。
- **プロンプト**: `prompt` 引数がある場合、その冒頭部分（最大100文字程度）を「ソースプロンプト」として表示。
- **フッター**: "Sent from Gemini CLI" または "Sent via Oshirase-MCP"

## 5. プロジェクト構造とGit管理
- **言語**: Rust
- **ホワイトリスト方式の `.gitignore`**:
    ```text
    /*
    !/.gitignore
    !/src/
    !/Cargo.toml
    !/Cargo.lock
    !/gemini-extension.json
    !/Oshirase-MCP仕様書.md
    !/README.md
    !/変数関数仕様書.md
    !/TODO.md
    ```

## 6. Gemini Extension 連携
`gemini-extension.json` を同梱し、`${extensionPath}` を用いてポータブルなインストールを可能にする。
