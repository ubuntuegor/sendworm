use std::{path::PathBuf, sync::Mutex};

use magic_wormhole::{transfer, transit, Code, MailboxConnection, Wormhole, WormholeError};
use serde::Serialize;
use tauri::{ipc::Channel, State};
use thiserror::Error;
use tokio::{fs, sync::mpsc};
use tokio_util::compat::TokioAsyncWriteCompatExt;

pub enum ReceiveCommand {
    Confirm { folder: String },
}

pub type ReceiveHandler = mpsc::Sender<ReceiveCommand>;

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase", tag = "event", content = "data")]
pub enum ReceiveEvent {
    #[serde(rename_all = "camelCase")]
    FileInfo { file_name: String, file_size: u64 },
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
enum ReceiveError {
    #[error("Wormhole error: {}", .0)]
    WormholeError(#[from] magic_wormhole::WormholeError),
    #[error("Transfer error: {}", .0)]
    TransferError(#[from] magic_wormhole::transfer::TransferError),
    #[error("Failed to parse code: {}", .0)]
    ParseCodeError(#[from] magic_wormhole::ParseCodeError),
    #[error("Failed to create file: {}", .0)]
    FileCreateError(#[from] std::io::Error),
}

async fn connect_wormhole(code: Code) -> Result<Wormhole, WormholeError> {
    let mailbox: MailboxConnection<transfer::AppVersion> =
        MailboxConnection::connect(transfer::APP_CONFIG, code, false).await?;
    Wormhole::connect(mailbox).await
}

async fn receive_file_impl(
    code: String,
    backend_to_ui: Channel<ReceiveEvent>,
    mut ui_to_backend: mpsc::Receiver<ReceiveCommand>,
) -> Result<(), ReceiveError> {
    let code: Code = code.parse()?;

    let wormhole = tokio::select! {
        wormhole = connect_wormhole(code) => {
            wormhole?
        }
        _ = async { while ui_to_backend.recv().await.is_some() {} } => {
            return Ok(())
        }
    };

    let relay_hint =
        transit::RelayHint::from_urls(None, [transit::DEFAULT_RELAY_SERVER.parse().unwrap()])
            .unwrap();
    let request =
        transfer::request_file(wormhole, vec![relay_hint], transit::Abilities::ALL, async {
            while ui_to_backend.recv().await.is_some() {}
        })
        .await?;

    let request = match request {
        Some(r) => r,
        None => {
            return Ok(());
        }
    };

    backend_to_ui
        .send(ReceiveEvent::FileInfo {
            file_name: request.file_name(),
            file_size: request.file_size(),
        })
        .unwrap();

    let folder = match ui_to_backend.recv().await {
        Some(ReceiveCommand::Confirm { folder }) => folder,
        None => return Ok(()),
    };

    let mut file_path = PathBuf::from(folder);
    file_path.push(request.file_name());
    let file = fs::File::create(file_path).await?;

    {
        let backend_to_ui = backend_to_ui.clone();
        let backend_to_ui2 = backend_to_ui.clone();

        request
            .accept(
                |info| {
                    let connection_type = match info.conn_type {
                        transit::ConnectionType::Direct => "direct",
                        transit::ConnectionType::Relay { .. } => "relay",
                        _ => "unknown",
                    }
                    .to_string();
                    backend_to_ui
                        .send(ReceiveEvent::TransitInfo {
                            connection_type,
                            address: info.peer_addr.to_string(),
                        })
                        .unwrap();
                },
                move |sent, total| {
                    backend_to_ui2
                        .send(ReceiveEvent::Progress { sent, total })
                        .unwrap();
                },
                &mut file.compat_write(),
                async { while ui_to_backend.recv().await.is_some() {} },
            )
            .await?;
    }

    Ok(())
}

#[tauri::command]
pub async fn receive_file(
    state: State<'_, Mutex<crate::AppState>>,
    code: String,
    on_event: Channel<ReceiveEvent>,
) -> Result<(), ()> {
    let (s, r) = mpsc::channel::<ReceiveCommand>(1);
    {
        let mut state = state.lock().unwrap();
        state.receive_task_handler = Some(s);
    }

    match receive_file_impl(code, on_event.clone(), r).await {
        Err(error) => {
            on_event
                .send(ReceiveEvent::Error {
                    message: error.to_string(),
                })
                .unwrap();
        }
        Ok(()) => {
            on_event.send(ReceiveEvent::Finished).unwrap();
        }
    }

    Ok(())
}

#[tauri::command]
pub fn confirm_receive(state: State<'_, Mutex<crate::AppState>>, folder: String) {
    let state = state.lock().unwrap();
    let handler = state.receive_task_handler.as_ref().unwrap();
    handler
        .blocking_send(ReceiveCommand::Confirm { folder })
        .unwrap();
}

#[tauri::command]
pub fn cancel_receive(state: State<'_, Mutex<crate::AppState>>) {
    let mut state = state.lock().unwrap();
    state.receive_task_handler = None;
}
