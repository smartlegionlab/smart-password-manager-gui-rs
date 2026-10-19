use eframe::egui;
use egui_native_look as look;

use crate::app::{App, MAX_DESCRIPTION_LEN, MAX_PASSWORD_LEN, MIN_PASSWORD_LEN, MIN_SECRET_LEN};
use crate::style;
use crate::theme;

pub mod add {
    use super::*;

    pub fn render(ctx: &egui::Context, app: &mut App) {
        let mut open = true;
        let mut close = false;
        let mut submit = false;

        egui::Window::new(egui::RichText::new("Add Entry").color(theme::PRIMARY).strong())
            .collapsible(false)
            .resizable(false)
            .open(&mut open)
            .show(ctx, |ui| {
                ui.set_min_width(460.0);

                ui.label(egui::RichText::new("Description").strong());
                ui.add(egui::TextEdit::singleline(&mut app.add.description).hint_text("e.g. GitHub Account").desired_width(f32::INFINITY));
                ui.label(style::muted(format!("{}/{}", app.add.description.chars().count(), MAX_DESCRIPTION_LEN)));

                ui.add_space(10.0);

                ui.label(egui::RichText::new(format!("Secret phrase (min {MIN_SECRET_LEN} chars)")).strong());
                ui.horizontal(|ui| {
                    let edit = egui::TextEdit::singleline(&mut app.add.secret)
                        .password(!app.add.secret_visible)
                        .desired_width(ui.available_width() - 90.0);
                    ui.add(edit);
                    if ui.add(look::Button::new(if app.add.secret_visible { "Hide" } else { "Show" })).clicked() {
                        app.add.secret_visible = !app.add.secret_visible;
                    }
                });
                ui.label(style::muted(format!("{} chars", app.add.secret.chars().count())));

                ui.add_space(10.0);

                ui.label(egui::RichText::new(format!("Password length ({MIN_PASSWORD_LEN}–{MAX_PASSWORD_LEN})")).strong());
                ui.add(egui::TextEdit::singleline(&mut app.add.length).desired_width(120.0));

                if let Some(err) = &app.add.error {
                    ui.add_space(10.0);
                    ui.colored_label(theme::DANGER, err);
                }

                ui.add_space(14.0);
                ui.horizontal(|ui| {
                    let valid = app.add.valid();
                    if ui.add_enabled(valid, look::Button::new("Create").accent()).clicked() {
                        submit = true;
                    }
                    if ui.add(look::Button::new("Cancel")).clicked() {
                        close = true;
                    }
                });
            });

        if submit {
            app.submit_add();
        }
        if close || !open {
            app.close_dialog();
        }
    }
}

pub mod get {
    use super::*;

    pub fn render(ctx: &egui::Context, app: &mut App) {
        let mut open = true;
        let mut close = false;
        let mut submit = false;
        let mut copy_pwd: Option<String> = None;

        let (description, key) = match &app.get {
            Some(s) => (s.description.clone(), s.key.clone()),
            None => { app.close_dialog(); return; }
        };

        egui::Window::new(egui::RichText::new("Get Password").color(theme::PRIMARY).strong())
            .collapsible(false)
            .resizable(false)
            .open(&mut open)
            .show(ctx, |ui| {
                ui.set_min_width(500.0);
                ui.label(egui::RichText::new(format!("Entry: {description}")).strong());
                ui.label(style::muted(format!("Key: {}", App::short_key(&key))));
                ui.separator();

                let state = match &mut app.get {
                    Some(s) => s,
                    None => return,
                };

                if state.password.is_none() {
                    ui.label(egui::RichText::new(format!("Secret phrase (min {MIN_SECRET_LEN} chars)")).strong());
                    ui.horizontal(|ui| {
                        let edit = egui::TextEdit::singleline(&mut state.secret)
                            .password(!state.secret_visible)
                            .desired_width(ui.available_width() - 90.0);
                        ui.add(edit);
                        if ui.add(look::Button::new(if state.secret_visible { "Hide" } else { "Show" })).clicked() {
                            state.secret_visible = !state.secret_visible;
                        }
                    });

                    if let Some(err) = &state.error {
                        ui.add_space(10.0);
                        ui.colored_label(theme::DANGER, err);
                    }

                    ui.add_space(14.0);
                    let valid = state.valid();
                    ui.horizontal(|ui| {
                        if ui.add_enabled(valid, look::Button::new("Generate").accent()).clicked() {
                            submit = true;
                        }
                        if ui.add(look::Button::new("Close")).clicked() {
                            close = true;
                        }
                    });
                } else {
                    ui.label(egui::RichText::new("Generated password").strong());
                    if let Some(pwd) = &state.password {
                        let mut display = pwd.clone();
                        ui.add(egui::TextEdit::singleline(&mut display).desired_width(f32::INFINITY).font(egui::TextStyle::Monospace));
                    }

                    ui.add_space(14.0);
                    ui.horizontal(|ui| {
                        if ui.add(look::Button::new("Copy to clipboard").accent()).clicked() {
                            if let Some(pwd) = &state.password {
                                copy_pwd = Some(pwd.clone());
                            }
                        }
                        if ui.add(look::Button::new("Close")).clicked() {
                            close = true;
                        }
                    });
                }
            });

        if submit {
            app.submit_get();
        }
        if let Some(pwd) = copy_pwd {
            if app.copy_password(&pwd) {
                app.message = Some(("Copied to clipboard".to_string(), false));
            } else {
                app.message = Some(("No clipboard available".to_string(), true));
            }
        }
        if close || !open {
            app.close_dialog();
        }
    }
}

