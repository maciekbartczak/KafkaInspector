use crate::tasks::metadata_fetcher_task;
use ki_core::{ConsumerParams, Metadata};
use tauri::ipc::Channel;
use tauri::Manager;
use tauri::State;
use tauri_plugin_log::Target;
use tokio::sync::oneshot;
use tokio::sync::{mpsc, Mutex};
use tokio::task::JoinHandle;

mod tasks;

enum AppMessage {
    SpawnMetadataFetcher(
        ConsumerParams,
        Channel<Metadata>,
        oneshot::Sender<Result<(), String>>,
    ),
    RemoveMetadataFetcher,
    SetLastMetadata(Metadata),
    GetLastMetadata(oneshot::Sender<Metadata>),
    GetIsConnected(oneshot::Sender<bool>),
}

struct AppState {
    message_sender: mpsc::Sender<AppMessage>,
    last_connection_params: Option<ConsumerParams>,
}

struct InternalState {
    metadata_fetcher_task_handle: Option<JoinHandle<()>>,
    last_metadata: Metadata,
}

impl AppState {
    fn new(sender: mpsc::Sender<AppMessage>) -> Self {
        Self {
            message_sender: sender,
            last_connection_params: None,
        }
    }
}

#[tauri::command]
async fn connect(
    state: State<'_, Mutex<AppState>>,
    params: ConsumerParams,
    on_event: Channel<Metadata>,
) -> Result<(), String> {
    let mut state = state.lock().await;

    let (sender, receiver) = oneshot::channel();
    state
        .message_sender
        .send(AppMessage::SpawnMetadataFetcher(
            params.clone(),
            on_event,
            sender,
        ))
        .await
        .map_err(|e| format!("failed to send message: {}", e))?;

    receiver.await.unwrap()?;

    state.last_connection_params = Some(params);
    Ok(())
}

#[tauri::command]
async fn reconnect_if_possible(
    state: State<'_, Mutex<AppState>>,
    metadata_channel: Channel<Metadata>,
) -> Result<Option<Metadata>, String> {
    let state = state.lock().await;

    let (sender, receiver) = oneshot::channel();
    state
        .message_sender
        .send(AppMessage::GetIsConnected(sender))
        .await
        .map_err(|e| format!("failed to send message: {}", e))?;
    let is_connected = receiver.await.unwrap();

    if !is_connected {
        return Ok(None);
    }

    state
        .message_sender
        .send(AppMessage::RemoveMetadataFetcher)
        .await
        .map_err(|e| format!("failed to send message: {}", e))?;

    let (sender, receiver) = oneshot::channel();
    state
        .message_sender
        .send(AppMessage::SpawnMetadataFetcher(
            state.last_connection_params.clone().unwrap(),
            metadata_channel,
            sender,
        ))
        .await
        .map_err(|e| format!("failed to send message: {}", e))?;
    receiver.await.unwrap()?;

    let (sender, receiver) = oneshot::channel();
    state
        .message_sender
        .send(AppMessage::GetLastMetadata(sender))
        .await
        .map_err(|e| format!("failed to send message: {}", e))?;
    let metadata = receiver.await.unwrap();

    Ok(Some(metadata))
}

#[tauri::command]
async fn disconnect(state: State<'_, Mutex<AppState>>) -> Result<(), String> {
    let state = state.lock().await;

    state
        .message_sender
        .send(AppMessage::RemoveMetadataFetcher)
        .await
        .map_err(|e| format!("failed to send message: {}", e))?;
    Ok(())
}

#[tokio::main]
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() {
    let (sender, mut receiver) = mpsc::channel(100);
    let state = AppState::new(sender.clone());

    let mut internal_state = InternalState {
        metadata_fetcher_task_handle: None,
        last_metadata: Metadata::default(),
    };

    // main loop that handles all app state changes
    tokio::spawn(async move {
        while let Some(message) = receiver.recv().await {
            match message {
                AppMessage::SpawnMetadataFetcher(params, on_event, result_sender) => {
                    if let Some(_) = internal_state.metadata_fetcher_task_handle {
                        result_sender
                            .send(Err("Metadata fetcher already running".to_string()))
                            .unwrap();
                        continue;
                    }

                    let sender = sender.clone();
                    let handle = tokio::spawn(metadata_fetcher_task(
                        params,
                        on_event,
                        sender,
                        result_sender,
                    ));

                    internal_state.metadata_fetcher_task_handle = Some(handle);
                }
                AppMessage::RemoveMetadataFetcher => {
                    internal_state
                        .metadata_fetcher_task_handle
                        .take()
                        .unwrap()
                        .abort();
                }
                AppMessage::SetLastMetadata(metadata) => {
                    internal_state.last_metadata = metadata;
                }
                AppMessage::GetLastMetadata(result_sender) => {
                    result_sender
                        .send(internal_state.last_metadata.clone())
                        .map_err(|_| "failed to send result")
                        .unwrap();
                }
                AppMessage::GetIsConnected(result_sender) => {
                    result_sender
                        .send(internal_state.metadata_fetcher_task_handle.is_some())
                        .map_err(|_| "failed to send result")
                        .unwrap();
                }
            }
        }
    });

    tauri::Builder::default()
        .setup(|app| {
            #[cfg(debug_assertions)]
            app.get_webview_window("main").unwrap().open_devtools();
            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .plugin(
            tauri_plugin_log::Builder::new()
                .clear_targets()
                .target(Target::new(tauri_plugin_log::TargetKind::Stdout))
                .level(log::LevelFilter::Debug)
                .build(),
        )
        .manage(Mutex::new(state))
        .invoke_handler(tauri::generate_handler![
            connect,
            disconnect,
            reconnect_if_connected
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
