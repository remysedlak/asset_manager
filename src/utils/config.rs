use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct AppConfig {
    pub vault_path: String,
    pub font_path: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            vault_path: "/home/remy/Pictures/images/svg".to_owned(),
            font_path: "/home/remy/Documents/fonts".to_owned()
        }
    }
}

impl AppConfig {
    fn get_config_path() -> PathBuf {
        let config_dir = if cfg!(target_os = "windows") {
            // Windows: C:\Users\Username\AppData\Roaming\AssetManager
            dirs::config_dir().unwrap_or_else(|| PathBuf::from("."))
        } else if cfg!(target_os = "macos") {
            // macOS: ~/Library/Application Support/AssetManager
            dirs::config_dir().unwrap_or_else(|| PathBuf::from("."))
        } else {
            // Linux: ~/.config/AssetManager
            dirs::config_dir().unwrap_or_else(|| PathBuf::from("."))
        };

        config_dir.join("AssetManager")
    }

    pub fn load() -> Self {
        let config_path = Self::get_config_path().join("config.json");

        if let Ok(content) = fs::read_to_string(&config_path) {
            if let Ok(config) = serde_json::from_str(&content) {
                return config;
            }
        }

        Self::default()
    }

    pub fn save(&self) {
        let config_dir = Self::get_config_path();
        if fs::create_dir_all(&config_dir).is_ok() {
            let config_path = config_dir.join("config.json");
            if let Ok(json) = serde_json::to_string_pretty(&self) {
                let _ = fs::write(config_path, json);
            }
        }
    }
}