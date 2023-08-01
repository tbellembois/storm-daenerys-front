
use egui::{RichText, Color32};

use crate::{ui::daenerys::DaenerysApp, worker::message::{ToWorker, ToWorkerMessage}, error::apperror::AppError};

pub fn update(app: &mut DaenerysApp, ctx: &egui::Context, _frame: &mut eframe::Frame) {

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("Daenerys");

        // A test button.
        if ui.button("ping").clicked() {
            
            if let Some(sender) = &app.sender {
                if sender.send(ToWorker{ message: ToWorkerMessage::Ping}).is_err() {
                    app.current_error = Some(AppError::ChannelSendError);
                };
            }

        }

        // Trigger an error button.
        if ui.button("error").clicked() {
            app.current_error = Some(AppError::TestError);
        }

        // Current error label.
        if let Some(current_error) = &app.current_error { 
            ui.label(RichText::new(current_error.to_string()).color(Color32::RED));
        }

        // Current info label.
        if let Some(current_info) = &app.current_info { 
            ui.label(RichText::new(current_info.to_string()));
        }

    });

}