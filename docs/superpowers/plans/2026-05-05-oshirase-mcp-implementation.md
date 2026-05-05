# Oshirase-MCP 実装プラン

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** OS通知とDiscord Embed通知を統合管理する、MCPサーバー兼CLIツールをRustで構築する。

**Architecture:** 
- `clap` によるコマンドライン引数解析（MCPモードとCLIモードの切り替え）。
- `serde`/`toml` によるハイブリッド設定管理（環境変数 + config.toml）。
- `notify-rust` によるOS通知と `reqwest` によるDiscord Webhook通知。
- 非同期ランタイム `tokio` 上での非ブロッキング処理。

**Tech Stack:** Rust, tokio, clap, serde, toml, notify-rust, reqwest, mcp-sdk-rs

---

### Task 1: 設定管理モジュールの実装

**Files:**
- Create: `src/config.rs`
- Modify: `src/main.rs`
- Test: `src/config.rs` (inline tests)

- [ ] **Step 1: 設定構造体とテンプレート生成ロジックの実装**
```rust
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub discord: DiscordConfig,
    pub general: GeneralConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DiscordConfig {
    pub webhook_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GeneralConfig {
    pub default_title: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            discord: DiscordConfig { webhook_url: None },
            general: GeneralConfig { default_title: "Gemini CLI".to_string() },
        }
    }
}
```

- [ ] **Step 2: 環境変数とtomlファイルを統合する読み込み関数の実装**
- [ ] **Step 3: 設定ファイルの自動生成テスト**
- [ ] **Step 4: コミット**

### Task 2: CLI引数解析とモード切替の実装

**Files:**
- Modify: `src/main.rs`

- [ ] **Step 1: `clap` を用いたサブコマンド定義**
- [ ] **Step 2: MCPモード（--mcp）とCLIモード（os, discord）の分岐処理**
- [ ] **Step 3: 各モードの動作確認（空の実装でOK）**
- [ ] **Step 4: コミット**

### Task 3: 通知ツールの実装 (OS & Discord)

**Files:**
- Create: `src/notifier.rs`
- Modify: `src/main.rs`

- [ ] **Step 1: `notify-rust` を使ったOS通知関数の実装**
- [ ] **Step 2: `reqwest` を使ったDiscord Embed通知関数の実装**
- [ ] **Step 3: CLIモードからの呼び出しテスト**
- [ ] **Step 4: コミット**

### Task 4: MCPプロトコルの実装

**Files:**
- Modify: `src/main.rs`

- [ ] **Step 1: MCPツールの定義とハンドラ登録**
- [ ] **Step 2: 標準入出力ループの開始**
- [ ] **Step 3: Gemini CLIからの `link` & `/mcp list` での動作確認**
- [ ] **Step 4: コミット**
