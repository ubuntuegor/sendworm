use tauri::App;

#[cfg(target_os = "linux")]
mod gtk_drag_and_drop;
#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "windows")]
mod windows;

#[allow(unreachable_code)]
#[tauri::command]
pub fn get_file_to_send() -> Option<String> {
    #[cfg(target_os = "windows")]
    {
        return windows::get_file_to_send_impl();
    }

    None
}

#[allow(unreachable_code)]
#[tauri::command]
pub async fn open_file(file_path: String) {
    #[cfg(target_os = "linux")]
    {
        linux::open_file_impl(file_path).await;
        return;
    }

    tauri_plugin_opener::open_path::<_, &str>(&file_path, None).unwrap();
}

#[allow(unreachable_code)]
#[tauri::command]
pub async fn reveal_file(file_path: String) {
    #[cfg(target_os = "linux")]
    {
        linux::reveal_file_impl(file_path).await;
        return;
    }

    tauri_plugin_opener::reveal_item_in_dir(&file_path).unwrap();
}

pub fn customize_app(app: &mut App) {
    #[cfg(target_os = "linux")]
    {
        gtk_drag_and_drop::connect(app);
    }
}
