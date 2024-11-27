use eframe::*;
use catppuccin_egui::{LATTE, MOCHA};

fn main() {
    let options = NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1000.0, 600.0]),
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

    // run_native(
    //     "Moox",
    //     options,
    //     Box::new(|_cc| {
    //         Ok(Box::new(app))
    //     }),
    // ).expect("App didnt run properly");
}

#[derive(Default)]
struct Moox {
    theme: ThemeChoice,
}

#[derive(Default, Debug, PartialEq)]
enum ThemeChoice {
    Light,
    #[default]
    Dark,
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

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open").clicked() {
                        todo!()
                    }
                    // Quit Program
                    if ui.button("Quit").clicked() {
                        std::process::exit(0);
                    }
                });

                ui.columns(2, |columns| {
                    columns[1].with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                        egui::ComboBox::from_label("Theme")
                            .selected_text(format!("{:?}", self.theme))
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut self.theme, ThemeChoice::Light, "Light");
                                ui.selectable_value(&mut self.theme, ThemeChoice::Dark, "Dark");
                            })
                    })
                })
            })
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.columns(2, |columns| {
                columns[0].heading("Welcome to Moox");
                // columns[1].with_layout()
            })
        });
    }
}