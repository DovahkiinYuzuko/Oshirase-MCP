# Oshirase-MCP

Unified notification manager for OS desktop alerts and Discord Webhooks via Model Context Protocol (MCP).

## Overview

Oshirase-MCP allows AI agents and CLI tools to send notifications to your local desktop or a Discord channel. It's designed to be a lightweight, single-responsibility server that improves your feedback loop during long-running tasks.

## Features

- **OS Desktop Notifications**: Native alerts on Windows, macOS, and Linux.
- **Discord Webhook Support**: Rich embeds with timestamp and context fields.

## Quick Start

1. **Build**:
   ```bash
   cargo build --release
   ```
2. **Setup AI Agents**:
   Follow the [Integration Guide](docs/setup/INTEGRATION.md) to add this server to your favorite AI tools.

## Configuration

Settings are managed in `config.toml` located in your system's config directory (e.g., `%APPDATA%\mcp-notifier\config.toml`).

```toml
[discord]
webhook_url = "https://discord.com/api/webhooks/..."

[general]
default_title = "Oshirase"
```

## License

MIT
