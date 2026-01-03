use serde::{Deserialize, Serialize};
use once_cell::sync::Lazy;
use std::fmt;

#[derive(Debug, Deserialize, Serialize)]
pub struct AppConfig {
    pub first_run: bool,
}

impl fmt::Display for AppConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AppConfig {{ first_run: {} }}", self.first_run)
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            first_run: true,
        }
    }
}

impl AppConfig {
    pub fn is_first_run(&self) -> bool {
        self.first_run
    }

    pub fn set_first_run(&mut self, value: bool) {
        self.first_run = value;
    }

    pub fn reset(&mut self) {
        *self = AppConfig::default();
    }
}

pub struct ConfigManager {
    pub config_path: std::path::PathBuf,
    pub config: AppConfig
}

impl Default for ConfigManager {
    fn default() -> Self {
        Self::new()
    }
}

impl ConfigManager {
    pub fn new () -> Self {
        let mut path = std::env::current_exe().expect("Failed to get current directory");
        path.pop(); // Remove executable name
        path.push("app_config.json");
        
        let app_config;
        if path.exists() {
            let data = std::fs::read_to_string(&path)
                .expect("Failed to read config file");
            app_config = serde_json::from_str(&data).expect("Failed to parse config file")
        } else {
            app_config = AppConfig::default()
        }

        println!("Configuration path: {:?}", path);
        println!("Configuration loaded: {:?}", app_config);
        Self {
            config_path: path,
            config: app_config,
        }
    }

    pub fn instance() -> &'static Self {
        static INSTANCE: Lazy<ConfigManager> = Lazy::new(|| {
            ConfigManager::new()
        });
        &INSTANCE
    }

    pub fn load_config(&self) -> AppConfig {
        if self.config_path.exists() {
            let data = std::fs::read_to_string(&self.config_path)
                .expect("Failed to read config file");
            serde_json::from_str(&data).expect("Failed to parse config file")
        } else {
            AppConfig::default()
        }
    }

    pub fn save_config(&self, config: &AppConfig) {
        let data = serde_json::to_string_pretty(config).expect("Failed to serialize config");
        std::fs::write(&self.config_path, data).expect("Failed to write config file");
    }
}

