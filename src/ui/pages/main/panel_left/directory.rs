use crate::{
    api,
    defines::{AF_FOLDER_CODE, AF_QUOTA_CODE, AF_REFRESH_CODE, AF_WARNING_CODE},
    ui::daenerys::{Action, DaenerysApp},
};
use egui::Ui;
use human_bytes::human_bytes;
use storm_daenerys_common::types::quota::QuotaUnit;

pub fn render_directory_list(
    app: &mut DaenerysApp,
    ctx: &egui::Context,
    ui: &mut Ui,
    scroll_height: f32,
) {
    // Refresh button.
    ui.horizontal_top(|ui| {
        ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
            ui.label(
                egui::RichText::new("my root directories").size(20.0).color(
                    app.state
                        .active_theme
                        .fg_primary_text_color_visuals()
                        .unwrap(),
                ),
            );
        });

        ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
            let button = egui::Button::new(format!("{} reload", AF_REFRESH_CODE));
            if ui.add_sized([30., 30.], button).clicked() {
                app.get_directories_promise = Some(api::directory::get_root_directories(
                    ctx,
                    app.api_url.clone(),
                ));
            }
        });
    });

    // Directory list.
    egui::ScrollArea::vertical()
        .id_source("directory_scroll")
        .max_height(scroll_height)
        .show(ui, |ui| {
            if app.directories.is_some() {
                egui::Grid::new("directory_list")
                    .num_columns(2)
                    .show(ui, |ui| {
                        for directory in app.directories.as_ref().unwrap().iter() {
                            // Icon.
                            let directory_icon = if directory.valid {
                                format!("{}", AF_FOLDER_CODE)
                            } else {
                                format!("{}", AF_WARNING_CODE)
                            };
                            // Text.
                            ui.add_sized([30., 30.], egui::Label::new(directory_icon));

                            // Disable button id directory is invalid.
                            let enabled = directory.valid;
                            ui.horizontal(|ui| {
                                ui.add_enabled_ui(enabled, |ui| {
                                    let button_label = directory.name.to_string();
                                    let button = egui::Button::new(button_label);

                                    if ui.add_sized([200., 30.], button).clicked() {
                                        // Save the clicked directory.
                                        app.active_action = Action::DirectoryEdit;
                                        app.current_directory = Some(Box::new(directory.clone()));

                                        // And its quota in bytes to populate the quota edition input text.
                                        if let Some(quota) = directory.quota {
                                            let quota_in_mb = quota / 1024 / 1024;
                                            app.edited_directory_quota = quota_in_mb.to_string()
                                        } else {
                                            app.edited_directory_quota = 0.to_string();
                                        }

                                        app.edited_directory_quota_unit = QuotaUnit::Megabyte;
                                        app.current_group = None;
                                        app.current_error = None;
                                        app.current_info = None;
                                        app.du = None;
                                    };
                                });
                            });

                            // Directory quota.
                            if let Some(quota) = directory.quota {
                                if quota.ne(&0) {
                                    ui.label(format!(
                                        "{} {}",
                                        AF_QUOTA_CODE,
                                        human_bytes(quota as f64)
                                    ));
                                }
                            }

                            ui.end_row()
                        }
                    });
                ui.add_space(20.0);
            }
        });
}
