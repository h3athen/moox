// use eframe::epaint::tessellator::Path;
use eframe::{egui, App, Frame};
// use catppuccin_egui::{LATTE, MOCHA};
use eframe::egui::global_theme_preference_switch;
use std::fs;
use std::path::PathBuf;

#[derive(Default)]
pub struct Moox {
    // pub(crate) theme: ThemeChoice,
    code: String,
    current_file: Option<PathBuf>,
}

// #[derive(Default, Debug, PartialEq)]
// pub enum ThemeChoice {
//     #[default]
//     Dark,
//     Light
// }

impl App for Moox {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        // catppuccin_egui::set_theme(
        //     ctx,
        //     match self.theme {
        //         ThemeChoice::Light => LATTE,
        //         ThemeChoice::Dark  => MOCHA
        //     },
        // );

        //// Set Font size across the application
        ctx.set_pixels_per_point(1.25);

        // count number of character excluding whitespace
        let charcount = self.code
        .chars()
        .filter(|c| !c.is_whitespace())
        .count()
        .to_string();

        // count number of words
        let wordcount = self.code
            .split_whitespace()
            .count()
            .to_string();

        // count number of lines
        let linecount = self.code
            .lines()
            .count()
            .to_string();


        //// Menu Bar
        egui::TopBottomPanel::top("menu").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {

                // ui.separator();

                ui.menu_button("File", |ui| {
                    // Open file
                    if ui.button("Open").clicked() {
                        if let Some(path) = rfd::FileDialog::new().pick_file() {
                            match fs::read_to_string(&path) {
                                Ok(contents) => {
                                    self.code = contents;
                                    self.current_file = Some(path);
                                }
                                Err(err) => eprintln!("Failed to read file: {}", err),
                            }
                        }
                    }

                    // Save file
                    if ui.button("Save").clicked() {
                        if let Some(path) = &self.current_file {
                            match fs::write(path, &self.code) {
                                Ok(_) => println!("File saved successfully to {:?}", path),
                                Err(err) => eprintln!("Failed to save file: {}", err),
                            }
                        } else {
                            if let Some(path) = rfd::FileDialog::new().save_file() {
                                match fs::write(&path, &self.code) {
                                    Ok(_) => {
                                        println!("File saved successfully to {:?}", path);
                                        self.current_file = Some(path);
                                    }
                                    Err(err) => eprintln!("Failed to save file: {}", err),
                                }
                            }
                        }
                    }
                });

                ui.menu_button("App", |ui| {
                    // Quit App
                    if ui.button("Quit").clicked() {
                        std::process::exit(0);
                    }
                });

                // ui.separator();

                //// Theme Setter Option for Light and Dark
                ui.columns(2, |columns| {
                    columns[1].with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                        // egui::ComboBox::from_label("Theme")
                        //     .selected_text(format!("{:?}", self.theme))
                        //     .show_ui(ui, |ui| {
                        //         ui.selectable_value(&mut self.theme, ThemeChoice::Dark, "Dark");
                        //         ui.selectable_value(&mut self.theme, ThemeChoice::Light, "Light");
                        //     });
                        global_theme_preference_switch(ui);
                        ui.separator();
                    })
                });

            })
        });

        // egui::SidePanel::left("left")
        //     .exact_width(20.0)
        //     .show(ctx, |ui| {
        //         ui.add_space(8.0);
        //         ui.label("1 2");
        // });

        //// Footer Panel
        egui::TopBottomPanel::bottom("footer").show(ctx, |ui| {
            ui.columns(2, |columns| {
                columns[1].with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                    ui.label(linecount);
                    ui.label("lin: ");

                    ui.separator();

                    ui.label(wordcount);
                    ui.label("wrd: ");

                    ui.separator();

                    ui.label(charcount);
                    ui.label("chr: ");
                });
            });
        });

        //// Center Panel
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.add(
                    egui::TextEdit::multiline(&mut self.code)
                        .desired_width(f32::INFINITY)
                        .desired_rows(35)
                        .frame(false)
                        .font(egui::TextStyle::Monospace)
                        .code_editor(),
                );
            });
        });


    }
}