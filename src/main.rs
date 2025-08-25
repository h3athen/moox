#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
mod gui;

use eframe::{egui, NativeOptions};

use crate::gui::Moox;

fn main() {
    //// Set App height and width
    // Load logo.png as icon
    let icon = {
        let image_bytes = include_bytes!("../images/logo.png");
        let img = image::load_from_memory(image_bytes).expect("Failed to load logo.png");
        // Resize the image to 200x200 so the app icon appears larger
        let resized = img
            .resize_exact(200, 200, image::imageops::FilterType::Lanczos3)
            .to_rgba8();
        let (width, height) = resized.dimensions();
        let rgba = resized.into_raw();
        std::sync::Arc::new(egui::viewport::IconData { rgba, width, height })
    };
    let options = NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 700.0])
            .with_icon(icon),
        ..Default::default()
    };

    let app = <Moox as Default>::default();

    //// App run
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