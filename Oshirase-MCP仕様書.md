# 監査用仕様書：Oshirase MCP Server (Rust)

## 1. プロジェクト概要
本プロジェクトは、Model Context Protocol (MCP) に準拠し、OS標準のデスクトップ通知および外部サービス（Discord等）への通知を統合管理するローカルサーバーである。
Rustで実装し、単一のバイナリとしてWindowsおよびmacOSの両環境で動作することを目標とする。

## 2. 技術スタック・依存ライブラリ
*   **言語:** Rust (最新のStable版)
*   **MCPプロトコル:** `mcp-rust-sdk` または `mcp-sdk-rs` (Stdio通信)
*   **OS通知:** `notify-rust` (Windows Toast / macOS Notification Center対応)
*   **HTTPクライアント:** `reqwest` (Discord Webhook送信、非同期)
*   **設定ファイル:** `serde` / `toml` (設定のシリアライズ)
*   **パス管理:** `directories-next` (OSごとの標準ディレクトリ取得)
*   **非同期実行:** `tokio`

## 3. 機能要件 (Tool Definitions)
MCPホスト（Gemini CLI等）から呼び出し可能なツールとして以下を定義する。

### ① `notify_os`
*   **説明:** OSのデスクトップ通知（ポップアップ）を表示する。
*   **引数:**
    *   `title` (string): 通知のタイトル。
    *   `message` (string): 通知の本文。
*   **実装上の注意:**
    *   Windowsではトースト通知、macOSでは通知センターを利用すること。
    *   アイコンやアプリ名は設定ファイルまたはデフォルト値から読み込む。

### ② `notify_discord`
*   **説明:** 設定されたWebhook URLを使用してDiscordにメッセージを送信する。
*   **引数:**
    *   `content` (string): 送信メッセージ。
*   **実装上の注意:**
    *   Webhook URLはバイナリにハードコードせず、設定ファイルから読み込むこと。
    *   送信失敗時（オフライン等）でも、プロセス全体をクラッシュさせずにエラーを返すこと。

## 4. 設定ファイルの仕様
OSごとの標準設定ディレクトリに `config.toml` を配置する。
*   **Windows:** `%AppData%\mcp-notifier\config.toml`
*   **macOS:** `~/Library/Application Support/mcp-notifier/config.toml`

### config.toml の構造 (例)
```toml
[discord]
webhook_url = "[https://discord.com/api/webhooks/](https://discord.com/api/webhooks/)..."

[general]
default_title = "Gemini CLI"
```

## 5. 非機能要件
*   **ポータビリティ:** 外部DLLに依存せず、静的リンクされた単一の実行バイナリとしてビルドすること。
*   **非同期処理:** 通知処理（特にネットワークを介するDiscord送信）がMCPのメインループをブロックしないよう、非同期で実行すること。
*   **堅牢性:** 設定ファイルが存在しない場合はデフォルト値で動作するか、エラーメッセージを標準エラー出力(stderr)に明確に出すこと。

## 6. 公開・配布形式
*   Windows環境用: `oshirase-mcp.exe` (x86_64-pc-windows-msvc)
*   macOS環境用: `oshirase-mcp` (Universal Binary または各アーキテクチャ用バイナリ)
*   各OSエージェント（Claude Desktop, Cursor, Gemini CLI等）から、バイナリのパスを指定するだけで利用可能とする。
  
## 7.ユズコからの追記
多分各エージェントごとにお作法があるはずだからそれはGoogleSearch使って調べよう。