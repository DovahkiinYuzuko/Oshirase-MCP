pub mod config;

use crate::config::Config;

#[tokio::main]
async fn main() {
    eprintln!("[DEBUG] Oshirase-MCP starting...");

    // 設定の読み込み
    let config = Config::load();
    eprintln!("[DEBUG] Config loaded: {:?}", config);

    // TODO: 以降のタスクで引数解析やMCP/CLIモードの分岐を実装する
    eprintln!("[DEBUG] Execution finished.");
}
