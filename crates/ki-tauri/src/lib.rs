use tauri::ipc::Channel;
use tauri::Manager;
use tauri::State;
use tauri_plugin_log::Target;
use tokio::sync::mpsc;
use tokio::sync::oneshot;
use tokio::task::JoinHandle;
use ki_core::{Metadata, MetadataFetcher};

enum AppMessage {
    SpawnMetadataFetcher(
        ki_core::ConsumerParams,
        Channel<Metadata>,
        oneshot::Sender<Result<(), String>>,
    ),
    RemoveMetadataFetcher,
}

struct AppState {
    message_sender: mpsc::Sender<AppMessage>,
}

struct InternalState {
    metadata_fetcher_task_handle: Option<JoinHandle<()>>,
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
    on_event: Channel<Metadata>
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

                    let handle = tokio::spawn(async move {
                        let metadata_fetcher = MetadataFetcher::new(&params).unwrap();
                        let mut interval = tokio::time::interval(std::time::Duration::from_secs(1));

                        match metadata_fetcher.fetch_metadata() {
                            Ok(_) => {
                                result_sender.send(Ok(())).unwrap();
                            }
                            Err(e) => {
                                sender
                                    .send(AppMessage::RemoveMetadataFetcher)
                                    .await
                                    .unwrap();
                                result_sender.send(Err(e.to_string())).unwrap();
                                return;
                            }
                        };

                        loop {
                            interval.tick().await;
                            let metadata = metadata_fetcher.fetch_metadata().unwrap();
                            on_event.send(metadata).unwrap()
                        }
                    });

                    internal_state.metadata_fetcher_task_handle = Some(handle);
                }
                AppMessage::RemoveMetadataFetcher => {
                    internal_state
                        .metadata_fetcher_task_handle
                        .take()
                        .unwrap()
                        .abort();
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
