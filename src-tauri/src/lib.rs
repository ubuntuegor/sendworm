use std::sync::Mutex;

use tauri::Manager;
use tokio::runtime;

mod send;

#[derive(Default)]
pub struct AppState {
    send_task_handler: Option<send::SendHandler>,
}

#[tauri::command]
async fn get_tasks_number() -> usize {
    let metrics = runtime::Handle::current().metrics();
    metrics.num_alive_tasks()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_tasks_number,
            send::compute_file_name,
            send::send_file,
            send::confirm_send,
            send::cancel_send,
        ])
        .setup(|app| {
            app.manage(Mutex::new(AppState::default()));
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
