use egui::{Color32, RichText, Vec2};

use crate::{
    api,
    defines::{AF_FOLDER_CODE, AF_GROUP_CODE, AF_REFRESH_CODE},
    ui::daenerys::DaenerysApp,
};

pub fn display_left_panel(app: &mut DaenerysApp, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    egui::SidePanel::left("group_and_directory_list")
        .resizable(false)
        .show(ctx, |ui| {
            ui.set_width(300.0);

            //
            // Refresh directory list button.
            //
            ui.horizontal_top(|ui| {
                let button = egui::Button::new(AF_REFRESH_CODE.to_string());

                if ui.add_sized([30., 30.], button).clicked() {
                    app.get_directories_promise = Some(api::directory::get_root_directories(ctx));
                }

                ui.label("my root directories".to_string());
            });

            ui.separator();

            //
            // Directories buttons.
            //
            ui.vertical_centered_justified(|ui| {
                if app.directories.is_some() {
                    egui::Grid::new("directory_list")
                        .num_columns(2)
                        .show(ui, |ui| {
                            for directory in app.directories.as_ref().unwrap().iter() {
                                ui.add_sized(
                                    [30., 30.],
                                    egui::Label::new(format!("{}", AF_FOLDER_CODE)),
                                );

                                ui.horizontal(|ui| {
                                    let button_label = directory.name.to_string();

                                    let button = egui::Button::new(button_label);

                                    // Save the clicked directory name.
                                    if ui.add_sized([200., 30.], button).clicked() {
                                        app.display_directory_button_clicked =
                                            Some(directory.clone());
                                        app.display_group_button_clicked = None;
                                        app.edit_directory_clicked = None;
                                        app.edit_group_clicked = None;
                                        app.edit_directory_add_user_clicked = false;
                                        app.edit_directory_add_group_clicked = false;
                                    };
                                });

                                ui.end_row()
                            }
                        });
                }
            });

            ui.separator();

            //
            // Refresh group list button.
            //
            ui.horizontal_top(|ui| {
                let button = egui::Button::new(AF_REFRESH_CODE.to_string());

                if ui.add_sized([30., 30.], button).clicked() {
                    app.get_groups_promise = Some(api::group::get_groups(ctx));
                }

                ui.label("my storm groups".to_string());
            });

            ui.separator();

            //
            // Groups buttons.
            //
            ui.vertical_centered_justified(|ui| {
                if app.groups.is_some() {
                    egui::Grid::new("group_list").num_columns(2).show(ui, |ui| {
                        for group in app.groups.as_ref().unwrap().iter() {
                            ui.add_sized(
                                [30., 30.],
                                egui::Label::new(format!("{}", AF_GROUP_CODE)),
                            );

                            ui.horizontal(|ui| {
                                let button_label = group.cn.to_string();

                                let button = egui::Button::new(button_label);

                                // Save the clicked group name.
                                if ui.add_sized([200., 30.], button).clicked() {
                                    app.display_group_button_clicked = Some(group.clone());
                                    app.display_directory_button_clicked = None;
                                    app.edit_directory_clicked = None;
                                    app.edit_group_clicked = None;
                                }
                            });

                            ui.end_row()
                        }
                    });
                }
            });
        });
}
