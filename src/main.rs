mod api;
mod defines;
mod error;
mod ui;
mod worker;

use eframe::egui;
use ui::daenerys::DaenerysApp;

//TODO: factorize types with backend.
fn main() -> Result<(), eframe::Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    // Set window options.
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(400.0, 240.0)),
        ..Default::default()
    };

    tracing::info!("Creating app.");

    // Create GUI.
    eframe::run_native(
        "STORM Daenerys (Mésocentre UCA)",
        options,
        Box::new(|cc| Box::new(DaenerysApp::new(cc))),
    )
}
