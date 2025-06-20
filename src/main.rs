#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
mod gui;

use eframe::{egui, NativeOptions};

use crate::gui::Moox;

// use image;

fn main() {
    //// Set App height and width
    let options = NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1200.0, 700.0]),
        // TODO: Add app logo
            // .with_icon(std::sync::Arc::new(egui::IconData { 
            //     rgba: image::load_from_memory(include_bytes!("../images/ferris.png")), 
            //     width: 384, 
            //     height: 384,
            // })),
        ..Default::default()
    };

    let app = <Moox as Default>::default();

    //// App run
    if let Err(e) = eframe::run_native(
        "Moox by Aryan (h3athen)",
        options,
        Box::new(|_|{
            Ok(Box::new(app))
        }),
    ) {
        eprintln!("{:?}", e);
        std::process::exit(1);
    }
}