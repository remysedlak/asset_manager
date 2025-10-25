use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct AppConfig {
    pub vault_path: String,
    pub font_path: String,
    pub thumbnail_size: f32
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            vault_path: Self::get_default_vault_path(),
            font_path: Self::get_default_font_path(),
            thumbnail_size: 8.0
        }
    }
}

impl AppConfig {
    fn get_config_path() -> PathBuf {
        let config_dir = if cfg!(target_os = "windows") {
            dirs::config_dir().unwrap_or_else(|| PathBuf::from("."))
        } else if cfg!(target_os = "macos") {
            dirs::config_dir().unwrap_or_else(|| PathBuf::from("."))
        } else {
            dirs::config_dir().unwrap_or_else(|| PathBuf::from("."))
        };

        config_dir.join("AssetManager")
    }

    fn get_default_vault_path() -> String {
        if let Some(home) = dirs::home_dir() {
            let vault_path = if cfg!(target_os = "windows") {
                home.join("Documents").join("AssetManager").join("SVGs")
            } else if cfg!(target_os = "macos") {
                home.join("Documents").join("AssetManager").join("SVGs")
            } else {
                // Linux
                home.join("Documents").join("AssetManager").join("SVGs")
            };

            // Create the directory if it doesn't exist
            let _ = fs::create_dir_all(&vault_path);

            vault_path.to_string_lossy().to_string()
        } else {
            String::new()
        }
    }

    fn get_default_font_path() -> String {
        if let Some(home) = dirs::home_dir() {
            let font_path = if cfg!(target_os = "windows") {
                home.join("Documents").join("AssetManager").join("Fonts")
            } else if cfg!(target_os = "macos") {
                home.join("Documents").join("AssetManager").join("Fonts")
            } else {
                // Linux
                home.join("Documents").join("AssetManager").join("Fonts")
            };

            // Create the directory if it doesn't exist
            let _ = fs::create_dir_all(&font_path);

            font_path.to_string_lossy().to_string()
        } else {
            String::new()
        }
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

    pub fn is_valid(&self) -> bool {
        !self.vault_path.is_empty() &&
            !self.font_path.is_empty() &&
            PathBuf::from(&self.vault_path).exists() &&
            PathBuf::from(&self.font_path).exists()
    }
}