use std::{
    sync::mpsc::{channel, Receiver, Sender},
    thread,
};

use tokio::runtime::Runtime;

use crate::{ConsumerParams, MetadataFetcher};

pub enum Command {
    Greet(String),
    SpawnMetadataFetcher(String),
}

struct CoreApp {
    command_rx: Receiver<Command>,
}

impl CoreApp {
    pub fn new(command_rx: Receiver<Command>) -> Self {
        Self { command_rx }
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

        tokio::spawn(async move {
            loop {
                interval.tick().await;

                let md = metadata_fetcher.fetch_metadata().unwrap();
                log::info!("Fetched metadata: {:?}", md);
            }
        });
    }
}

// Spawns the core runtime and returns a sender that can be used to communicate with it
pub fn spawn() -> Sender<Command> {
    let rt = Runtime::new().expect("unable to create tokio rt");

    let (tx, rx) = channel();

    let _ = thread::Builder::new()
        .name("ki_core_rt".to_owned())
        .spawn(move || {
            log::info!("core rt starting");
            let core_app = CoreApp::new(rx);

            let mut is_running = true;
            rt.block_on(async {
                while is_running {
                    is_running = core_app.tick().await;
                }
            });

            log::info!("core rt exitting");
        });

    tx
}
