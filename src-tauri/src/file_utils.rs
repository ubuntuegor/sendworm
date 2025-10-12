use tokio::fs;

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
