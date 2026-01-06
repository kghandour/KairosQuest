use std::collections::HashMap;
use tauri_plugin_store::StoreExt;
use tauri_plugin_fs::FsExt;

pub const STORE_FILE: &str = "settings.json";

fn convert_store_to_json(
    entries: Vec<(String, serde_json::Value)>,
) -> HashMap<String, serde_json::Value> {
    let mut all_data = HashMap::new();
    for (key, value) in entries {
        all_data.insert(key.clone(), value.clone());
    }

    all_data
}

#[tauri::command]
pub async fn save_to_store(
    app: tauri::AppHandle,
    key: String,
    value: serde_json::Value,
) -> HashMap<String, serde_json::Value> {
    let store = app.store(STORE_FILE).expect("Could not load config");
    store.set(key.clone(), value.clone());
    match store.save() {
        Ok(_) => {
            if key == "workspace_path" {
                match grant_workspace_permission(app, value.to_string()).await {
                    Ok(_) =>   println!("Saved config"),
                    Err(e) => println!("Error {}", e)
                };
            }
            
        },
        Err(e) => println!("{}", e),
    };
    convert_store_to_json(store.entries())
}

#[tauri::command]
pub async fn get_all_config(app: tauri::AppHandle) -> HashMap<String, serde_json::Value> {
    let store = app.store(STORE_FILE).expect("Could not load config");
    convert_store_to_json(store.entries())
}

#[tauri::command]
pub async fn check_first_run(app: tauri::AppHandle) -> bool {
    let store = app.store(STORE_FILE).expect("Could not load config");
    store.is_empty()
}

#[tauri::command]
pub async fn grant_workspace_permission(app: tauri::AppHandle, path: String) -> Result<(), tauri_plugin_fs::Error>{
    let scope = app.fs_scope();
    scope.allow_directory(path, true)?;

    Ok(())
}

#[tauri::command]
pub async fn get_workspace_path(app: tauri::AppHandle) -> String {
    let store = app.store(STORE_FILE).expect("Could not load config");
    store.get("workspace_path").expect("could not retrieve ws_path").as_str().expect("Parsing as str failed").to_string()
}