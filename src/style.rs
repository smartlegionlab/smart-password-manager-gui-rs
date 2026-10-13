use eframe::egui::{self, Color32, Frame, Margin, Rounding, Stroke};

use crate::theme;

pub fn apply_theme(ctx: &egui::Context) {
    let mut visuals = egui::Visuals::dark();

    visuals.panel_fill = theme::BG_DARK;
    visuals.window_fill = theme::BG_MEDIUM;
    visuals.extreme_bg_color = theme::BG_INPUT;
    visuals.faint_bg_color = theme::BG_MEDIUM;
    visuals.code_bg_color = theme::BG_INPUT;

    visuals.window_stroke = Stroke::new(1.0_f32, theme::BORDER);
    visuals.window_rounding = Rounding::same(8.0);
    visuals.menu_rounding = Rounding::same(6.0);

    visuals.widgets.noninteractive.bg_fill = theme::BG_MEDIUM;
    visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0_f32, theme::BORDER);
    visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0_f32, theme::TEXT_NORMAL);
    visuals.widgets.noninteractive.rounding = Rounding::same(5.0);

    visuals.widgets.inactive.bg_fill = theme::BG_LIGHT;
    visuals.widgets.inactive.weak_bg_fill = theme::BG_LIGHT;
    visuals.widgets.inactive.bg_stroke = Stroke::new(1.0_f32, theme::BORDER);
    visuals.widgets.inactive.fg_stroke = Stroke::new(1.0_f32, theme::TEXT_NORMAL);
    visuals.widgets.inactive.rounding = Rounding::same(5.0);

    visuals.widgets.hovered.bg_fill = theme::PRIMARY_HOVER;
    visuals.widgets.hovered.weak_bg_fill = theme::PRIMARY_HOVER;
    visuals.widgets.hovered.bg_stroke = Stroke::new(1.0_f32, theme::PRIMARY);
    visuals.widgets.hovered.fg_stroke = Stroke::new(1.0_f32, Color32::WHITE);
    visuals.widgets.hovered.rounding = Rounding::same(5.0);

    visuals.widgets.active.bg_fill = theme::PRIMARY;
    visuals.widgets.active.weak_bg_fill = theme::PRIMARY;
    visuals.widgets.active.bg_stroke = Stroke::new(1.0_f32, theme::PRIMARY);
    visuals.widgets.active.fg_stroke = Stroke::new(1.0_f32, Color32::WHITE);
    visuals.widgets.active.rounding = Rounding::same(5.0);

    visuals.selection.bg_fill = theme::PRIMARY;
    visuals.selection.stroke = Stroke::new(1.0_f32, Color32::WHITE);

    visuals.hyperlink_color = theme::PRIMARY;
    visuals.warn_fg_color = theme::WARNING;
    visuals.error_fg_color = theme::DANGER;

    ctx.set_visuals(visuals);

    let mut style = (*ctx.style()).clone();
    style.spacing.item_spacing = egui::vec2(8.0, 6.0);
    style.spacing.button_padding = egui::vec2(12.0, 6.0);
    style.spacing.window_margin = Margin::same(12.0);
    style.spacing.scroll.bar_width = 10.0;
    ctx.set_style(style);
}

pub fn header_frame() -> Frame {
    Frame::default()
        .fill(theme::BG_HEADER)
        .inner_margin(Margin::symmetric(20.0, 14.0))
}

pub fn toolbar_frame() -> Frame {
    Frame::default()
        .fill(theme::BG_PANEL)
        .inner_margin(Margin::symmetric(20.0, 10.0))
}

pub fn info_frame() -> Frame {
    Frame::default()
        .fill(theme::BG_PANEL)
        .inner_margin(Margin::symmetric(20.0, 8.0))
}

pub fn status_frame() -> Frame {
    Frame::default()
        .fill(theme::BG_HEADER)
        .inner_margin(Margin::symmetric(20.0, 6.0))
}

pub fn primary_button(text: &str) -> egui::Button<'_> {
    egui::Button::new(egui::RichText::new(text).color(Color32::WHITE).strong())
        .fill(theme::PRIMARY)
        .rounding(Rounding::same(5.0))
}

pub fn success_button(text: &str) -> egui::Button<'_> {
    egui::Button::new(egui::RichText::new(text).color(Color32::WHITE).strong())
        .fill(theme::SUCCESS)
        .rounding(Rounding::same(5.0))
}

pub fn warning_button(text: &str) -> egui::Button<'_> {
    egui::Button::new(egui::RichText::new(text).color(Color32::WHITE).strong())
        .fill(theme::WARNING)
        .rounding(Rounding::same(5.0))
}

pub fn danger_button(text: &str) -> egui::Button<'_> {
    egui::Button::new(egui::RichText::new(text).color(Color32::WHITE).strong())
        .fill(theme::DANGER)
        .rounding(Rounding::same(5.0))
}

pub fn info_button(text: &str) -> egui::Button<'_> {
    egui::Button::new(egui::RichText::new(text).color(Color32::WHITE).strong())
        .fill(theme::INFO)
        .rounding(Rounding::same(5.0))
}

pub fn secondary_button(text: &str) -> egui::Button<'_> {
    egui::Button::new(egui::RichText::new(text).color(Color32::WHITE))
        .fill(theme::SECONDARY)
        .rounding(Rounding::same(5.0))
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
