mod ui;
mod worker;
mod error;

use ui::daenerys::DaenerysApp;
use eframe::egui;

fn main() -> Result<(), eframe::Error> {

    tracing_subscriber::fmt()
    .with_max_level(tracing::Level::INFO)
    .init();

    // Set window options.
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };

    tracing::info!("Creating app.");
    
    // Create GUI.
    eframe::run_native(
        "Daenerys",
        options,
        Box::new(|cc| Box::new(DaenerysApp::new(cc))),
    )

}