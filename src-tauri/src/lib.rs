use std::collections::HashMap;
use tauri_plugin_store::StoreExt;

pub mod config;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

fn store_to_json(entries: Vec<(String, serde_json::Value)>) -> HashMap<String, serde_json::Value>{
    let mut all_data = HashMap::new();
    for (key, value) in entries {
        all_data.insert(key.clone(), value.clone());
    }

    all_data
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn save_to_store(app: tauri::AppHandle, key: String, value: serde_json::Value) -> HashMap<String, serde_json::Value> {
    let store = app.store("settings.json").expect("Could not load config");
    store.set(key.clone(), value.clone());
    match store.save() {
        Ok(_) => println!("Saved config"),
        Err(e) => println!("{}",e)
    };
    store_to_json(store.entries())
}

#[tauri::command]
async fn get_all_config(app: tauri::AppHandle) -> HashMap<String, serde_json::Value> {
    let store = app.store("settings.json").expect("Could not load config");
    store_to_json(store.entries())
}

#[tauri::command]
async fn check_first_run(app: tauri::AppHandle) -> bool {
    let store = app.store("settings.json").expect("Could not load config");
    println!("{}",store.get("workspace_path").expect(""));
    store.is_empty()
}

// #[tauri::command]
// fn get_config() -> String {
//     return serde_json::to_string_pretty(&config::get_config()).unwrap();
// }

// #[tauri::command]
// fn update_config_field(key: &str, field: serde_json::Value) -> String {
//     config::update_config_field(key, field);
//     save_config();
//     get_config()
// }

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    config::init_config();
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            // get_config,
            // update_config_field,
            save_to_store,
            get_all_config,
            check_first_run
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
