pub mod ui;

use eframe::{egui, App, Frame};
use std::path::PathBuf;
use ui::menu;

pub struct Moox {
    code: String,
    current_file: Option<PathBuf>,
    is_saved: bool,
    ui_initialized: bool,
    line_count: usize,
    word_count: usize,
    char_count: usize,
    line_numbers_cache: String,
    stats_dirty: bool,
    last_stats_refresh_time: f64,
}

impl Default for Moox {
    fn default() -> Self {
        let mut app = Self {
            code: String::new(),
            current_file: None,
            is_saved: true,
            ui_initialized: false,
            line_count: 1,
            word_count: 0,
            char_count: 0,
            line_numbers_cache: String::new(),
            stats_dirty: false,
            last_stats_refresh_time: 0.0,
        };
        app.refresh_cached_text_data();
        app
    }
}

impl App for Moox {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        if !self.ui_initialized {
            ctx.set_pixels_per_point(1.25);
            apply_glass_theme(ctx);
            self.ui_initialized = true;
        }

        // Keybind for Save File
        if ctx.input(|i| i.key_pressed(egui::Key::S) && i.modifiers.matches_logically(egui::Modifiers::CTRL)) {
            if menu::save_file(self) {
                self.mark_saved();
            }
        }

        // Keybind for Open File
        if ctx.input(|i| i.key_pressed(egui::Key::O) && i.modifiers.matches_logically(egui::Modifiers::CTRL)) {
            menu::open_file(self);
        }

        self.refresh_stats_if_due(ctx);

        //// Call UI components
        ui::menu::build_menu(self, ctx);
        ui::footer::build_footer(self, ctx);
        ui::editor::build_editor(self, ctx);

    }
}

fn apply_glass_theme(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();
    let visuals = &mut style.visuals;

    visuals.dark_mode = true;
    visuals.override_text_color = Some(egui::Color32::from_rgb(220, 226, 238));

    // Keep it minimal and translucent so native window transparency can show through.
    visuals.panel_fill = egui::Color32::from_rgba_unmultiplied(8, 12, 20, 58);
    visuals.window_fill = egui::Color32::from_rgba_unmultiplied(10, 14, 22, 70);
    visuals.window_stroke = egui::Stroke::new(
        1.0,
        egui::Color32::from_rgba_unmultiplied(224, 234, 255, 90),
    );
    visuals.window_rounding = egui::Rounding::same(10.0);
    visuals.window_shadow = egui::epaint::Shadow {
        offset: egui::vec2(0.0, 4.0),
        blur: 14.0,
        spread: 1.0,
        color: egui::Color32::from_black_alpha(80),
    };
    visuals.menu_rounding = egui::Rounding::same(8.0);

    visuals.extreme_bg_color = egui::Color32::from_rgba_unmultiplied(7, 12, 24, 128);
    visuals.faint_bg_color = egui::Color32::from_rgba_unmultiplied(90, 120, 185, 10);
    visuals.code_bg_color = egui::Color32::from_rgba_unmultiplied(8, 14, 28, 112);

    visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgba_unmultiplied(20, 26, 40, 72);
    visuals.widgets.noninteractive.bg_stroke = egui::Stroke::new(
        1.0,
        egui::Color32::from_rgba_unmultiplied(223, 234, 255, 72),
    );
    visuals.widgets.noninteractive.rounding = egui::Rounding::same(7.0);

    visuals.widgets.inactive.weak_bg_fill = egui::Color32::from_rgba_unmultiplied(28, 38, 58, 70);
    visuals.widgets.inactive.bg_fill = egui::Color32::from_rgba_unmultiplied(35, 46, 70, 82);
    visuals.widgets.inactive.bg_stroke = egui::Stroke::new(
        1.0,
        egui::Color32::from_rgba_unmultiplied(205, 220, 255, 74),
    );
    visuals.widgets.inactive.rounding = egui::Rounding::same(6.0);

    visuals.widgets.hovered.weak_bg_fill = egui::Color32::from_rgba_unmultiplied(52, 68, 102, 95);
    visuals.widgets.hovered.bg_fill = egui::Color32::from_rgba_unmultiplied(64, 84, 126, 120);
    visuals.widgets.hovered.bg_stroke = egui::Stroke::new(
        1.0,
        egui::Color32::from_rgba_unmultiplied(220, 232, 255, 122),
    );

    visuals.widgets.active.weak_bg_fill = egui::Color32::from_rgba_unmultiplied(66, 89, 132, 118);
    visuals.widgets.active.bg_fill = egui::Color32::from_rgba_unmultiplied(82, 106, 154, 144);
    visuals.widgets.active.bg_stroke = egui::Stroke::new(
        1.0,
        egui::Color32::from_rgba_unmultiplied(231, 240, 255, 150),
    );

    style.spacing.window_margin = egui::Margin::symmetric(12.0, 10.0);
    ctx.set_style(style);
}

impl Moox {
    fn refresh_line_number_cache(&mut self) {
        let new_line_count = self.code.bytes().filter(|&b| b == b'\n').count() + 1;
        if new_line_count == self.line_count && !self.line_numbers_cache.is_empty() {
            return;
        }

        self.line_count = new_line_count;
        let digits = self.line_count.to_string().len().max(2);
        self.line_numbers_cache = (1..=self.line_count)
            .map(|n| format!("{:>width$}", n, width = digits))
            .collect::<Vec<_>>()
            .join("\n");
    }

    fn refresh_footer_stats(&mut self) {
        self.word_count = self.code.split_whitespace().count();
        self.char_count = self.code.chars().filter(|c| !c.is_whitespace()).count();
        self.stats_dirty = false;
    }

    fn refresh_stats_if_due(&mut self, ctx: &egui::Context) {
        if !self.stats_dirty {
            return;
        }

        let now = ctx.input(|i| i.time);
        if now - self.last_stats_refresh_time >= 0.08 {
            self.refresh_footer_stats();
            self.last_stats_refresh_time = now;
        } else {
            ctx.request_repaint_after(std::time::Duration::from_millis(16));
        }
    }

    pub(crate) fn refresh_cached_text_data(&mut self) {
        self.refresh_line_number_cache();
        self.refresh_footer_stats();
        self.last_stats_refresh_time = 0.0;
    }

    pub(crate) fn mark_unsaved_with_refresh(&mut self) {
        self.is_saved = false;
        self.refresh_line_number_cache();
        self.stats_dirty = true;
    }

    pub fn mark_saved(&mut self) {
        self.is_saved = true;
    }
}
