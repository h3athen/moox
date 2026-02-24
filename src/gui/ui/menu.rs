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
        open_file(app);
    }

    ui.menu_button("Open Recent", |ui| {
        open_recent_menu(app, ui);
    });

    //// Save buffer of file if already exists or create new file
    if ui
        .add( 
            egui::Button::new("Save")
                .shortcut_text(ui.ctx().format_shortcut(&save_shortcut)),
        )
        .clicked() {
        if save_file(app) {
            app.mark_saved();
        }
    }
}

fn open_recent_menu(app: &mut Moox, ui: &mut egui::Ui) {
    if app.recent_files().is_empty() {
        ui.label("No recent files");
        return;
    }

    let items = app.recent_files().to_vec();
    for path in items {
        let label = path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("[invalid file name]")
            .to_string();

        let clicked = ui
            .button(label)
            .on_hover_text(path.display().to_string())
            .clicked();

        if clicked {
            let _ = app.open_path(path);
            ui.close_menu();
        }
    }

    ui.separator();
    if ui.button("Clear Recent").clicked() {
        app.clear_recent_files();
        ui.close_menu();
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

pub fn save_file(app: &mut Moox) -> bool {
    if let Some(path) = &app.current_file {
        if let Err(err) = fs::write(path, &app.code) {
            eprintln!("Failed to save file: {}", err);
            false
        } else {
            app.remember_saved_path(path.clone());
            true
        }
    } else if let Some(path) = rfd::FileDialog::new().save_file() {
        if let Err(err) = fs::write(&path, &app.code) {
            eprintln!("Failed to save file: {}", err);
            false
        } else {
            app.remember_saved_path(path);
            true
        }
    } else {
        false
    }
}

pub fn open_file(app: &mut Moox) -> bool {
    if let Some(path) = rfd::FileDialog::new().pick_file() {
        app.open_path(path)
    } else {
        false
    }
}
