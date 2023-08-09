
use egui::{RichText, Color32, ScrollArea};
use tracing::debug;
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

        egui::SidePanel::left("left").resizable(false)
        .show(ctx, |ui| {

            ui.set_width(200.0);

            ScrollArea::vertical()
                .show_viewport(ui, |ui, _viewport| {

                    ui.horizontal_top(|ui| {

                        // Refresh directory list button.
                        if ui.button(crate::defines::AF_REFRESH_CODE.to_string()).clicked() {
                            app.get_directories_promise = Some(api::directory::get_root_directories(ctx));
                        }

                        // Create new directory button.
                        if ui.button(crate::defines::AF_FOLDER_CREATE_CODE.to_string()).clicked() {
                        }

                    });

                    ui.separator();

                    ui.vertical_centered_justified(|ui| {

                        // Create directories buttons.
                        if app.directories.is_some() {                         

                            for directory in app.directories.as_ref().unwrap().iter() {
                            
                                let button_label = format!("{} {}", crate::defines::AF_FOLDER_CODE, &directory.name);

                                // Save the clicked directory name.
                                if ui.button(button_label).clicked() {
                                    app.directory_button_clicked = Some(directory.name.clone());
                                }

                            }
                
                        }
                    });
                });
        });

        egui::CentralPanel::default().show(ctx, |ui| {

            // If a directory is clicked then display its acls.
            if let Some(d) = &app.directory_button_clicked {

                ui.heading(d);

                let acls = app.directories_map.get(d).unwrap(); // this should never panic as the key always exists

                for acl in acls {
                    ui.label(format!("{} {:?} {}", acl.qualifier, acl.qualifier_cn, acl.perm));
                }

            }

        });

    });

}