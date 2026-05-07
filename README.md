# Oshirase-MCP

> [!TIP]
> **AI Agents**: Please refer to [AGENTS.md](./AGENTS.md) for specialized tool usage guidelines and instructions.

[日本語](#日本語)|[English](#english)

## 日本語

### 概要
Oshirase-MCPは、Model Context Protocol (MCP) を通じて、OSデスクトップ通知とDiscord Webhook通知を統合して管理・送信できるサーバーです。AIエージェントやCLIツールから、ビルド完了やエラーなどの情報を視覚的に受け取ることができます。

### インストール方法

#### Gemini CLI (推奨)
GitHubリポジトリのURLを指定するだけで、バイナリのダウンロードから設定まで自動で行えます。
```bash
gemini extensions install https://github.com/DovahkiinYuzuko/Oshirase-MCP
```
*インストール中に Discord Webhook URL などの設定を聞かれるので、画面の指示に従ってください。*

#### Claude Code
```bash
claude mcp add oshirase --command "npx" --args "-y,oshirase-mcp"
```

#### Codex CLI
`~/.codex/config.toml` に以下の設定を追加してください。
```toml
[mcp_servers.oshirase]
command = "npx"
args = ["-y", "oshirase-mcp", "--mcp"]
```

### 使い方
1. **ビルド**: `oshirase-mcp-server` フォルダへ移動し、`cargo build --release` を実行します。
2. **MCP登録**: お使いのAIエージェント（Gemini CLI, Claude, Cursor等）に、ビルドしたバイナリをMCPサーバーとして登録します。
3. **設定**: 初回起動時に生成される `config.toml` に Discord Webhook などを設定してください。

### リポジトリ構成
- `oshirase-mcp-server/`: MCPサーバーのソースコードとビルド用ファイル。
- `LICENSE`: MITライセンス。
- `NOTICE.md`: サードパーティ製ライブラリのライセンス告知。

詳細なセットアップ手順については、[oshirase-mcp-server/README.md](./oshirase-mcp-server/README.md) を参照してください。

## English

### Overview
Oshirase-MCP is a unified server for managing and sending notifications via OS desktop alerts and Discord Webhooks through the Model Context Protocol (MCP). It allows AI agents and CLI tools to provide visual feedback for events like build completions or critical errors.

### Installation

#### Gemini CLI (Recommended)
You can install the extension directly from the GitHub repository URL. The binary download and initial configuration are handled automatically.
```bash
gemini extensions install https://github.com/DovahkiinYuzuko/Oshirase-MCP
```
*You will be prompted to enter your Discord Webhook URL and other settings during installation.*

#### Claude Code
```bash
claude mcp add oshirase --command "npx" --args "-y,oshirase-mcp"
```

#### Codex CLI
Add the following configuration to your `~/.codex/config.toml`:
```toml
[mcp_servers.oshirase]
command = "npx"
args = ["-y", "oshirase-mcp", "--mcp"]
```

### Usage
1. **Build**: Navigate to the `oshirase-mcp-server` directory and run `cargo build --release`.
2. **MCP Registration**: Register the compiled binary as an MCP server in your AI agent (e.g., Gemini CLI, Claude, Cursor).
3. **Configuration**: Set up your Discord Webhook or other settings in the `config.toml` file generated upon the first run.

### Repository Structure
- `oshirase-mcp-server/`: Source code and build files for the MCP server.
- `LICENSE`: MIT License.
- `NOTICE.md`: License notices for third-party libraries.

For detailed setup instructions, please refer to [oshirase-mcp-server/README.md](./oshirase-mcp-server/README.md).
