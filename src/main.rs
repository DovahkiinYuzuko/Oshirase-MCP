pub mod config;

use clap::{Parser, Subcommand};
use crate::config::Config;

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
async fn main() {
    eprintln!("[DEBUG] Oshirase-MCP starting...");

    // 引数の解析
    let cli = Cli::parse();

    // 設定の読み込み
    let config = Config::load();
    eprintln!("[DEBUG] Config loaded: {:?}", config);

    if cli.mcp {
        // MCPモードの開始（Task 4で実装）
        eprintln!("[INFO] Starting MCP mode...");
        eprintln!("[DEBUG] MCP mode is not yet implemented. Stopping.");
        return;
    }

    match cli.command {
        Some(Commands::Os { title, message }) => {
            // OS通知のプレースホルダ（Task 3で実装）
            let t = title.unwrap_or_else(|| config.general.default_title.clone());
            eprintln!("[INFO] Sending OS notification...");
            eprintln!("[DEBUG] Title: {}, Message: {}", t, message);
        }
        Some(Commands::Discord { content, prompt }) => {
            // Discord通知のプレースホルダ（Task 3で実装）
            eprintln!("[INFO] Sending Discord notification...");
            eprintln!("[DEBUG] Content: {}, Prompt: {:?}", content, prompt);
            if config.discord.webhook_url.is_none() {
                eprintln!("[WARN] Discord Webhook URL is not configured!");
            }
        }
        None => {
            // 何も指定がない場合はヘルプを表示
            use clap::CommandFactory;
            let mut cmd = Cli::command();
            cmd.print_help().unwrap();
            println!(); // 改行
        }
    }

    eprintln!("[DEBUG] Execution finished.");
}
