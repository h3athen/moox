use crate::gui::Moox;
use eframe::egui::{self};
use std::fs;

/*
* Build menu bar at the top of the screen
*/
pub fn build_menu(app: &mut Moox, ctx: &egui::Context) {
    egui::TopBottomPanel::top("menu").show(ctx, |ui| {
        egui::menu::bar(ui, |ui| {
            ui.menu_button("File", |ui| file_menu(app, ui));

            // if !app.is_saved {
            //     ui.label(RichText::new("*").strong());
            // }

            ui.menu_button("App", |ui| app_menu(ui));

            theme_switcher(ui);
        });
    });
}

//// File Menu
fn file_menu(app: &mut Moox, ui: &mut egui::Ui) {
    let open_shortcut = egui::KeyboardShortcut::new(egui::Modifiers::CTRL, egui::Key::O);
    let save_shortcut = egui::KeyboardShortcut::new(egui::Modifiers::CTRL, egui::Key::S);

    //// Open existing file to buffer
    if ui
        .add( 
            egui::Button::new("Open")
                .shortcut_text(ui.ctx().format_shortcut(&open_shortcut)),
        )
        .clicked() {
        app.mark_saved();
        open_file(app);
    }

    //// Save buffer of file if already exists or create new file
    if ui
        .add( 
            egui::Button::new("Save")
                .shortcut_text(ui.ctx().format_shortcut(&save_shortcut)),
        )
        .clicked() {
        app.mark_saved();
        save_file(app);
    }
}

//// App Menu
fn app_menu(ui: &mut egui::Ui) {
    if ui.button("Quit").clicked() {
        std::process::exit(0);
    }
}

//// Theme Menu
fn theme_switcher(ui: &mut egui::Ui) {
    ui.columns(2, |columns| {
        columns[1].with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
            egui::global_theme_preference_switch(ui);
            ui.separator();
        })
    });
}

pub fn save_file(app: &mut Moox) {
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

pub fn open_file(app: &mut Moox) {
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