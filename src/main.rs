mod api;
mod defines;
mod error;
mod ui;
mod worker;
use eframe::egui;
use log::info;
use std::env;
use ui::daenerys::DaenerysApp;

fn main() -> Result<(), eframe::Error> {
    env_logger::init();

    // Get application version.
    let app_version = env!("CARGO_PKG_VERSION");
    info!("app_version: {app_version}");

    // Set window options.
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([400.0, 400.0]),
        ..Default::default()
    };

    info!("Creating app.");

    // Create GUI.
    eframe::run_native(
        "STORM Daenerys (Mésocentre UCA)",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Ok(Box::new(DaenerysApp::new(
                cc,
                "http://localhost:3000".to_string(),
                app_version.to_string(),
            )))
        }),
    )
}
