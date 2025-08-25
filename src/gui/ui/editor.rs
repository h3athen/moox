use crate::gui::Moox;
use eframe::egui;

pub fn build_editor(app: &mut Moox, ctx: &egui::Context) {
    egui::CentralPanel::default().show(ctx, |ui| {  
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.horizontal(|ui| {
                // Calculate font_id and row_height before the closure to avoid double borrowing
                let font_id = egui::TextStyle::Monospace.resolve(ui.style());
                let row_height = ui.fonts(|f| f.row_height(&font_id));
                let col_width = 28.0;
                ui.vertical(|ui_nums| {
                    ui_nums.spacing_mut().item_spacing.y = 0.0;
                    ui_nums.set_width(col_width);

                    let line_count = app.code.split('\n').count().max(1);
                    for n in 1..=line_count {
                        let txt = egui::RichText::new(format!("{:>2}", n)).font(font_id.clone()).weak();
                        ui_nums.allocate_ui_with_layout(
                            egui::vec2(col_width, row_height),
                            egui::Layout::right_to_left(egui::Align::Center),
                            |ui| {
                                ui.label(txt.clone());
                            },
                        );
                    }
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
                    app.mark_unsaved();
                }
            });
        });
    });
}
