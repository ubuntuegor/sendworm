use std::path::PathBuf;

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

#[tauri::command]
pub fn compute_nonexisting_path(folder: String, file_name: String) -> String {
    let mut path = PathBuf::from(folder);
    path.push(file_name);
    if !path.exists() {
        return path.to_string_lossy().into_owned();
    }

    let file_stem = path.file_stem().unwrap().to_string_lossy().into_owned();
    let extension = path.extension().unwrap().to_string_lossy().into_owned();
    let mut i = 1;
    path.set_file_name(format!("{} ({}).{}", file_stem, i, extension));

    while path.exists() {
        i += 1;
        path.set_file_name(format!("{} ({}).{}", file_stem, i, extension));
    }

    path.to_string_lossy().into_owned()
}
