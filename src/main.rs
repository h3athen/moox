#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod gui;

use eframe::{egui, NativeOptions};

use crate::gui::Moox;

fn main() {
    let options = NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1250.0, 650.0]),
        ..Default::default()
    };

    let app = Moox {
        theme: Default::default()
    };

    if let Err(e) = eframe::run_native(
        "Moox",
        options,
        Box::new(|_|{
            Ok(Box::new(app))
        }),
    ) {
        eprintln!("{:?}", e);
        std::process::exit(1);
    }
}