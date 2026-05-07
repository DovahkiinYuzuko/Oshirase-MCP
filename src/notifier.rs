use notify_rust::Notification;
use reqwest::Client;
use serde_json::json;

/// OSのデスクトップ通知を送信する
pub fn send_os_notification(title: &str, message: &str) {
    eprintln!("[DEBUG] send_os_notification開始: {} - {}", title, message);
    if let Err(e) = Notification::new()
        .summary(title)
        .body(message)
        .appname("Oshirase-MCP")
        .timeout(0) // ユーザーが閉じるまで表示
        .show()
    {
        eprintln!("[ERROR] OS通知の送信に失敗しました: {}", e);
    } else {
        eprintln!("[DEBUG] OS通知の送信が正常に完了しました。");
    }
}

/// Discord Webhookを使用して通知を送信する
pub async fn send_discord_notification(
    webhook_url: &str,
    content: &str,
    prompt: Option<&str>,
    default_title: &str,
) {
    eprintln!("[DEBUG] send_discord_notification開始: {}", default_title);
    let client = Client::new();

    // プロンプトがある場合は、冒頭100文字をフィールドに追加してリッチにする
    let mut fields = Vec::new();
    if let Some(p) = prompt {
        let truncated_prompt = if p.len() > 100 {
            format!("{}...", &p[..100])
        } else {
            p.to_string()
        };
        fields.push(json!({
            "name": "Original Prompt",
            "value": truncated_prompt,
            "inline": false
        }));
    }

    let payload = json!({
        "embeds": [{
            "title": default_title,
            "description": content,
            "color": 0x00ff00, // 通知カラー（緑）
            "fields": fields,
            "footer": {
                "text": "Sent via Oshirase-MCP"
            },
            "timestamp": chrono::Utc::now().to_rfc3339()
        }]
    });

    match client.post(webhook_url).json(&payload).send().await {
        Ok(res) => {
            if res.status().is_success() {
                eprintln!("[DEBUG] Discord通知の送信が正常に完了しました。");
            } else {
                let status = res.status();
                let body = res.text().await.unwrap_or_default();
                eprintln!("[ERROR] Discord APIからエラーが返されました: {} - {}", status, body);
            }
        }
        Err(e) => {
            eprintln!("[ERROR] Discord通知の送信中にネットワークエラーが発生しました: {}", e);
        }
    }
}
