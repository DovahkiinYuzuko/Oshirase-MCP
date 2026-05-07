# Oshirase-MCP Integration Guide

Detailed setup instructions for various AI agents and tools.

## Prerequisites
- Rust (installed and available in PATH)
- A Discord Webhook URL (if using Discord notifications)

---

## 1. Gemini CLI
The easiest way to integrate with Gemini CLI is by linking the extension:
```bash
gemini extensions link .
```
This will automatically register the server defined in `gemini-extension.json`.

## 2. Claude Desktop
Add the following to your `claude_desktop_config.json`:
```json
{
  "mcpServers": {
    "oshirase": {
      "command": "cargo",
      "args": ["run", "--release", "--", "--mcp"],
      "cwd": "C:\\path\\to\\Oshirase-MCP"
    }
  }
}
```

## 3. Cursor / VS Code (GitHub Copilot Chat)
Create or edit `.cursor/mcp.json` (or `.vscode/mcp.json`):
```json
{
  "mcpServers": {
    "oshirase": {
      "command": "cargo",
      "args": ["run", "--release", "--", "--mcp"],
      "cwd": "C:\\path\\to\\Oshirase-MCP"
    }
  }
}
```

## 4. Claude Code CLI
```bash
claude mcp add oshirase -- cargo run --release -- --mcp
```

## 5. Codex CLI
Add to `~/.codex/config.toml`:
```toml
[mcp_servers.oshirase]
command = "cargo"
args = ["run", "--release", "--", "--mcp"]
cwd = "C:\\path\\to\\Oshirase-MCP"
```