pub mod edit {
    use super::*;

    pub fn render(ctx: &egui::Context, app: &mut App) {
        let mut open = true;
        let mut close = false;
        let mut submit = false;

        egui::Window::new(egui::RichText::new("Edit Entry").color(theme::PRIMARY).strong())
            .collapsible(false)
            .resizable(false)
            .open(&mut open)
            .show(ctx, |ui| {
                ui.set_min_width(460.0);

                let state = match &mut app.edit {
                    Some(s) => s,
                    None => return,
                };

                ui.label(egui::RichText::new("Description").strong());
                ui.add(egui::TextEdit::singleline(&mut state.description).desired_width(f32::INFINITY));
                ui.label(style::muted(format!("{}/{}", state.description.chars().count(), MAX_DESCRIPTION_LEN)));

                ui.add_space(10.0);

                ui.label(egui::RichText::new(format!("Password length ({MIN_PASSWORD_LEN}–{MAX_PASSWORD_LEN})")).strong());
                ui.add(egui::TextEdit::singleline(&mut state.length).desired_width(120.0));

                if let Some(err) = &state.error {
                    ui.add_space(10.0);
                    ui.colored_label(theme::DANGER, err);
                }

                ui.add_space(14.0);
                ui.horizontal(|ui| {
                    let valid = state.valid();
                    if ui.add_enabled(valid, look::Button::new("Save").accent()).clicked() {
                        submit = true;
                    }
                    if ui.add(look::Button::new("Cancel")).clicked() {
                        close = true;
                    }
                });
            });

        if submit {
            app.submit_edit();
        }
        if close || !open {
            app.close_dialog();
        }
    }
}

pub mod verify {
    use super::*;

    pub fn render(ctx: &egui::Context, app: &mut App) {
        let mut open = true;
        let mut close = false;
        let mut submit = false;

        egui::Window::new(egui::RichText::new("Verify Secret").color(theme::PRIMARY).strong())
            .collapsible(false)
            .resizable(false)
            .open(&mut open)
            .show(ctx, |ui| {
                ui.set_min_width(460.0);

                let state = match &mut app.verify {
                    Some(s) => s,
                    None => return,
                };

                ui.label(egui::RichText::new(format!("Entry: {}", state.description)).strong());
                ui.separator();

                ui.label(egui::RichText::new(format!("Secret phrase (min {MIN_SECRET_LEN} chars)")).strong());
                ui.horizontal(|ui| {
                    let edit = egui::TextEdit::singleline(&mut state.secret)
                        .password(!state.secret_visible)
                        .desired_width(ui.available_width() - 90.0);
                    ui.add(edit);
                    if ui.add(look::Button::new(if state.secret_visible { "Hide" } else { "Show" })).clicked() {
                        state.secret_visible = !state.secret_visible;
                    }
                });

                if let Some(err) = &state.error {
                    ui.add_space(10.0);
                    ui.colored_label(theme::DANGER, err);
                }

                if let Some(result) = state.result {
                    ui.add_space(10.0);
                    if result {
                        ui.colored_label(theme::SUCCESS, "✓ Secret matches");
                    } else {
                        ui.colored_label(theme::DANGER, "✗ Secret does NOT match");
                    }
                }

                ui.add_space(14.0);
                ui.horizontal(|ui| {
                    let valid = state.valid();
                    if ui.add_enabled(valid, look::Button::new("Verify").accent()).clicked() {
                        submit = true;
                    }
                    if ui.add(look::Button::new("Close")).clicked() {
                        close = true;
                    }
                });
            });

        if submit {
            app.submit_verify();
        }
        if close || !open {
            app.close_dialog();
        }
    }
}

