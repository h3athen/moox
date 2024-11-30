use crate::gui::Moox;
use eframe::egui;

pub fn build_editor(app: &mut Moox, ctx: &egui::Context) {
    egui::CentralPanel::default().show(ctx, |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.add(
                egui::TextEdit::multiline(&mut app.code)
                    .desired_width(f32::INFINITY)
                    .desired_rows(35)
                    .frame(false)
                    .font(egui::TextStyle::Monospace)
                    .code_editor(),
            );
        });
    });
}
