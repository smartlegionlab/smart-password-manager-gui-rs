mod app;
mod clipboard;
mod style;
mod theme;
mod ui;

use anyhow::Result;
use clap::Parser;
use eframe::egui;

use app::{APP_NAME, APP_VERSION, App};

#[derive(Parser, Debug)]
#[command(
    name = APP_NAME,
    version = APP_VERSION,
    about = "A cross-platform GUI for smart password management",
    long_about = "Smart Password Manager is a cross-platform desktop application\n\
                  for managing smart passwords: passwords regenerated on demand\n\
                  from a secret phrase instead of being stored.\n\n\
                  It stores only public keys, descriptions and lengths — never\n\
                  secrets or passwords.\n\n\
                  Author:  Alexander Suvorov\n\
                  License: BSD-3-Clause\n\
                  Repo:    https://github.com/smartlegionlab/smart-password-manager-gui-rs\n\
                  Website: https://smartlegionlab.github.io"
)]
struct Cli {}

fn main() -> Result<()> {
    let _cli = Cli::parse();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([820.0, 520.0])
            .with_min_inner_size([560.0, 360.0])
            .with_title("Smart Password Manager"),
        ..Default::default()
    };

    eframe::run_native(
        "Smart Password Manager",
        options,
        Box::new(|cc| {
            style::apply_theme(&cc.egui_ctx);
            Ok(Box::new(App::new(cc)))
        }),
    )
    .map_err(|e| anyhow::anyhow!("Failed to run GUI: {e}"))?;

    Ok(())
}
