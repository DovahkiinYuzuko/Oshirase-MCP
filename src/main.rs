pub mod config;
pub mod notifier;

use async_trait::async_trait;
use clap::{Parser, Subcommand};
use crate::config::Config;
use mcp_sdk_rs::server::{Server, ServerHandler};
use mcp_sdk_rs::transport::stdio::StdioTransport;
use mcp_sdk_rs::transport::Transport;
use mcp_sdk_rs::types::{
    ClientCapabilities, Implementation, ServerCapabilities, Tool,
    ToolResult, MessageContent, ToolSchema,
};
use serde_json::json;
use std::sync::Arc;

struct OshiraseHandler {
    config: Arc<Config>,
}

#[async_trait]
impl ServerHandler for OshiraseHandler {
    async fn initialize(
        &self,
        _implementation: Implementation,
        _capabilities: ClientCapabilities,
    ) -> Result<ServerCapabilities, mcp_sdk_rs::Error> {
        Ok(ServerCapabilities {
            tools: Some(json!(true)),
            resources: Some(json!(false)),
            prompts: Some(json!(false)),
            logging: Some(json!(false)),
            ..Default::default()
        })
    }

    async fn shutdown(&self) -> Result<(), mcp_sdk_rs::Error> {
        Ok(())
    }

    async fn handle_method(
        &self,
        method: &str,
        params: Option<serde_json::Value>,
    ) -> Result<serde_json::Value, mcp_sdk_rs::Error> {
        match method {
            "tools/list" => {
                let tools = vec![
                    Tool {
                        name: "notify_os".to_string(),
                        description: "OSデスクトップ通知を送信します。".to_string(),
                        input_schema: Some(ToolSchema {
                            properties: Some(json!({
                                "title": { "type": "string", "description": "通知のタイトル" },
                                "message": { "type": "string", "description": "通知のメッセージ内容" }
                            })),
                            required: Some(vec!["message".to_string()]),
                        }),
                        annotations: None,
                    },
                    Tool {
                        name: "notify_discord".to_string(),
                        description: "Discord Webhookを使用してEmbed形式の通知を送信します。".to_string(),
                        input_schema: Some(ToolSchema {
                            properties: Some(json!({
                                "content": { "type": "string", "description": "通知の本文" },
                                "prompt": { "type": "string", "description": "元のプロンプトなどの付加情報（任意）" }
                            })),
                            required: Some(vec!["content".to_string()]),
                        }),
                        annotations: None,
                    }
                ];
                Ok(json!({ "tools": tools }))
            }
            "tools/call" => {
                let params = params.unwrap_or_default();
                let name = params.get("name").and_then(|n| n.as_str()).unwrap_or("");
                let args = params.get("arguments").cloned().unwrap_or_else(|| json!({}));

                match name {
                    "notify_os" => {
                        let title = args.get("title").and_then(|t| t.as_str()).unwrap_or(&self.config.general.default_title);
                        let message = args.get("message").and_then(|m| m.as_str()).unwrap_or("");
                        
                        eprintln!("[INFO] MCP Tool [notify_os] called: {}", title);
                        notifier::send_os_notification(title, message);
                        
                        let result = ToolResult {
                            content: vec![MessageContent::Text {
                                text: "OS通知を送信しました。".to_string(),
                            }],
                            structured_content: None,
                        };
                        Ok(serde_json::to_value(result).unwrap())
                    }
                    "notify_discord" => {
                        let content = args.get("content").and_then(|c| c.as_str()).unwrap_or("");
                        let prompt = args.get("prompt").and_then(|p| p.as_str());

                        eprintln!("[INFO] MCP Tool [notify_discord] called");
                        if let Some(webhook_url) = &self.config.discord.webhook_url {
                            notifier::send_discord_notification(
                                webhook_url,
                                content,
                                prompt,
                                &self.config.general.default_title,
                            )
                            .await;
                            
                            let result = ToolResult {
                                content: vec![MessageContent::Text {
                                    text: "Discord通知を送信しました。".to_string(),
                                }],
                                structured_content: None,
                            };
                            Ok(serde_json::to_value(result).unwrap())
                        } else {
                            eprintln!("[WARN] Discord Webhook URL is not configured!");
                            let result = ToolResult {
                                content: vec![MessageContent::Text {
                                    text: "Error: Discord Webhook URLが設定されていません。".to_string(),
                                }],
                                structured_content: None,
                            };
                            Ok(serde_json::to_value(result).unwrap())
                        }
                    }
                    _ => Err(mcp_sdk_rs::Error::Protocol {
                        code: (-32601).into(),
                        message: format!("Unknown tool: {}", name),
                        data: None,
                    }),
                }
            }
            _ => Err(mcp_sdk_rs::Error::Protocol {
                code: (-32601).into(),
                message: format!("Unknown method: {}", method),
                data: None,
            }),
        }
    }
}

