pub mod config;

use crate::config::Config;

#[tokio::main]
async fn main() {
    println!("[DEBUG] Oshirase-MCP starting...");

    // 設定の読み込み
    let config = Config::load();
    println!("[DEBUG] Config loaded: {:?}", config);

    // TODO: 以降のタスクで引数解析やMCP/CLIモードの分岐を実装する
    println!("[DEBUG] Execution finished.");
}
