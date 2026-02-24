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
            ctx.set_pixels_per_point(1.27);
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
    ctx.style_mut_of(egui::Theme::Dark, |style| {
        let visuals = &mut style.visuals;
        visuals.dark_mode = true;
        visuals.override_text_color = Some(egui::Color32::from_rgb(224, 224, 228));

        visuals.panel_fill = egui::Color32::from_rgba_unmultiplied(18, 18, 20, 180);
        visuals.window_fill = egui::Color32::from_rgba_unmultiplied(24, 24, 28, 198);
        visuals.window_stroke = egui::Stroke::new(
            1.0,
            egui::Color32::from_rgba_unmultiplied(220, 220, 228, 90),
        );
        visuals.window_rounding = egui::Rounding::same(10.0);
        visuals.window_shadow = egui::epaint::Shadow {
            offset: egui::vec2(0.0, 4.0),
            blur: 16.0,
            spread: 1.0,
            color: egui::Color32::from_black_alpha(120),
        };
        visuals.menu_rounding = egui::Rounding::same(8.0);

        visuals.extreme_bg_color = egui::Color32::from_rgba_unmultiplied(18, 18, 22, 214);
        visuals.faint_bg_color = egui::Color32::from_rgba_unmultiplied(96, 96, 104, 24);
        visuals.code_bg_color = egui::Color32::from_rgba_unmultiplied(16, 16, 20, 206);

        visuals.widgets.noninteractive.bg_fill =
            egui::Color32::from_rgba_unmultiplied(38, 38, 44, 160);
        visuals.widgets.noninteractive.bg_stroke = egui::Stroke::new(
            1.0,
            egui::Color32::from_rgba_unmultiplied(214, 214, 222, 72),
        );
        visuals.widgets.noninteractive.rounding = egui::Rounding::same(7.0);

        visuals.widgets.inactive.weak_bg_fill =
            egui::Color32::from_rgba_unmultiplied(48, 48, 56, 140);
        visuals.widgets.inactive.bg_fill = egui::Color32::from_rgba_unmultiplied(58, 58, 66, 156);
        visuals.widgets.inactive.bg_stroke = egui::Stroke::new(
            1.0,
            egui::Color32::from_rgba_unmultiplied(198, 198, 208, 74),
        );
        visuals.widgets.inactive.rounding = egui::Rounding::same(6.0);

        visuals.widgets.hovered.weak_bg_fill =
            egui::Color32::from_rgba_unmultiplied(72, 72, 82, 168);
        visuals.widgets.hovered.bg_fill = egui::Color32::from_rgba_unmultiplied(86, 86, 98, 188);
        visuals.widgets.hovered.bg_stroke = egui::Stroke::new(
            1.0,
            egui::Color32::from_rgba_unmultiplied(220, 220, 228, 122),
        );

        visuals.widgets.active.weak_bg_fill =
            egui::Color32::from_rgba_unmultiplied(92, 92, 106, 190);
        visuals.widgets.active.bg_fill = egui::Color32::from_rgba_unmultiplied(110, 110, 124, 208);
        visuals.widgets.active.bg_stroke = egui::Stroke::new(
            1.0,
            egui::Color32::from_rgba_unmultiplied(228, 228, 236, 150),
        );

        style.spacing.window_margin = egui::Margin::symmetric(12.0, 10.0);
    });

    ctx.style_mut_of(egui::Theme::Light, |style| {
        let visuals = &mut style.visuals;
        visuals.dark_mode = false;
        visuals.override_text_color = Some(egui::Color32::from_rgb(36, 40, 48));

        // Softer slate-gray light mode with stronger separation between layers.
        visuals.panel_fill = egui::Color32::from_rgba_unmultiplied(226, 230, 238, 214);
        visuals.window_fill = egui::Color32::from_rgba_unmultiplied(236, 240, 248, 224);
        visuals.window_stroke = egui::Stroke::new(
            1.0,
            egui::Color32::from_rgba_unmultiplied(96, 104, 120, 116),
        );
        visuals.window_rounding = egui::Rounding::same(10.0);
        visuals.window_shadow = egui::epaint::Shadow {
            offset: egui::vec2(0.0, 4.0),
            blur: 14.0,
            spread: 1.0,
            color: egui::Color32::from_black_alpha(54),
        };
        visuals.menu_rounding = egui::Rounding::same(8.0);

        visuals.extreme_bg_color = egui::Color32::from_rgba_unmultiplied(246, 249, 255, 236);
        visuals.faint_bg_color = egui::Color32::from_rgba_unmultiplied(104, 112, 128, 28);
        visuals.code_bg_color = egui::Color32::from_rgba_unmultiplied(243, 247, 255, 236);

        visuals.widgets.noninteractive.bg_fill =
            egui::Color32::from_rgba_unmultiplied(220, 225, 236, 210);
        visuals.widgets.noninteractive.bg_stroke = egui::Stroke::new(
            1.0,
            egui::Color32::from_rgba_unmultiplied(104, 112, 130, 96),
        );
        visuals.widgets.noninteractive.rounding = egui::Rounding::same(7.0);

        visuals.widgets.inactive.weak_bg_fill =
            egui::Color32::from_rgba_unmultiplied(210, 216, 230, 196);
        visuals.widgets.inactive.bg_fill =
            egui::Color32::from_rgba_unmultiplied(202, 209, 224, 208);
        visuals.widgets.inactive.bg_stroke = egui::Stroke::new(
            1.0,
            egui::Color32::from_rgba_unmultiplied(96, 105, 124, 112),
        );
        visuals.widgets.inactive.rounding = egui::Rounding::same(6.0);

        visuals.widgets.hovered.weak_bg_fill =
            egui::Color32::from_rgba_unmultiplied(190, 198, 214, 214);
        visuals.widgets.hovered.bg_fill =
            egui::Color32::from_rgba_unmultiplied(182, 190, 208, 226);
        visuals.widgets.hovered.bg_stroke = egui::Stroke::new(
            1.0,
            egui::Color32::from_rgba_unmultiplied(84, 94, 112, 136),
        );

        visuals.widgets.active.weak_bg_fill =
            egui::Color32::from_rgba_unmultiplied(170, 180, 200, 226);
        visuals.widgets.active.bg_fill =
            egui::Color32::from_rgba_unmultiplied(160, 170, 192, 236);
        visuals.widgets.active.bg_stroke = egui::Stroke::new(
            1.0,
            egui::Color32::from_rgba_unmultiplied(70, 82, 102, 150),
        );

        style.spacing.window_margin = egui::Margin::symmetric(12.0, 10.0);
    });
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
