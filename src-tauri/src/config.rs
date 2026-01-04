use serde::{Deserialize, Serialize};
use std::fmt;
use std::path::PathBuf;
use std::sync::{OnceLock, RwLock};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AppConfig {
    pub first_run: bool,
    pub workspace_path: String,
}

static CONFIG: OnceLock<RwLock<AppConfig>> = OnceLock::new();

impl fmt::Display for AppConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(self).unwrap())
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            first_run: true,
            workspace_path: "".to_string(),
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
}

pub fn get_config_path() -> PathBuf {
    let mut path = std::env::current_exe().expect("Failed to get current directory");
    path.pop(); // Remove executable name
    path.push("app_config.json");
    path
}

pub fn init_config() {
    let path = get_config_path();

    let app_config = if path.exists() {
        let data = std::fs::read_to_string(&path).expect("Failed to read config file");
        match serde_json::from_str(&data) {
            Ok(app_config) => app_config,
            Err(_) => AppConfig::default(),
        }
    } else {
        AppConfig::default()
    };
    let _ = CONFIG.set(RwLock::new(app_config));
}

pub fn get_config() -> AppConfig {
    let lock = CONFIG.get().expect("Config not initialized");
    let config = lock.read().unwrap();
    config.clone()
}

pub fn update_config_field(key: &str, value: serde_json::Value) {
    if let Some(lock) = CONFIG.get() {
        let mut config = lock.write().unwrap();

        match key {
            "first_run" => {
                if let Some(val) = value.as_bool() {
                    config.first_run = val;
                }
            }
            "workspace_path" => {
                if let Some(val) = value.as_str() {
                    config.workspace_path = val.to_string();
                }
            }
            _ => {
                eprintln!("Warning: Unknown configuration key '{}'", key);
            }
        }
    }
}

pub fn update_first_use(val: bool) {
    update_config_field("first_run", serde_json::Value::Bool(val));
}

pub fn save_config() {
    let config = get_config();
    let path = get_config_path();
    let data = serde_json::to_string_pretty(&config).expect("Failed to serialize config");
    std::fs::write(path, data).expect("Failed to write config file");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn display_and_serde_roundtrip() {
        let cfg = AppConfig {
            first_run: false,
            workspace_path: "".to_string(),
        };
        assert_eq!(format!("{}", cfg), "AppConfig { first_run: false }");

        let json = serde_json::to_string(&cfg).unwrap();
        let round: AppConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(cfg.first_run, round.first_run);
    }
}
