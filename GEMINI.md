# Oshirase-MCP Extension Guide

This extension provides a Model Context Protocol (MCP) server for sending notifications via local OS desktop alerts and Discord webhooks.

## Core Capabilities

### 1. `notify_os` Tool
- **Function**: Displays a native desktop notification on the host machine.
- **Strict Usage**: Use this for immediate feedback to the local user (e.g., build completion, long-running task finished, or critical local error).
- **Parameters**:
    - `title`: (Optional) The title of the notification. Defaults to the configured value in `config.toml` (usually "Gemini CLI").
    - `message`: (Required) The body text of the notification. Be concise but informative.

### 2. `notify_discord` Tool
- **Function**: Sends a rich embed notification to a pre-configured Discord channel via Webhook.
- **Strict Usage**: Use this for sharing status with a team, logging external events, or when the user specifically requests a persistent notification in Discord.
- **Parameters**:
    - `content`: (Required) The main description/body of the Discord embed.
    - `prompt`: (Optional) The original user prompt or relevant context. The server will automatically truncate this to the first 100 characters.

## Agent Instructions (Rules of Engagement)

- **Proactive Notification**: When a user initiates a task that is expected to take significant time, offer to send a notification upon completion.
- **Choice of Channel**: If the destination is ambiguous, prioritize `notify_os` for individual local tasks and `notify_discord` for tasks involving shared environments or explicit requests for "Discord".
- **Clarity of Content**: Always summarize the execution results into the `message` or `content` fields. Do not send raw, unformatted data.
- **Error Handling**: If a notification fails (e.g., Webhook URL missing), inform the user about the specific configuration issue found in `config.toml`.

## Installation for Gemini CLI

To register this extension locally for development:
```bash
gemini extensions link .
```
This will allow Gemini to discover the tools defined in `gemini-extension.json`.
