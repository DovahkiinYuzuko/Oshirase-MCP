# Oshirase-MCP

[日本語](#日本語)|[English](#english)

## 日本語

### 概要
Oshirase-MCPは、Model Context Protocol (MCP) を使用して、OSデスクトップ通知およびDiscord Webhook通知を送信するための統合通知管理サーバーです。AIエージェントやCLIツールからのフィードバックを視覚的に受け取ることができます。

### 特徴
- **OSデスクトップ通知**: Windows、macOS、Linuxでネイティブ通知を表示します。
- **Discord Webhook対応**: Webhookを使用してDiscordチャンネルにリッチな埋め込み通知を送信します。

### 設定
設定は、システムのユーザー設定ディレクトリ（Windowsの場合は `%APPDATA%\mcp-notifier\config.toml`）に保存されます。

### AIエージェントへの導入
詳細な導入手順については、ルートディレクトリの `docs/setup/INTEGRATION.md` を参照してください。

## English

### Overview
Oshirase-MCP is a unified notification manager for sending OS desktop alerts and Discord Webhook notifications via the Model Context Protocol (MCP). It allows you to receive visual feedback from AI agents and CLI tools.

### Features
- **OS Desktop Notifications**: Native alerts on Windows, macOS, and Linux.
- **Discord Webhook Support**: Rich embed notifications to Discord channels using Webhooks.

### Configuration
Settings are managed in `config.toml` located in your system's config directory (e.g., `%APPDATA%\mcp-notifier\config.toml`).

### AI Agent Integration
For detailed setup instructions, please refer to `docs/setup/INTEGRATION.md` in the root directory.
