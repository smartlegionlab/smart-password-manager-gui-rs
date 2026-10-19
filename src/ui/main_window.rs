use eframe::egui;
use egui_extras::{Column, TableBuilder};
use egui_native_look as look;

use crate::app::App;
use crate::style;
use crate::theme;

pub fn render(ui: &mut egui::Ui, app: &mut App) {
    egui::Panel::top("menu_bar").show(ui, |ui| render_menu(ui, app));
    egui::Panel::top("header").frame(style::header_frame()).show(ui, |ui| render_header(ui, app));
    egui::Panel::top("toolbar").frame(style::toolbar_frame()).show(ui, |ui| render_toolbar(ui, app));
    egui::Panel::top("search").frame(style::toolbar_frame()).show(ui, |ui| render_search(ui, app));

    egui::Panel::bottom("status").frame(style::status_frame()).show(ui, |ui| render_status(ui, app));
    egui::Panel::bottom("info").frame(style::info_frame()).show(ui, |ui| render_info(ui, app));

    egui::CentralPanel::default()
        .frame(egui::Frame::default().fill(theme::BG_DARK).inner_margin(egui::Margin::symmetric(20, 10)))
        .show(ui, |ui| render_table(ui, app));
}

fn render_menu(ui: &mut egui::Ui, app: &mut App) {
    egui::MenuBar::new().ui(ui, |ui| {
        ui.menu_button("File", |ui| {
            if ui.button("Refresh    F5").clicked() {
                app.refresh();
                ui.close();
            }
            ui.separator();
            if ui.button("Exit       Ctrl+Q").clicked() {
                ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
            }
        });
        ui.menu_button("Passwords", |ui| {
            if ui.button("Create new  Ctrl+N").clicked() {
                app.open_add();
                ui.close();
            }
            let has_sel = app.selected_entry().is_some();
            ui.add_enabled_ui(has_sel, |ui| {
                if ui.button("Get         Ctrl+G").clicked() {
                    app.open_get();
                    ui.close();
                }
                if ui.button("Edit        Ctrl+E").clicked() {
                    app.open_edit();
                    ui.close();
                }
                if ui.button("Verify      Ctrl+R").clicked() {
                    app.open_verify();
                    ui.close();
                }
                if ui.button("Delete      Del").clicked() {
                    app.open_delete();
                    ui.close();
                }
            });
        });
        ui.menu_button("Help", |ui| {
            if ui.button("Help        F1").clicked() {
                app.open_help();
                ui.close();
            }
            if ui.button("Shortcuts   Ctrl+/").clicked() {
                app.open_shortcuts();
                ui.close();
            }
            ui.separator();
            if ui.button("About").clicked() {
                app.open_about();
                ui.close();
            }
        });
    });
}

fn render_header(ui: &mut egui::Ui, app: &App) {
    ui.horizontal(|ui| {
        ui.label(style::title("Smart Password Manager"));
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            let total = app.keys.len();
            let shown = app.filtered_keys.len();
            if shown == total {
                ui.label(style::muted(format!("{total} entries")));
            } else {
                ui.label(style::muted(format!("{shown} / {total} entries")));
            }
        });
    });
}

fn render_toolbar(ui: &mut egui::Ui, app: &mut App) {
    ui.horizontal(|ui| {
        if ui.add(look::Button::new("+ Add").accent()).clicked() {
            app.open_add();
        }

        let has_sel = app.selected_entry().is_some();

        ui.add_enabled_ui(has_sel, |ui| {
            if ui.add(look::Button::new("Get")).clicked() {
                app.open_get();
            }
            if ui.add(look::Button::new("Edit")).clicked() {
                app.open_edit();
            }
            if ui.add(look::Button::new("Verify")).clicked() {
                app.open_verify();
            }
            if ui.add(look::Button::new("Delete")).clicked() {
                app.open_delete();
            }
        });
    });
}

fn render_search(ui: &mut egui::Ui, app: &mut App) {
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new("Search").color(theme::TEXT_MUTED).strong());
        let response = ui.add(
            egui::TextEdit::singleline(&mut app.search)
                .hint_text("Description or public key")
                .desired_width(f32::INFINITY),
        );
        if response.changed() {
            app.apply_filter();
        }
        if !app.search.is_empty() && ui.button("Clear").clicked() {
            app.search.clear();
            app.apply_filter();
        }
    });
}

fn render_table(ui: &mut egui::Ui, app: &mut App) {
    if app.filtered_keys.is_empty() {
        ui.vertical_centered(|ui| {
            ui.add_space(60.0);
            ui.label(
                egui::RichText::new(if app.keys.is_empty() {
                    "No entries yet. Click '+ Add' to create one."
                } else {
                    "No entries match your search."
                })
                .color(theme::TEXT_MUTED)
                .size(15.0),
            );
        });
        return;
    }

    let available = ui.available_height();
    let mut clicked: Option<usize> = None;
    let mut open_get_on_double: Option<usize> = None;

    TableBuilder::new(ui)
        .striped(true)
        .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
        .column(Column::remainder().at_least(200.0))
        .column(Column::initial(80.0).at_least(60.0))
        .column(Column::initial(140.0).at_least(100.0))
        .min_scrolled_height(0.0)
        .max_scroll_height(available - 10.0)
        .header(24.0, |mut header| {
            header.col(|ui| {
                ui.label(egui::RichText::new("Description").color(theme::PRIMARY).strong());
            });
            header.col(|ui| {
                ui.label(egui::RichText::new("Length").color(theme::PRIMARY).strong());
            });
            header.col(|ui| {
                ui.label(egui::RichText::new("Public Key").color(theme::PRIMARY).strong());
            });
        })
        .body(|body| {
            body.rows(26.0, app.filtered_keys.len(), |mut row| {
                let i = row.index();
                let key = &app.filtered_keys[i];

                let entry = match app.manager.get(key) {
                    Some(e) => e,
                    None => return,
                };

                let selected = i == app.selected;

                row.col(|ui| {
                    let text = App::truncate(&entry.description, 80);
                    let label = ui.selectable_label(
                        selected,
                        egui::RichText::new(text).color(if selected {
                            egui::Color32::WHITE
                        } else {
                            theme::TEXT_NORMAL
                        }),
                    );
                    if label.clicked() {
                        clicked = Some(i);
                    }
                    if label.double_clicked() {
                        open_get_on_double = Some(i);
                    }
                });
                row.col(|ui| {
                    ui.label(egui::RichText::new(format!("{}", entry.length)).color(theme::INFO));
                });
                row.col(|ui| {
                    ui.label(
                        egui::RichText::new(App::short_key(key))
                            .monospace()
                            .color(theme::TEXT_MUTED),
                    );
                });
            });
        });

    if let Some(i) = clicked {
        app.selected = i;
    }
    if let Some(i) = open_get_on_double {
        app.selected = i;
        app.open_get();
    }
}

fn render_info(ui: &mut egui::Ui, app: &App) {
    ui.horizontal(|ui| {
        ui.label(style::muted(format!(
            "Storage: {}",
            app.manager.file_path().display()
        )));
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.label(
                egui::RichText::new(format!("{} entries", app.keys.len()))
                    .color(theme::PRIMARY)
                    .strong(),
            );
        });
    });
}

fn render_status(ui: &mut egui::Ui, app: &App) {
    ui.horizontal(|ui| {
        ui.label(style::muted(&app.status));
    });
}