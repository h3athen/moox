use crate::gui::Moox;
use eframe::egui;

pub fn build_editor(app: &mut Moox, ctx: &egui::Context) {
    egui::CentralPanel::default().show(ctx, |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.horizontal_top(|ui| {
                // Keep gutter and editor in sync by using the same font metrics.
                let font_id = egui::TextStyle::Monospace.resolve(ui.style());
                let line_count = app.line_count;
                let gutter_digits = line_count.to_string().len().max(2);
                let glyph_width = ui.fonts(|f| f.glyph_width(&font_id, '8'));
                let col_width = (gutter_digits as f32 * glyph_width + 14.0).max(30.0);
                let gutter_text_color = if ui.visuals().dark_mode {
                    ui.visuals().weak_text_color()
                } else {
                    egui::Color32::from_rgb(92, 98, 112)
                };

                ui.scope(|ui_nums| {
                    ui_nums.set_width(col_width);
                    ui_nums.add(
                        egui::TextEdit::multiline(&mut app.line_numbers_cache)
                            .desired_width(col_width)
                            .desired_rows(35)
                            .frame(false)
                            .interactive(false)
                            .text_color(gutter_text_color)
                            .font(egui::TextStyle::Monospace)
                            .code_editor(),
                    );
                });

                ui.separator();

                // Editor area
                let mut changed = false;

                // Prevent wrapping by giving a very large desired width so the
                // TextEdit will horizontally scroll instead of wrapping.
                let editor = egui::TextEdit::multiline(&mut app.code)
                    .desired_width(10_000.0)
                    .desired_rows(35)
                    .frame(false)
                    .font(egui::TextStyle::Monospace)
                    .code_editor()
                    .show(ui);

                if editor.response.changed() {
                    changed = true;
                }

                if changed {
                    app.mark_unsaved_with_refresh();
                }
            });
        });
    });
}
