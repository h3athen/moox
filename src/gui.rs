use eframe::{egui, App, Frame};
use catppuccin_egui::{LATTE, MOCHA};
#[derive(Default)]
pub struct Moox {
    pub(crate) theme: ThemeChoice,
}

#[derive(Default, Debug, PartialEq)]
pub enum ThemeChoice {
    #[default]
    Dark,
    Light
}

impl App for Moox {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        catppuccin_egui::set_theme(
            ctx,
            match self.theme {
                ThemeChoice::Light => LATTE,
                ThemeChoice::Dark  => MOCHA
            },
        );

        //// Set Font size across the application
        ctx.set_pixels_per_point(1.20);


        //// Menu Bar
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    // Quit Program
                    if ui.button("Quit").clicked() {
                        std::process::exit(0);
                    }
                });

                //// Theme Setter Option for Light and Dark
                ui.columns(2, |columns| {
                    columns[1].with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                        egui::ComboBox::from_label("Theme")
                            .selected_text(format!("{:?}", self.theme))
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut self.theme, ThemeChoice::Dark, "Dark");
                                ui.selectable_value(&mut self.theme, ThemeChoice::Light, "Light");
                            });
                    })
                })
            })
        });

        //// Center Panel
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Moox");
        });
    }
}