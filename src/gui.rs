pub mod ui;

use eframe::{egui, App, Frame};
use std::path::PathBuf;

#[derive(Default)]
pub struct Moox {
    code: String,
    current_file: Option<PathBuf>,
}

impl App for Moox {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        ctx.set_pixels_per_point(1.25);

        ui::menu::build_menu(self, ctx);
        ui::editor::build_editor(self, ctx);
        ui::footer::build_footer(self, ctx);

    }
}