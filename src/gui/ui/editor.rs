use crate::gui::Moox;
use eframe::egui;

pub fn build_editor(app: &mut Moox, ctx: &egui::Context) {
    egui::CentralPanel::default().show(ctx, |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            let mut changed = false;

            let editor =
                egui::TextEdit::multiline(&mut app.code)
                    .desired_width(f32::INFINITY)
                    .desired_rows(35)
                    .frame(false)
                    .font(egui::TextStyle::Monospace)
                    .code_editor()
                    .show(ui);

            if editor.response.changed() {
                changed = true;
            }

            if changed {
                app.mark_unsaved();
            }
        });
    });
}
