use crate::gui::Moox;
use eframe::egui;
use std::fs;

/*
* Build menu bar at the top of the screen
*/
pub fn build_menu(app: &mut Moox, ctx: &egui::Context) {
    egui::TopBottomPanel::top("menu").show(ctx, |ui| {
        egui::menu::bar(ui, |ui| {
            ui.menu_button("File", |ui| file_menu(app, ui));
            ui.menu_button("App", |ui| app_menu(ui));
            theme_switcher(ui);
        });
    });
}

fn file_menu(app: &mut Moox, ui: &mut egui::Ui) {
    if ui.button("Open...").clicked() {
        if let Some(path) = rfd::FileDialog::new().pick_file() {
            match fs::read_to_string(&path) {
                Ok(contents) => {
                    app.code = contents;
                    app.current_file = Some(path);
                }
                Err(err) => eprintln!("Failed to read file: {}", err),
            }
        }
    }

    if ui.button("Save").clicked() {
        if let Some(path) = &app.current_file {
            if let Err(err) = fs::write(path, &app.code) {
                eprintln!("Failed to save file: {}", err);
            }
        } else if let Some(path) = rfd::FileDialog::new().save_file() {
            if let Err(err) = fs::write(&path, &app.code) {
                eprintln!("Failed to save file: {}", err);
            } else {
                app.current_file = Some(path);
            }
        }
    }
}

fn app_menu(ui: &mut egui::Ui) {
    if ui.button("Quit").clicked() {
        std::process::exit(0);
    }
}

fn theme_switcher(ui: &mut egui::Ui) {
    ui.columns(2, |columns| {
        columns[1].with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
            egui::global_theme_preference_switch(ui);
            ui.separator();
        })
    });
}