pub mod delete {
    use super::*;

    pub fn render(ctx: &egui::Context, app: &mut App) {
        let mut open = true;
        let mut close = false;
        let mut submit = false;

        egui::Window::new(egui::RichText::new("Confirm Delete").color(theme::DANGER).strong())
            .collapsible(false)
            .resizable(false)
            .open(&mut open)
            .show(ctx, |ui| {
                ui.set_min_width(400.0);

                let (description, key) = match &app.delete_target {
                    Some(k) => {
                        let desc = app.manager.get(k).map(|e| e.description.clone()).unwrap_or_default();
                        (desc, k.clone())
                    }
                    None => { app.close_dialog(); return; }
                };

                ui.label(egui::RichText::new(format!("Delete entry '{description}'?")).color(theme::TEXT_NORMAL).size(15.0));
                ui.add_space(4.0);
                ui.label(style::muted(format!("Key: {}", App::short_key(&key))));
                ui.add_space(6.0);
                ui.label(style::muted("Only metadata will be removed. The password can still be regenerated if you remember your secret phrase."));

                ui.add_space(14.0);
                ui.horizontal(|ui| {
                    if ui.add(look::Button::new("Yes, delete").accent()).clicked() {
                        submit = true;
                    }
                    if ui.add(look::Button::new("Cancel")).clicked() {
                        close = true;
                    }
                });
            });

        if submit {
            app.submit_delete();
        }
        if close || !open {
            app.close_dialog();
        }
    }
}

pub mod about {
    use super::*;
    use crate::app::{APP_AUTHOR, APP_LICENSE, APP_NAME, APP_REPO_URL, APP_SITE_URL, APP_VERSION};

    pub fn render(ctx: &egui::Context, app: &mut App) {
        let mut open = true;
        let mut close = false;

        egui::Window::new(egui::RichText::new("About").color(theme::PRIMARY).strong())
            .collapsible(false)
            .resizable(false)
            .open(&mut open)
            .show(ctx, |ui| {
                ui.set_min_width(520.0);

                ui.vertical_centered(|ui| {
                    ui.label(egui::RichText::new("Smart Password Manager").color(theme::PRIMARY).size(22.0).strong());
                    ui.label(style::muted(format!("binary: {APP_NAME}")));
                    ui.add_space(6.0);
                    ui.label(format!("Version {APP_VERSION}"));
                    ui.add_space(10.0);
                });

                ui.separator();

                ui.label(egui::RichText::new("DECENTRALIZED BY DESIGN").color(theme::PRIMARY).strong());
                ui.label(style::muted("• No cloud, no database, no trust required\n• Your secrets never leave your device\n• There is no \"forgot password\" button — you are in complete control"));

                ui.add_space(10.0);

                ui.label(egui::RichText::new("SECURITY MODEL").color(theme::PRIMARY).strong());
                ui.label(style::muted("• Proof of Knowledge: Public keys verify secrets without exposing them\n• Deterministic Security: Same secret + length = same password\n• Zero-Storage: No passwords or secrets are ever stored\n• Local Processing: Secrets never leave your device"));

                ui.add_space(10.0);

                ui.label(egui::RichText::new("TECHNICAL FOUNDATION").color(theme::PRIMARY).strong());
                ui.label(style::muted("Powered by smartpasslib — deterministic password generation library\nKey derivation: 15–30 iterations (private) / 45–60 iterations (public)\nCharacter set: !@#$%^&*()_+-=[]{};:,.<>?/A-Za-z0-9"));

                ui.add_space(14.0);
                ui.separator();
                ui.add_space(8.0);

                ui.horizontal(|ui| {
                    ui.label(style::muted(format!("Copyright (c) 2026 {APP_AUTHOR}")));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.hyperlink_to("Website", APP_SITE_URL);
                        ui.hyperlink_to("Repo", APP_REPO_URL);
                        ui.label(style::muted(format!("License: {APP_LICENSE}")));
                    });
                });

                ui.add_space(10.0);
                ui.vertical_centered(|ui| {
                    if ui.add(look::Button::new("Close")).clicked() {
                        close = true;
                    }
                });
            });

        if close || !open {
            app.close_dialog();
        }
    }
}

