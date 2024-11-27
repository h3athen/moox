use eframe::{egui, App, Frame};
// use catppuccin_egui::{LATTE, MOCHA};
use eframe::egui::global_theme_preference_switch;

#[derive(Default)]
pub struct Moox {
    // pub(crate) theme: ThemeChoice,
    code: String,
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
        ctx.set_pixels_per_point(1.20);


        //// Menu Bar
        egui::TopBottomPanel::top("menu").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {

                // ui.separator();

                ui.menu_button("File", |ui| {
                    // Open file
                    if ui.button("🗁  Open").clicked() {
                        println!("open");
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

        //// Center Panel
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.add(
                    egui::TextEdit::multiline(&mut self.code)
                        .desired_width(f32::INFINITY)
                        .desired_rows(38)
                        .frame(false)
                        .font(egui::TextStyle::Monospace)
                        .code_editor(),
                );
            });
        });

        //// Footer Panel
        egui::TopBottomPanel::bottom("footer").show(ctx, |ui| {
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
    }
}