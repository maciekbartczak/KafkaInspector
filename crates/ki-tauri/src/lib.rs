use log::debug;
use tauri::Manager;
use tauri::State;
use tauri_plugin_log::Target;
use tokio::sync::mpsc;
use tokio::sync::oneshot;
use tokio::task::JoinHandle;

enum AppMessage {
    SpawnMetadataFetcher(
        ki_core::ConnectToClusterParams,
        oneshot::Sender<Result<(), String>>,
    ),
}

struct AppState {
    message_sender: mpsc::Sender<AppMessage>,
}

struct InternalState {
    metadata_fetcher_handle: Option<JoinHandle<()>>,
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
    params: ki_core::ConnectToClusterParams,
) -> Result<(), String> {
    let (sender, receiver) = oneshot::channel();
    state
        .message_sender
        .send(AppMessage::SpawnMetadataFetcher(params, sender))
        .await
        .map_err(|e| e.to_string());

    receiver.await.unwrap()
}

#[tokio::main]
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() {
    let (sender, mut receiver) = mpsc::channel(100);
    let state = AppState::new(sender);

    let mut internal_state = InternalState {
        metadata_fetcher_handle: None,
    };

    tokio::spawn(async move {
        while let Some(message) = receiver.recv().await {
            match message {
                AppMessage::SpawnMetadataFetcher(params, sender) => {
                    debug!("Spawning metadata fetcher");
                    let handle = tokio::spawn(async move {
                        let mut interval = tokio::time::interval(std::time::Duration::from_secs(1));

                        match ki_core::connect(&params) {
                            Ok(_) => sender.send(Ok(())),
                            Err(e) => {
                                sender.send(Err(e.to_string())).unwrap();
                                return;
                            }
                        }
                        .unwrap();

                        loop {
                            interval.tick().await;
                            ki_core::connect(&params).unwrap();
                        }
                    });

                    internal_state.metadata_fetcher_handle = Some(handle);
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
        .invoke_handler(tauri::generate_handler![connect])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
