use eframe::egui;
use egui_native_look as look;

use crate::app::App;
use crate::theme;

pub fn render(ctx: &egui::Context, app: &mut App) {
    let message = match &app.message {
        Some(m) => m.clone(),
        None => return,
    };

    let mut open = true;
    let (text, is_error) = message;

    egui::Window::new(if is_error {
        egui::RichText::new("Error").color(theme::DANGER).strong()
    } else {
        egui::RichText::new("Info").color(theme::SUCCESS).strong()
    })
    .collapsible(false)
    .resizable(false)
    .open(&mut open)
    .show(ctx, |ui| {
        ui.set_min_width(320.0);
        if is_error {
            ui.colored_label(theme::DANGER, &text);
        } else {
            ui.label(&text);
        }
        ui.add_space(10.0);
        ui.vertical_centered(|ui| {
            if ui.add(look::Button::new("OK").accent()).clicked() {
                app.message = None;
            }
        });
    });

    if !open {
        app.message = None;
    }
}