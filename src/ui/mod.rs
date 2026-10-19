pub mod dialogs;
pub mod main_window;
pub mod message;

use eframe::egui;

use crate::app::{App, Dialog};

pub fn render(ui: &mut egui::Ui, app: &mut App) {
    main_window::render(ui, app);

    match app.dialog {
        Dialog::None => {}
        Dialog::Add => dialogs::add::render(ui.ctx(), app),
        Dialog::Get => dialogs::get::render(ui.ctx(), app),
        Dialog::Edit => dialogs::edit::render(ui.ctx(), app),
        Dialog::Verify => dialogs::verify::render(ui.ctx(), app),
        Dialog::Delete => dialogs::delete::render(ui.ctx(), app),
        Dialog::About => dialogs::about::render(ui.ctx(), app),
        Dialog::Help => dialogs::help::render(ui.ctx(), app),
        Dialog::Shortcuts => dialogs::shortcuts::render(ui.ctx(), app),
    }

    message::render(ui.ctx(), app);
}