pub mod config;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_config() -> String {
    config::get_config().to_string()
}

#[tauri::command]
fn update_first_use() -> String {
    config::update_first_use(!config::get_config().is_first_run());
    config::save_config();
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
            update_first_use
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
