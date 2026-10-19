use eframe::egui::{self, Color32, Frame, Margin, Stroke};

use crate::theme;

pub fn apply_theme(ctx: &egui::Context) {
    let mut style = (*ctx.style_of(egui::Theme::Dark)).clone();

    style.spacing.item_spacing = egui::vec2(8.0, 6.0);
    style.spacing.button_padding = egui::vec2(12.0, 6.0);
    style.spacing.window_margin = Margin::same(12);
    style.spacing.scroll.bar_width = 10.0;

    ctx.set_style_of(egui::Theme::Dark, style);

    let mut visuals = egui::Visuals::dark();
    visuals.panel_fill = theme::BG_DARK;
    visuals.window_fill = theme::BG_MEDIUM;
    visuals.extreme_bg_color = theme::BG_INPUT;
    visuals.faint_bg_color = theme::BG_MEDIUM;
    visuals.code_bg_color = theme::BG_INPUT;
    visuals.window_stroke = Stroke::new(1.0_f32, theme::BORDER);
    visuals.hyperlink_color = theme::PRIMARY;
    visuals.warn_fg_color = theme::WARNING;
    visuals.error_fg_color = theme::DANGER;
    visuals.selection.bg_fill = theme::PRIMARY;
    visuals.selection.stroke = Stroke::new(1.0_f32, Color32::WHITE);
    ctx.set_visuals_of(egui::Theme::Dark, visuals);
}

pub fn header_frame() -> Frame {
    Frame::default()
        .fill(theme::BG_HEADER)
        .inner_margin(Margin::symmetric(20, 14))
}

pub fn toolbar_frame() -> Frame {
    Frame::default()
        .fill(theme::BG_PANEL)
        .inner_margin(Margin::symmetric(20, 10))
}

pub fn info_frame() -> Frame {
    Frame::default()
        .fill(theme::BG_PANEL)
        .inner_margin(Margin::symmetric(20, 8))
}

pub fn status_frame() -> Frame {
    Frame::default()
        .fill(theme::BG_HEADER)
        .inner_margin(Margin::symmetric(20, 6))
}

pub fn muted(text: impl Into<String>) -> egui::RichText {
    egui::RichText::new(text).color(theme::TEXT_MUTED)
}

pub fn title(text: impl Into<String>) -> egui::RichText {
    egui::RichText::new(text)
        .color(theme::PRIMARY)
        .size(22.0)
        .strong()
}