use eframe::*;

#[derive(Default)]
struct Moox {}

impl App for Moox {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
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
                })
            })
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Welcome to Moox");
        });
    }
}

fn main() {
    let options = NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1000.0, 600.0]),
        ..Default::default()
    };

    run_native(
        "Moox",
        options,
        Box::new(|_cc| {
            Ok(Box::new(Moox {}))
        }),
    ).expect("App didnt run properly");
}