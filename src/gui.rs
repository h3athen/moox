pub mod ui;

use eframe::{egui, App, Frame};
use ui::menu;
use std::path::PathBuf;

pub struct Moox {
    code: String,
    current_file: Option<PathBuf>,
    is_saved: bool,
}

impl Default for Moox {
    fn default() -> Self {
        Self {
            code: String::new(),
            current_file: None,
            is_saved: true,
        }
    }
}

impl App for Moox {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        //// Set App font size
        ctx.set_pixels_per_point(1.15);
        if ctx.input(|i| i.key_pressed(egui::Key::S) && i.modifiers.matches_logically(egui::Modifiers::CTRL)) {
            if self.current_file.is_some() {
                menu::save_file(self);
                self.mark_saved();
            } else {
                menu::save_file(self);
                self.mark_saved();
            }
        }

        //// Call UI components
        ui::menu::build_menu(self, ctx);
        ui::editor::build_editor(self, ctx);
        ui::footer::build_footer(self, ctx);

        self.check_unsaved_changes();
    }
}

impl Moox {
    /// Mark the buffer as unsaved if there are changes
    fn check_unsaved_changes(&mut self) {
        // You could use a more sophisticated check like hashing for large files
        if !self.is_saved {
            return; // Already marked as unsaved
        }
        if let Some(path) = &self.current_file {
            // Compare the file's content with the buffer
            if let Ok(content) = std::fs::read_to_string(path) {
                self.is_saved = content == self.code;
            }
        } else {
            // No file associated means unsaved changes
            self.is_saved = false;
        }
    }

    pub fn mark_saved(&mut self) {
        self.is_saved = true;
    }

    pub fn mark_unsaved(&mut self) {
        self.is_saved = false;
    }
}