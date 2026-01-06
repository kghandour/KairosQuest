use crate::storage::settings::get_workspace_path;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

#[tauri::command]
pub async fn create_file(app: tauri::AppHandle, name: String, content: String) -> String{
    let workspace_path = get_workspace_path(app).await;
    println!("{}",workspace_path);
    let file_path = PathBuf::from(workspace_path).join(name);
    println!("File creating {}", file_path.clone().into_os_string().into_string().unwrap());
    let mut file = File::create(file_path).expect("Could not create file");
    match file.write_all(content.as_bytes()){
        Ok(_) => content,
        Err(_e) => "Could not create file".to_string()
    }
}