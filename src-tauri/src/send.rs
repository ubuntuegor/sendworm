use std::{path::Path, sync::Mutex};

use magic_wormhole::{transfer, transit, MailboxConnection, Wormhole, WormholeError};
use serde::Serialize;
use tauri::{ipc::Channel, State};
use thiserror::Error;
use tokio::sync::mpsc;

pub enum SendCommand {
    Confirm,
}

pub type SendHandler = mpsc::Sender<SendCommand>;

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase", tag = "event", content = "data")]
pub enum SendEvent {
    #[serde(rename_all = "camelCase")]
    Code { code: String },
    #[serde(rename_all = "camelCase")]
    Connected,
    #[serde(rename_all = "camelCase")]
    TransitInfo {
        connection_type: String,
        address: String,
    },
    #[serde(rename_all = "camelCase")]
    Progress { sent: u64, total: u64 },
    #[serde(rename_all = "camelCase")]
    Finished,
    #[serde(rename_all = "camelCase")]
    Error { message: String },
}

#[derive(Error, Debug)]
enum SendError {
    #[error("Wormhole error: {}", .0)]
    WormholeError(#[from] magic_wormhole::WormholeError),
    #[error("Transfer error: {}", .0)]
    TransferError(#[from] magic_wormhole::transfer::TransferError),
}

#[tauri::command]
pub fn compute_file_name(file_path: &str) -> Option<String> {
    Path::new(&file_path)
        .file_name()
        .and_then(|x| x.to_str())
        .map(|x| x.to_string())
}

async fn create_wormhole(
    code_handler: impl FnOnce(String) -> (),
) -> Result<Wormhole, WormholeError> {
    let mailbox = MailboxConnection::create(transfer::APP_CONFIG, 2).await?;
    code_handler(mailbox.code().to_string());
    Wormhole::connect(mailbox).await
}

async fn send_file_impl(
    file_path: String,
    backend_to_ui: Channel<SendEvent>,
    mut ui_to_backend: mpsc::Receiver<SendCommand>,
) -> Result<(), SendError> {
    let file_name = compute_file_name(&file_path).unwrap();

    let wormhole = tokio::select! {
        wormhole = create_wormhole(|code| { backend_to_ui.send(SendEvent::Code { code }).unwrap(); }) => {
            wormhole?
        }
        _ = async { while ui_to_backend.recv().await.is_some() {} } => {
            return Ok(())
        }
    };

    backend_to_ui.send(SendEvent::Connected).unwrap();

    match ui_to_backend.recv().await {
        Some(SendCommand::Confirm) => {}
        None => return Ok(()),
    }

    {
        let backend_to_ui = backend_to_ui.clone();
        let backend_to_ui2 = backend_to_ui.clone();
        let relay_hint =
            transit::RelayHint::from_urls(None, [transit::DEFAULT_RELAY_SERVER.parse().unwrap()])
                .unwrap();
        transfer::send_file_or_folder(
            wormhole,
            vec![relay_hint],
            &file_path,
            file_name,
            transit::Abilities::ALL,
            |info| {
                let connection_type = match info.conn_type {
                    transit::ConnectionType::Direct => "direct",
                    transit::ConnectionType::Relay { .. } => "relay",
                    _ => "unknown",
                }
                .to_string();
                backend_to_ui
                    .send(SendEvent::TransitInfo {
                        connection_type,
                        address: info.peer_addr.to_string(),
                    })
                    .unwrap();
            },
            move |sent, total| {
                backend_to_ui2
                    .send(SendEvent::Progress { sent, total })
                    .unwrap();
            },
            async { while ui_to_backend.recv().await.is_some() {} },
        )
        .await?;
    }

    Ok(())
}

#[tauri::command]
pub async fn send_file(
    state: State<'_, Mutex<crate::AppState>>,
    file_path: String,
    on_event: Channel<SendEvent>,
) -> Result<(), ()> {
    let (s, r) = mpsc::channel::<SendCommand>(1);
    {
        let mut state = state.lock().unwrap();
        state.send_task_handler = Some(s);
    }

    match send_file_impl(file_path, on_event.clone(), r).await {
        Err(error) => {
            on_event
                .send(SendEvent::Error {
                    message: error.to_string(),
                })
                .unwrap();
        }
        Ok(()) => {
            on_event.send(SendEvent::Finished).unwrap();
        }
    }

    Ok(())
}

#[tauri::command]
pub fn confirm_send(state: State<'_, Mutex<crate::AppState>>) {
    let state = state.lock().unwrap();
    let handler = state.send_task_handler.as_ref().unwrap();
    handler.blocking_send(SendCommand::Confirm).unwrap();
}

#[tauri::command]
pub fn cancel_send(state: State<'_, Mutex<crate::AppState>>) {
    let mut state = state.lock().unwrap();
    state.send_task_handler = None;
}
