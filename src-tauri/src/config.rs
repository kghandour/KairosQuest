use serde::{Deserialize, Serialize};
use std::fmt;
use std::path::PathBuf;
use std::sync::{OnceLock, RwLock};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AppConfig {
    pub first_run: bool,
}

static CONFIG: OnceLock<RwLock<AppConfig>> = OnceLock::new();

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
}

pub fn get_config_path() -> PathBuf {
    let mut path = std::env::current_exe().expect("Failed to get current directory");
    path.pop(); // Remove executable name
    path.push("app_config.json");
    return path;
}

pub fn init_config(){
    let path = get_config_path();
    
    let app_config;
    if path.exists() {
        let data = std::fs::read_to_string(&path)
            .expect("Failed to read config file");
        app_config = serde_json::from_str(&data).expect("Failed to parse config file")
    } else {
        app_config = AppConfig::default()
    }
    let _ = CONFIG.set(RwLock::new(app_config));
}

pub fn get_config() -> AppConfig{
    let lock = CONFIG.get().expect("Config not initialized");
    let config = lock.read().unwrap();
    return config.clone();
}

pub fn update_first_use(val: bool){
    if let Some(lock) = CONFIG.get() {
        let mut config = lock.write().unwrap();
        config.set_first_run(val);
    }
}

pub fn save_config() {
    let config = get_config();
    let path = get_config_path();
    let data = serde_json::to_string_pretty(&config).expect("Failed to serialize config");
    std::fs::write(path, data).expect("Failed to write config file");

}
