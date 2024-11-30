use crate::gui::Moox;
use eframe::egui;

pub fn build_footer(app: &Moox, ctx: &egui::Context) {
    // Count number of characters
    let char_count = app
        .code
        .chars()
        .filter(|c| !c.is_whitespace())
        .count()
        .to_string();

    // Count number of words
    let word_count = app
        .code
        .split_whitespace()
        .count()
        .to_string();

    // Count number of lines
    let line_count = app
        .code
        .lines()
        .count()
        .to_string();

    //// Display the counts: character, word, line
    egui::TopBottomPanel::bottom("footer").show(ctx, |ui| {
        ui.columns(2, |columns| {
            columns[1].with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                ui.label(line_count);
                ui.label("lin: ");
                ui.separator();

                ui.label(word_count);
                ui.label("wrd: ");
                ui.separator();

                ui.label(char_count);
                ui.label("chr: ");
            });
        });
    });
}
