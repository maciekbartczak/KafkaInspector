use std::{
    sync::mpsc::{channel, Receiver, Sender},
    thread,
};

pub enum Command {
    Greet(String),
}

struct CoreApp {
    command_rx: Receiver<Command>,
}

impl CoreApp {
    pub fn new(command_rx: Receiver<Command>) -> Self {
        Self { command_rx }
    }

    pub fn tick(&self) -> bool {
        match self.command_rx.recv() {
            Ok(command) => self.handle_command(command),
            // Exit the core rt when the sender is disconnected
            Err(_) => return false,
        }

        true
    }

    fn handle_command(&self, command: Command) -> () {
        match command {
            Command::Greet(name) => log::info!("Greetings {}!", name),
        }
    }
}

// Spawns the core runtime and returns a sender that can be used to communicate with it
pub fn spawn() -> Sender<Command> {
    let (tx, rx) = channel();

    let _ = thread::Builder::new()
        .name("ki_core_rt".to_owned())
        .spawn(move || {
            log::info!("core rt starting");
            let core_app = CoreApp::new(rx);

            let mut is_running = true;
            while is_running {
                is_running = core_app.tick();
            }

            log::info!("core rt exitting");
        });

    tx
}
