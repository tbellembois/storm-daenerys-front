
use egui::{RichText, Color32, ScrollArea};
use tracing_subscriber::fmt::format;

use crate::{ui::daenerys::DaenerysApp, worker::message::{ToWorker, ToWorkerMessage}, error::apperror::AppError};
use crate::api;

pub fn update(app: &mut DaenerysApp, ctx: &egui::Context, _frame: &mut eframe::Frame) {

    egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {

        // Current error label.
        if let Some(current_error) = &app.current_error { 
            ui.label(RichText::new(current_error.to_string()).color(Color32::RED));
        }

        // Current info label.
        if let Some(current_info) = &app.current_info { 
            ui.label(RichText::new(current_info.to_string()));
        }

    });

    egui::CentralPanel::default().show(ctx, |ui| {

        // A test button.
        // if ui.button("ping").clicked() {
            
        //     if let Some(sender) = &app.sender {
        //         if sender.send(ToWorker{ message: ToWorkerMessage::Ping}).is_err() {
        //             app.current_error = Some(AppError::ChannelSendError);
        //         };
        //     }

        // }

        // Trigger an error button.
        // if ui.button("error").clicked() {
        //     app.current_error = Some(AppError::TestError);
        // }

        egui::SidePanel::left("left").resizable(false)
        .show(ctx, |ui| {

            ui.set_width(200.0);

            ScrollArea::vertical()
                .show_viewport(ui, |ui, _viewport| {

                    ui.horizontal_top(|ui| {

                        if ui.button(crate::defines::AF_REFRESH_CODE.to_string()).clicked() {
                        }

                        if ui.button(crate::defines::AF_FOLDER_CREATE_CODE.to_string()).clicked() {
                        }

                    });

                    ui.separator();

                    ui.vertical_centered_justified(|ui| {

                        if app.directories.is_some() {                         

                            for directory in app.directories.as_ref().unwrap().iter() {
                            
                                let button_label = format!("{} {}", crate::defines::AF_FOLDER_CODE, &directory.name);

                                if ui.button(button_label).clicked() {
                                }
                            }
                
                        }
                    });
                });
        });

    });

}