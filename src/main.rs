mod api;
mod defines;
mod error;
mod ui;
mod worker;

use eframe::egui;
use log::info;
use ui::daenerys::DaenerysApp;

fn main() -> Result<(), eframe::Error> {
    // Set window options.
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(400.0, 400.0)),
        ..Default::default()
    };

    info!("Creating app.");

    // Create GUI.
    eframe::run_native(
        "STORM Daenerys (Mésocentre UCA)",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Box::new(DaenerysApp::new(cc, "http://localhost:3000".to_string()))
        }),
    )
}
