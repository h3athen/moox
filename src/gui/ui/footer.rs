use crate::gui::Moox;
use eframe::egui;

pub fn build_footer(app: &mut Moox, ctx: &egui::Context) {
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

        egui::menu::bar(ui, |ui| {
            display_file_name(app, ui);

            display_counts(line_count, word_count, char_count, ui);
        });
    });
}

fn display_file_name(app: &mut Moox, ui: &mut egui::Ui) {
    // Display file name with saved/un-saved changes
    let file_name = app
    .current_file
    .as_ref()
    .map(|path| path.file_name().unwrap_or_default().to_string_lossy().to_string())
    .unwrap_or_else(|| "Untitled".to_string());

    let file_name_with_indicator = if app.is_saved {
        file_name
    } else {
        format!("{}*",file_name)
    };

    ui.horizontal(|ui|{
        ui.label(file_name_with_indicator);
    });
}

fn display_counts(line_count: String, word_count: String, char_count: String, ui: &mut egui::Ui) {
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
}