use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use directories_next::ProjectDirs;

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
            discord: DiscordConfig {
                webhook_url: None,
            },
            general: GeneralConfig {
                default_title: "Gemini CLI".to_string(),
            },
        }
    }
}

impl Config {
    /// 設定を読み込む。
    /// 優先順位: 環境変数 > config.toml > デフォルト値
    pub fn load() -> Self {
        println!("[DEBUG] Config::load() started.");
        let mut config = Config::default();

        // 1. 設定ファイルからの読み込み
        if let Some(proj_dirs) = ProjectDirs::from("", "", "mcp-notifier") {
            let config_dir = proj_dirs.config_dir();
            let config_path = config_dir.join("config.toml");

            println!("[DEBUG] Target config path: {:?}", config_path);

            if !config_dir.exists() {
                println!("[DEBUG] Config directory does not exist. Creating: {:?}", config_dir);
                if let Err(e) = fs::create_dir_all(config_dir) {
                    eprintln!("[ERROR] Failed to create config directory: {}", e);
                }
            }

            if config_path.exists() {
                println!("[DEBUG] Loading existing config file.");
                match fs::read_to_string(&config_path) {
                    Ok(content) => {
                        match toml::from_str::<Config>(&content) {
                            Ok(toml_config) => {
                                config = toml_config;
                                println!("[DEBUG] Config file loaded successfully.");
                            }
                            Err(e) => {
                                eprintln!("[ERROR] Failed to parse config.toml: {}. Using defaults.", e);
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("[ERROR] Failed to read config.toml: {}", e);
                    }
                }
            } else {
                println!("[DEBUG] Config file not found. Generating default config.toml.");
                let toml_string = toml::to_string_pretty(&config).unwrap();
                if let Err(e) = fs::write(&config_path, toml_string) {
                    eprintln!("[ERROR] Failed to write default config.toml: {}", e);
                }
            }
        } else {
            eprintln!("[ERROR] Could not determine project directory. Using defaults.");
        }

        // 2. 環境変数による上書き
        if let Ok(url) = env::var("DISCORD_WEBHOOK_URL") {
            println!("[DEBUG] Overriding Discord Webhook URL from environment variable.");
            config.discord.webhook_url = Some(url);
        }

        if let Ok(title) = env::var("OS_NOTIFICATION_DEFAULT_TITLE") {
            println!("[DEBUG] Overriding default title from environment variable.");
            config.general.default_title = title;
        }

        println!("[DEBUG] Final config state: {:?}", config);
        config
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.general.default_title, "Gemini CLI");
        assert!(config.discord.webhook_url.is_none());
    }

    #[test]
    fn test_env_override() {
        env::set_var("DISCORD_WEBHOOK_URL", "https://example.com/webhook");
        env::set_var("OS_NOTIFICATION_DEFAULT_TITLE", "Test Title");

        let config = Config::load();
        
        assert_eq!(config.discord.webhook_url, Some("https://example.com/webhook".to_string()));
        assert_eq!(config.general.default_title, "Test Title");

        // クリーンアップ
        env::remove_var("DISCORD_WEBHOOK_URL");
        env::remove_var("OS_NOTIFICATION_DEFAULT_TITLE");
    }
}
