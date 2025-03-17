use crate::{
    api,
    defines::{AF_ADD_CODE, AF_FOLDER_CODE, AF_QUOTA_CODE, AF_REFRESH_CODE, AF_WARNING_CODE},
    ui::daenerys::{Action, DaenerysApp},
};
use egui::{vec2, Color32, Layout, Ui};
use human_bytes::human_bytes;
use storm_daenerys_common::types::quota::QuotaUnit;

pub fn render_directory_list(
    app: &mut DaenerysApp,
    ctx: &egui::Context,
    ui: &mut Ui,
    scroll_height: f32,
) {
    ui.style_mut().spacing.item_spacing = vec2(16.0, 16.0);

    ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
        ui.label(
            egui::RichText::new("ROOT DIRECTORIES")
                .size(18.0)
                .color(Color32::WHITE),
        );
    });

    ui.horizontal_top(|ui| {
        ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
            // Reload button.
            let button = egui::Button::new(format!("{} reload", AF_REFRESH_CODE));
            if ui.add_sized([30., 30.], button).clicked() {
                app.get_directories_promise = Some(api::directory::get_root_directories(
                    ctx,
                    app.api_url.clone(),
                ));
            }

            // Create directory button.
            let button_label = format!("{} {}", AF_ADD_CODE, "create directory");
            let button = egui::Button::new(button_label);

            if ui.add_sized([150., 30.], button).clicked() {
                app.active_action = Action::DirectoryCreate;

                app.current_directory = None;
                app.current_group = None;
                app.du = None;

                app.create_directory_name.clear();
            }
        });
    });

    // ui.separator();

    // Directory list.
    egui::ScrollArea::vertical()
        .id_salt("directory_scroll")
        .max_height(scroll_height)
        .show(ui, |ui| {
            ui.style_mut().spacing.item_spacing = vec2(5.0, 5.0);

            if app.directories.is_some() {
                ui.with_layout(Layout::top_down(egui::Align::LEFT), |ui| {
                    for directory in app.directories.as_ref().unwrap().iter() {
                        // Icon.
                        let directory_icon = if directory.valid {
                            format!("{}", AF_FOLDER_CODE)
                        } else {
                            format!("{}", AF_WARNING_CODE)
                        };

                        // Directory quota.
                        let quota = match directory.quota {
                            Some(quota) => {
                                if quota.ne(&0) {
                                    format!("[{}:{}]", AF_QUOTA_CODE, human_bytes(quota as f64))
                                } else {
                                    "".to_string()
                                }
                            }
                            None => "".to_string(),
                        };

                        // Disable button id directory is invalid.
                        let enabled = directory.valid;
                        ui.horizontal(|ui| {
                            ui.add_enabled_ui(enabled, |ui| {
                                let button_label =
                                    format!("{} {} {}", directory_icon, directory.name, quota);
                                let button = egui::Button::new(button_label);

                                // if ui.add_sized([100., 20.], button).clicked() {
                                if ui.add(button).clicked() {
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
                    }
                });
            }
        });
}
