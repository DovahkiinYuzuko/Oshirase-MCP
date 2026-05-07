# Oshirase-MCP

Unified notification manager for OS desktop alerts and Discord Webhooks via Model Context Protocol (MCP).

## Tools

- **notify_os**: Native desktop notifications.
- **notify_discord**: Rich Discord embed notifications.

## Prerequisites

- Rust (for building from source)
- Discord Webhook URL (optional)

## Configuration

Settings are stored in `config.toml` (e.g., `%APPDATA%\mcp-notifier\config.toml` on Windows).

```toml
[discord]
webhook_url = "YOUR_DISCORD_WEBHOOK_URL"

[general]
default_title = "Oshirase"
```

## AI Agent Integration

### Gemini CLI
Run this in the project root:
```bash
gemini extensions link .
```

### Claude Code CLI
```bash
claude mcp add oshirase -- cargo run --release -- --mcp
```

### Cursor / VS Code (GitHub Copilot Chat)
Add the following to your MCP settings or `.cursor/mcp.json`:
```json
{
  "mcpServers": {
    "oshirase": {
      "command": "cargo",
      "args": ["run", "--release", "--", "--mcp"],
      "cwd": "/absolute/path/to/Oshirase-MCP"
    }
  }
}
```

### Codex CLI
Add to `~/.codex/config.toml`:
```toml
[mcp_servers.oshirase]
command = "cargo"
args = ["run", "--release", "--", "--mcp"]
cwd = "/absolute/path/to/Oshirase-MCP"
```
