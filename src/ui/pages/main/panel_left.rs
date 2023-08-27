use egui::{Color32, RichText};

use crate::{api, ui::daenerys::DaenerysApp};

pub fn display_left_panel(app: &mut DaenerysApp, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    egui::SidePanel::left("group_and_directory_list")
        .resizable(false)
        .show(ctx, |ui| {
            ui.set_width(200.0);

            //
            // Refresh directory list button.
            //
            ui.horizontal_top(|ui| {
                if ui
                    .button(crate::defines::AF_REFRESH_CODE.to_string())
                    .clicked()
                {
                    app.get_directories_promise = Some(api::directory::get_root_directories(ctx));
                }
            });

            ui.separator();

            //
            // Directories buttons.
            //
            ui.vertical_centered_justified(|ui| {
                if app.directories.is_some() {
                    for directory in app.directories.as_ref().unwrap().iter() {
                        let button_label =
                            format!("{} {}", crate::defines::AF_FOLDER_CODE, &directory.name);

                        // Save the clicked directory name.
                        if ui.button(button_label).clicked() {
                            app.directory_button_clicked = Some(directory.clone());
                            app.group_button_clicked = None;
                            app.edit_directory_clicked = None;
                            app.edit_group_clicked = None;
                        }
                    }
                }
            });

            //
            // Refresh group list button.
            //
            ui.horizontal_top(|ui| {
                if ui
                    .button(crate::defines::AF_REFRESH_CODE.to_string())
                    .clicked()
                {
                    app.get_groups_promise = Some(api::group::get_groups(ctx));
                }
            });

            ui.separator();

            //
            // Groups buttons.
            //
            ui.vertical_centered_justified(|ui| {
                if app.groups.is_some() {
                    for group in app.groups.as_ref().unwrap().iter() {
                        let button_label =
                            format!("{} {}", crate::defines::AF_GROUP_CODE, &group.cn);

                        // Save the clicked group name.
                        if ui.button(button_label).clicked() {
                            app.group_button_clicked = Some(group.cn.clone());
                            app.directory_button_clicked = None;
                            app.edit_directory_clicked = None;
                            app.edit_group_clicked = None;
                        }
                    }
                }
            });
        });
}