pub mod help {
    use super::*;

    pub fn render(ctx: &egui::Context, app: &mut App) {
        let mut open = true;
        let mut close = false;

        egui::Window::new(egui::RichText::new("Help").color(theme::PRIMARY).strong())
            .collapsible(false)
            .resizable(true)
            .default_width(640.0)
            .open(&mut open)
            .show(ctx, |ui| {
                ui.label(egui::RichText::new("How to use Smart Password Manager").size(16.0).strong());
                ui.add_space(8.0);

                let sections = [
                    ("What is a smart password?", "A smart password is not stored anywhere. It is generated deterministically from your secret phrase and a length parameter. The same secret + the same length always produce the same password, on every platform."),
                    ("Creating an entry", "Click '+ Add'. Enter a description (for your own reference), a secret phrase (minimum 12 characters), and a password length (12–100). Only the public verification key is stored — never the secret phrase, never the password."),
                    ("Retrieving a password", "Select an entry and click 'Get'. Enter your secret phrase. If it matches the stored public key, the password is regenerated and can be copied to the clipboard."),
                    ("Verifying a secret", "'Verify' proves that you know the secret phrase without generating or displaying the password."),
                    ("Security warning", "Your secret phrase is the only key to all derived passwords. If you lose it, there is no recovery — by design. There is no cloud, no reset link, no support override."),
                ];

                for (title, text) in sections {
                    ui.add_space(6.0);
                    ui.label(egui::RichText::new(title).color(theme::PRIMARY).strong());
                    ui.label(style::muted(text));
                }

                ui.add_space(14.0);
                ui.vertical_centered(|ui| {
                    if ui.add(look::Button::new("Close")).clicked() {
                        close = true;
                    }
                });
            });

        if close || !open {
            app.close_dialog();
        }
    }
}

pub mod shortcuts {
    use super::*;

    pub fn render(ctx: &egui::Context, app: &mut App) {
        let mut open = true;
        let mut close = false;

        egui::Window::new(egui::RichText::new("Keyboard Shortcuts").color(theme::PRIMARY).strong())
            .collapsible(false)
            .resizable(false)
            .open(&mut open)
            .show(ctx, |ui| {
                ui.set_min_width(400.0);

                let rows = [
                    ("Ctrl+N", "Add new entry"),
                    ("Ctrl+G", "Get password for selected entry"),
                    ("Ctrl+E", "Edit selected entry"),
                    ("Ctrl+R", "Verify secret"),
                    ("Del", "Delete selected entry"),
                    ("F5", "Refresh list"),
                    ("F1", "Help"),
                    ("Ctrl+/", "This dialog"),
                    ("Ctrl+Q", "Exit"),
                    ("Enter", "Get password (when a row is selected)"),
                ];

                egui::Grid::new("shortcuts_grid")
                    .num_columns(2)
                    .spacing([24.0, 6.0])
                    .show(ui, |ui| {
                        for (key, action) in rows {
                            ui.label(egui::RichText::new(key).monospace().color(theme::PRIMARY).strong());
                            ui.label(action);
                            ui.end_row();
                        }
                    });

                ui.add_space(14.0);
                ui.vertical_centered(|ui| {
                    if ui.add(look::Button::new("Close")).clicked() {
                        close = true;
                    }
                });
            });

        if close || !open {
            app.close_dialog();
        }
    }
}