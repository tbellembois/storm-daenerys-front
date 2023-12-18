use eframe::egui::{self, Context};
use egui::{Color32, Frame, Margin};
use number_prefix::NumberPrefix;

use crate::{
    api,
    defines::{
        AF_FOLDER_CODE, AF_GAUGE_CODE, AF_GROUP_CODE, AF_HALF_LOCK_CODE, AF_LOCK_CODE,
        AF_REFRESH_CODE, DARK_BACKGROUND_COLOR, LIGHT_BACKGROUND_COLOR,
    },
    ui::daenerys::DaenerysApp,
};

pub fn display_left_panel(app: &mut DaenerysApp, ctx: &Context, _frame: &mut eframe::Frame) {
    let background: Color32 = if app.theme.dark_mode {
        DARK_BACKGROUND_COLOR
    } else {
        LIGHT_BACKGROUND_COLOR
    };

    egui::SidePanel::left("group_and_directory_list")
        .frame(Frame {
            inner_margin: Margin {
                left: 20.0,
                right: 20.0,
                top: 10.0,
                bottom: 10.0,
            },
            fill: background,
            ..Default::default()
        })
        .resizable(false)
        .show_separator_line(false)
        .show(ctx, |ui| {
            ui.set_width(300.0);

            let available_height: f32 = ui.available_size().y;
            let available_width: f32 = ui.available_size().x;
            let scroll_height: f32 = (available_height - 300.) / 2.;

            //
            // Disk usage button.
            //
            ui.horizontal_top(|ui| {
                let button = egui::Button::new(format!("{} show disk usage", AF_GAUGE_CODE));

                if ui.add_sized([150., 30.], button).clicked() {
                    app.is_working = true;
                    app.create_group_clicked = false;
                    app.directory_button_clicked = None;
                    app.group_button_clicked = None;
                    app.is_directory_editing = false;
                    app.is_group_editing = false;

                    let du_width = available_width as u32 / 2;
                    app.get_du_promise =
                        Some(api::root::get_du(ctx, app.api_url.clone(), du_width));
                }
            });

            //ui.image(egui::include_image!("../../media/separator.svg"));
            ui.add_space(20.0);

            //
            // Quota.
            //
            if let Some(quota) = &app.quota {
                //ui.label(format!("{:?}", quota));

                let used_space = quota.total_space - quota.available_space;
                let percent_used: f32 = (used_space * 100 / quota.total_space) as f32;
                let float_used: f32 = percent_used / 100.;

                let formated_total = match NumberPrefix::binary(quota.total_space as f32) {
                    NumberPrefix::Standalone(bytes) => {
                        format!("{} bytes", bytes)
                    }
                    NumberPrefix::Prefixed(prefix, n) => {
                        format!("{:.1} {}B", n, prefix)
                    }
                };
                let formated_used = match NumberPrefix::binary(used_space as f32) {
                    NumberPrefix::Standalone(bytes) => {
                        format!("{} bytes", bytes)
                    }
                    NumberPrefix::Prefixed(prefix, n) => {
                        format!("{:.1} {}B", n, prefix)
                    }
                };

                ui.horizontal_top(|ui| {
                    ui.label(formated_used);
                    ui.add(egui::ProgressBar::new(float_used).show_percentage());
                    ui.label(formated_total);
                });
            }

            //ui.image(egui::include_image!("../../media/separator.svg"));
            ui.add_space(20.0);

            //
            // Refresh directory list button.
            //
            ui.horizontal_top(|ui| {
                let button = egui::Button::new(AF_REFRESH_CODE.to_string());

                if ui.add_sized([30., 30.], button).clicked() {
                    app.get_directories_promise = Some(api::directory::get_root_directories(
                        ctx,
                        app.api_url.clone(),
                    ));
                }

                ui.label(egui::RichText::new("my root directories").heading());
            });

            //
            // Directories buttons.
            //
            // ui.vertical_centered_justified(|ui| {
            egui::ScrollArea::vertical()
                .id_source("directory_scroll")
                .max_height(scroll_height)
                .show(ui, |ui| {
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
                                            app.directory_button_clicked =
                                                Some(Box::new(directory.clone()));

                                            app.group_button_clicked = None;
                                            app.is_directory_editing = false;
                                            app.is_group_editing = false;
                                            app.edit_directory_add_user_clicked = false;
                                            app.edit_directory_add_group_clicked = false;
                                            app.create_group_clicked = false;
                                            app.current_error = None;
                                            app.current_info = None;
                                            app.edit_group_delete_confirm = false;
                                            app.du = None;
                                        };
                                    });

                                    ui.end_row()
                                }
                            });
                    }
                });

            //ui.image(egui::include_image!("../../media/separator.svg"));
            ui.add_space(20.0);
            //ui.image(egui::include_image!("../../media/separator.svg"));
            ui.add_space(20.0);

            //
            // Refresh group list button.
            //
            ui.horizontal_top(|ui| {
                let button = egui::Button::new(AF_REFRESH_CODE.to_string());

                if ui.add_sized([30., 30.], button).clicked() {
                    app.get_groups_promise = Some(api::group::get_groups(ctx, app.api_url.clone()));
                }

                ui.label(egui::RichText::new("my storm groups").heading());
            });

            //
            // Groups buttons.
            //
            // ui.vertical_centered_justified(|ui| {
            egui::ScrollArea::vertical()
                .id_source("group_scroll")
                .max_height(scroll_height)
                .show(ui, |ui| {
                    if app.groups.is_some() {
                        egui::Grid::new("group_list").num_columns(2).show(ui, |ui| {
                            for group in app.groups.as_ref().unwrap().iter() {
                                let is_group_auto = group.cn.eq(app.group_prefix.as_ref().unwrap());
                                let is_group_invite = group
                                    .cn
                                    .eq(&format!("{}-invite", app.group_prefix.as_ref().unwrap()));

                                ui.add_sized(
                                    [30., 30.],
                                    egui::Label::new(format!("{}", AF_GROUP_CODE)),
                                );

                                ui.horizontal(|ui| {
                                    let mut button_label = group.cn.to_string();

                                    if is_group_auto {
                                        button_label = format!("{} {}", AF_LOCK_CODE, group.cn)
                                    }
                                    if is_group_invite {
                                        button_label = format!("{} {}", AF_HALF_LOCK_CODE, group.cn)
                                    }

                                    let button = egui::Button::new(button_label);

                                    // Save the clicked group name.
                                    if ui.add_sized([200., 30.], button).clicked() {
                                        app.group_button_clicked = Some(Box::new(group.clone()));

                                        app.directory_button_clicked = None;
                                        app.is_directory_editing = false;
                                        app.is_group_editing = false;
                                        app.edit_directory_add_user_clicked = false;
                                        app.edit_directory_add_group_clicked = false;
                                        app.create_group_clicked = false;
                                        app.current_error = None;
                                        app.current_info = None;
                                        app.edit_group_delete_confirm = false;
                                        app.du = None;
                                    }
                                });

                                ui.end_row()
                            }
                        });
                    }
                });

            //ui.image(egui::include_image!("../../media/separator.svg"));
            ui.add_space(20.0);

            //
            // Create group button.
            //
            let button_label = format!("{} {}", crate::defines::AF_ADD_CODE, "create group");

            let button = egui::Button::new(button_label);

            if ui.add_sized([150., 30.], button).clicked() {
                app.create_group_clicked = true;
                app.directory_button_clicked = None;
                app.group_button_clicked = None;
                app.is_directory_editing = false;
                app.is_group_editing = false;
                app.du = None;

                app.create_group_name.clear();
                app.create_group_description.clear();
            }
        });
}
