mod api;
mod defines;
mod error;
mod ui;
mod worker;

use std::env;

use chrono::{TimeZone, Utc};
use eframe::egui;
use log::info;
use ui::daenerys::DaenerysApp;

fn main() -> Result<(), eframe::Error> {
    // Get compilation time information.
    let source_date_epoch = match env::var("SOURCE_DATE_EPOCH") {
        Ok(val) => {
            dbg!("source_date_epoch: {}", &val);
            Utc.timestamp_opt(val.parse::<i64>().unwrap(), 0).unwrap()
        }
        Err(e) => {
            dbg!("can not get SOURCE_DATE_EPOCH: {}", e);
            Utc::now()
        }
    };
    let compilation_time = format!("{}", source_date_epoch.format("%d/%m/%Y %H:%M"));
    dbg!("compilation_time: {}", &compilation_time);

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

            Box::new(DaenerysApp::new(
                cc,
                "http://localhost:3000".to_string(),
                compilation_time,
            ))
        }),
    )
}
