use std::sync::Mutex;

use tauri::Manager;
use tauri_plugin_prevent_default::Flags;
use tokio::runtime;

mod completion;
mod receive;
mod send;

#[derive(Default)]
pub struct AppState {
    send_task_handler: Option<send::SendHandler>,
    receive_task_handler: Option<receive::ReceiveHandler>,
}

#[tauri::command]
async fn get_tasks_number() -> usize {
    let metrics = runtime::Handle::current().metrics();
    metrics.num_alive_tasks()
}

#[cfg(not(debug_assertions))]
fn prevent_default_flags() -> Flags {
    Flags::all().difference(Flags::FOCUS_MOVE | Flags::CONTEXT_MENU)
}

#[cfg(debug_assertions)]
fn prevent_default_flags() -> Flags {
    Flags::all().difference(Flags::FOCUS_MOVE | Flags::CONTEXT_MENU | Flags::DEV_TOOLS)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_prevent_default::Builder::new()
                .with_flags(prevent_default_flags())
                .build(),
        )
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_tasks_number,
            completion::get_completions,
            send::compute_file_name,
            send::get_file_size,
            send::is_folder,
            send::send_file_or_folder,
            send::confirm_send,
            send::cancel_send,
            receive::receive_file,
            receive::confirm_receive,
            receive::cancel_receive,
        ])
        .setup(|app| {
            app.manage(Mutex::new(AppState::default()));
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
