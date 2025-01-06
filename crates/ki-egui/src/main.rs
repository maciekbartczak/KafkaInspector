#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use std::sync::mpsc::Sender;

use eframe::egui;
use ki_core::rt::Command;

fn main() -> eframe::Result {
    env_logger::init();

    let command_tx = ki_core::rt::spawn();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Ok(Box::new(MyApp {
                name: "Arthur".to_owned(),
                age: 42,
                command_tx,
            }))
        }),
    )
}

struct MyApp {
    name: String,
    age: u32,
    command_tx: Sender<Command>,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name)
                    .labelled_by(name_label.id);
            });
            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            if ui.button("Increment").clicked() {
                self.age += 1;
                self.command_tx
                    .send(Command::Greet(self.name.clone()))
                    .unwrap();
            }
            ui.label(format!("Hello '{}', age {}", self.name, self.age));
        });
    }
}
