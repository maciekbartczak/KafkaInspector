use crate::tasks::metadata_fetcher_task;
use ki_core::Metadata;
use tauri::ipc::Channel;
use tauri::Manager;
use tauri::State;
use tauri_plugin_log::Target;
use tokio::sync::mpsc;
use tokio::sync::oneshot;
use tokio::task::JoinHandle;

mod tasks;

enum AppMessage {
    SpawnMetadataFetcher(
        ki_core::ConsumerParams,
        Channel<Metadata>,
        oneshot::Sender<Result<(), String>>,
    ),
    RemoveMetadataFetcher,
    SetLastMetadata(Metadata),
    GetLastMetadata(oneshot::Sender<Metadata>),
}

struct AppState {
    message_sender: mpsc::Sender<AppMessage>,
}

struct InternalState {
    metadata_fetcher_task_handle: Option<JoinHandle<()>>,
    last_metadata: Metadata,
}

impl AppState {
    fn new(sender: mpsc::Sender<AppMessage>) -> Self {
        Self {
            message_sender: sender,
        }
    }
}

#[tauri::command]
async fn connect(
    state: State<'_, AppState>,
    params: ki_core::ConsumerParams,
    on_event: Channel<Metadata>,
) -> Result<(), String> {
    let (sender, receiver) = oneshot::channel();
    state
        .message_sender
        .send(AppMessage::SpawnMetadataFetcher(params, on_event, sender))
        .await
        .map_err(|e| format!("failed to send message: {}", e))?;

    receiver.await.unwrap()
}

#[tauri::command]
async fn disconnect(state: State<'_, AppState>) -> Result<(), String> {
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
                        .map_err(|| "failed to send result")
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
        .manage(state)
        .invoke_handler(tauri::generate_handler![connect, disconnect])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
