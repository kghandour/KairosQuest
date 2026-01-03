use crate::config::save_config;

pub mod config;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_config() -> String {
    return serde_json::to_string_pretty(&config::get_config()).unwrap();
}

#[tauri::command]
fn update_config_field(key: &str, field: serde_json::Value) -> String {
    println!("Key is {} and field is {}", key, field);
    config::update_config_field(key, field);
    save_config();
    get_config()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    config::init_config();
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            get_config,
            update_config_field
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
