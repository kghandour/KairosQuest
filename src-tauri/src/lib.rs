mod storage;
use storage::settings;
use storage::workspace;
use tauri_plugin_store::StoreExt;
use tauri_plugin_fs::FsExt;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            settings::save_to_store,
            settings::get_all_config,
            settings::check_first_run,
            workspace::create_file
        ]).setup(|app| {
            let store = app.store(settings::STORE_FILE).expect("could not load config");
            let ws_path = store.get("workspace_path").expect("Workspace path not found");
            let scope = app.fs_scope();
            match scope.allow_directory(std::path::Path::new(&ws_path.to_string()), true) {
                Ok(_) => println!("Scope granted"),
                Err(e) => println!("Error granting directory permission {}", e)
            };
      

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
