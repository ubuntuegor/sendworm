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