// --- CLI Definitions ---

/// OS通知とDiscord通知を統合管理するMCPサーバー兼CLIツール
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// MCPモードで起動する
    #[arg(long)]
    mcp: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// OSデスクトップ通知を送信する
    Os {
        /// 通知のタイトル
        #[arg(short, long)]
        title: Option<String>,
        /// 通知のメッセージ内容
        #[arg(short, long)]
        message: String,
    },
    /// Discord Webhook通知を送信する
    Discord {
        /// メッセージ内容
        #[arg(short, long)]
        content: String,
        /// 送信元のプロンプト情報など（任意）
        #[arg(short, long)]
        prompt: Option<String>,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 引数の解析
    let cli = Cli::parse();

    // 設定の読み込み
    let config = Arc::new(Config::load());

    if cli.mcp {
        // MCPモードの開始
        eprintln!("[INFO] Oshirase-MCP starting in MCP mode...");
        
        let (read_tx, read_rx) = tokio::sync::mpsc::channel::<String>(100);
        let (write_tx, mut write_rx) = tokio::sync::mpsc::channel::<String>(100);

        // Stdin reader
        tokio::spawn(async move {
            use tokio::io::AsyncBufReadExt;
            let stdin = tokio::io::stdin();
            let mut reader = tokio::io::BufReader::new(stdin).lines();
            while let Ok(Some(line)) = reader.next_line().await {
                if let Err(_) = read_tx.send(line).await {
                    break;
                }
            }
        });

        // Stdout writer
        tokio::spawn(async move {
            use tokio::io::AsyncWriteExt;
            let mut stdout = tokio::io::stdout();
            while let Some(line) = write_rx.recv().await {
                if let Err(_) = stdout.write_all(line.as_bytes()).await {
                    break;
                }
                if let Err(_) = stdout.write_all(b"\n").await {
                    break;
                }
                if let Err(_) = stdout.flush().await {
                    break;
                }
            }
        });

        let transport: Arc<dyn Transport> = Arc::new(StdioTransport::new(read_rx, write_tx));
        let handler: Arc<dyn ServerHandler> = Arc::new(OshiraseHandler { config: config.clone() });
        
        let server = Server::new(transport, handler);

        eprintln!("[INFO] MCP server listening on stdio.");
        server.start().await?;
    } else if let Some(command) = cli.command {
        match command {
            Commands::Os { title, message } => {
                let t = title.unwrap_or_else(|| config.general.default_title.clone());
                eprintln!("[INFO] Sending OS notification...");
                notifier::send_os_notification(&t, &message);
            }
            Commands::Discord { content, prompt } => {
                eprintln!("[INFO] Sending Discord notification...");
                if let Some(webhook_url) = &config.discord.webhook_url {
                    notifier::send_discord_notification(
                        webhook_url,
                        &content,
                        prompt.as_deref(),
                        &config.general.default_title,
                    )
                    .await;
                } else {
                    eprintln!("[WARN] Discord Webhook URL is not configured!");
                }
            }
        }
    } else {
        // 何も指定がない場合はヘルプを表示
        use clap::CommandFactory;
        let mut cmd = Cli::command();
        cmd.print_help().unwrap();
        println!(); // 改行
    }

    Ok(())
}
