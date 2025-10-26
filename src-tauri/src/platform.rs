use tauri::App;

#[cfg(target_os = "linux")]
mod gtk_drag_and_drop;
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

pub fn customize_app(app: &mut App) {
    #[cfg(target_os = "linux")]
    {
        gtk_drag_and_drop::connect(app);
    }
}
