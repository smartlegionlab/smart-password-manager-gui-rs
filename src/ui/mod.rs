pub mod dialogs;
pub mod main_window;
pub mod message;

use eframe::egui;

use crate::app::{App, Dialog};

pub fn render(ctx: &egui::Context, app: &mut App) {
    main_window::render(ctx, app);

    match app.dialog {
        Dialog::None => {}
        Dialog::Add => dialogs::add::render(ctx, app),
        Dialog::Get => dialogs::get::render(ctx, app),
        Dialog::Edit => dialogs::edit::render(ctx, app),
        Dialog::Verify => dialogs::verify::render(ctx, app),
        Dialog::Delete => dialogs::delete::render(ctx, app),
        Dialog::About => dialogs::about::render(ctx, app),
        Dialog::Help => dialogs::help::render(ctx, app),
        Dialog::Shortcuts => dialogs::shortcuts::render(ctx, app),
    }

    message::render(ctx, app);
}
