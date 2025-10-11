use std::path::Path;

use tokio::fs;

#[tauri::command]
pub fn compute_file_name(file_path: &str) -> Option<String> {
    Path::new(&file_path)
        .file_name()
        .and_then(|x| x.to_str())
        .map(|x| x.to_string())
}

#[tauri::command]
pub async fn get_file_size(file_path: String) -> Option<u64> {
    let metadata = fs::metadata(file_path).await.ok()?;
    if !metadata.is_dir() {
        metadata.len().into()
    } else {
        None
    }
}

#[tauri::command]
pub async fn is_folder(file_path: String) -> Option<bool> {
    fs::metadata(file_path).await.ok()?.is_dir().into()
}
