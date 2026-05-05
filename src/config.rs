use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use directories_next::ProjectDirs;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub discord: DiscordConfig,
    pub general: GeneralConfig,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DiscordConfig {
    pub webhook_url: Option<String>,
}

impl std::fmt::Debug for DiscordConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DiscordConfig")
            .field("webhook_url", &self.webhook_url.as_ref().map(|_| "********"))
            .finish()
    }
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
        eprintln!("[DEBUG] Config::load() started.");
        let mut config = Config::default();

        // 1. 設定ファイルからの読み込み
        if let Some(proj_dirs) = ProjectDirs::from("", "", "mcp-notifier") {
            let config_dir = proj_dirs.config_dir();
            let config_path = config_dir.join("config.toml");

            eprintln!("[DEBUG] Target config path: {:?}", config_path);

            if !config_dir.exists() {
                eprintln!("[DEBUG] Config directory does not exist. Creating: {:?}", config_dir);
                if let Err(e) = fs::create_dir_all(config_dir) {
                    eprintln!("[ERROR] Failed to create config directory: {}", e);
                }
            }

            if config_path.exists() {
                eprintln!("[DEBUG] Loading existing config file.");
                match fs::read_to_string(&config_path) {
                    Ok(content) => {
                        match toml::from_str::<Config>(&content) {
                            Ok(toml_config) => {
                                config = toml_config;
                                eprintln!("[DEBUG] Config file loaded successfully.");
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
                eprintln!("[DEBUG] Config file not found. Generating default config.toml.");
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
            eprintln!("[DEBUG] Overriding Discord Webhook URL from environment variable.");
            config.discord.webhook_url = Some(url);
        }

        if let Ok(title) = env::var("OS_NOTIFICATION_DEFAULT_TITLE") {
            eprintln!("[DEBUG] Overriding default title from environment variable.");
            config.general.default_title = title;
        }

        eprintln!("[DEBUG] Final config state: {:?}", config);
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
    fn test_config_debug_mask() {
        let config = Config {
            discord: DiscordConfig {
                webhook_url: Some("https://hooks.discord.com/api/webhooks/secret".to_string()),
            },
            general: GeneralConfig {
                default_title: "Test".to_string(),
            },
        };
        let debug_str = format!("{:?}", config);
        assert!(!debug_str.contains("secret"));
        assert!(debug_str.contains("********"));
    }
}
