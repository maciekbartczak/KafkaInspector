#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use std::{
    sync::{mpsc::Sender, Arc, RwLock},
    thread,
};

use eframe::egui;
use egui_extras::{Column, TableBuilder};
use ki_core::rt::{Command, State};

fn main() -> eframe::Result {
    env_logger::init();

    let (command_tx, update_rx, state) = ki_core::rt::spawn();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Kafka Inspector",
        options,
        Box::new(|cc| {
            let ctx = cc.egui_ctx.clone();
            thread::spawn(move || loop {
                if let Ok(_) = update_rx.recv() {
                    ctx.request_repaint();
                }
            });

            Ok(Box::new(KIApp {
                command_tx,
                state,
                cluster_address: "localhost:29092".to_string(),
                connected: false,
                connecting: false,
            }))
        }),
    )
}

struct KIApp {
    command_tx: Sender<Command>,
    state: Arc<RwLock<State>>,
    cluster_address: String,
    connected: bool,
    connecting: bool,
}

impl KIApp {
    fn connect_to_cluster_screen(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            let name_label = ui.label("Cluster address: ");
            ui.text_edit_singleline(&mut self.cluster_address)
                .labelled_by(name_label.id);
        });
        if self.connecting {
            ui.spinner();
        } else {
            if ui.button("Connect").clicked() {
                self.command_tx
                    .send(Command::Greet(self.cluster_address.clone()))
                    .unwrap();
            }
            if ui.button("Spawn").clicked() {
                self.command_tx
                    .send(Command::SpawnMetadataFetcher(self.cluster_address.clone()))
                    .unwrap();
                self.connected = true;
            }
        }
    }

    fn topics_table(&mut self, ui: &mut egui::Ui) {
        match self.state.read().unwrap().metadata.as_ref() {
            Some(metadata) => {
                metadata.topics().iter().for_each(|topic| {
                    ui.horizontal(|ui| {
                        ui.label(topic.name());
                        ui.label(format!("{}", topic.partitions().to_string()));
                    });
                });
            }
            None => {}
        }
    }
}

impl eframe::App for KIApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Kafka Inspector");

            if !self.connected {
                self.connect_to_cluster_screen(ui);
            } else {
                self.topics_table(ui);
            }
        });
    }
}
