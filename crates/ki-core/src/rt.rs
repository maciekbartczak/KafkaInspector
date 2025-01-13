use std::{
    sync::{
        mpsc::{channel, Receiver, Sender},
        Arc, RwLock,
    },
    thread,
};

use rdkafka::metadata::Metadata;
use tokio::runtime::Runtime;

use crate::{ConsumerParams, MetadataFetcher};

pub enum Command {
    Greet(String),
    SpawnMetadataFetcher(String),
}

struct CoreApp {
    command_rx: Receiver<Command>,
    update_tx: Sender<()>,
    state: Arc<RwLock<State>>,
}

pub struct State {
    pub metadata: Option<Metadata>,
}

impl Default for State {
    fn default() -> Self {
        Self { metadata: None }
    }
}

impl CoreApp {
    pub fn new(
        command_rx: Receiver<Command>,
        update_tx: Sender<()>,
        state: Arc<RwLock<State>>,
    ) -> Self {
        Self {
            command_rx,
            update_tx,
            state,
        }
    }

    async fn tick(&self) -> bool {
        match self.command_rx.recv() {
            Ok(command) => self.handle_command(command).await,
            // Exit the core rt when the sender is disconnected
            Err(_) => return false,
        };

        true
    }

    async fn handle_command(&self, command: Command) -> () {
        match command {
            Command::Greet(name) => log::info!("Greetings {}!", name),
            Command::SpawnMetadataFetcher(address) => self.spawn_metadata_fetcher(address).await,
        }
    }

    async fn spawn_metadata_fetcher(&self, address: String) {
        let metadata_fetcher = MetadataFetcher::new(&ConsumerParams { address }).unwrap();
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(1));

        let state = self.state.clone();
        let update_tx = self.update_tx.clone();

        tokio::spawn(async move {
            loop {
                interval.tick().await;

                let metadata = Some(metadata_fetcher.fetch_raw().unwrap());

                let mut state = state.write().unwrap();
                state.metadata = metadata;
                drop(state);

                update_tx.send(()).unwrap();

                log::info!("md updated");
            }
        });
    }
}

// Spawns the core runtime and returns a sender that can be used to communicate with it
pub fn spawn() -> (Sender<Command>, Receiver<()>, Arc<RwLock<State>>) {
    let rt = Runtime::new().expect("unable to create tokio rt");

    let (command_tx, command_rx) = channel();
    let (update_tx, update_rx) = channel();
    let state = Arc::new(RwLock::new(State::default()));
    let state_clone = state.clone();

    let _ = thread::Builder::new()
        .name("ki_core_rt".to_owned())
        .spawn(move || {
            log::info!("core rt starting");
            let core_app = CoreApp::new(command_rx, update_tx, state_clone);

            let mut is_running = true;
            rt.block_on(async {
                while is_running {
                    is_running = core_app.tick().await;
                }
            });

            log::info!("core rt exitting");
        });

    (command_tx, update_rx, state)
}
