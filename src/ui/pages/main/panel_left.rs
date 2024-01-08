use eframe::egui::{self, Context};
use egui::{Color32, Frame, Margin};
use number_prefix::NumberPrefix;

use crate::{
    api,
    defines::{AF_FOLDER_CODE, AF_GROUP_CODE, AF_HALF_LOCK_CODE, AF_LOCK_CODE, AF_REFRESH_CODE},
    ui::daenerys::DaenerysApp,
};

pub fn display_left_panel(app: &mut DaenerysApp, ctx: &Context, _frame: &mut eframe::Frame) {
    egui::SidePanel::left("group_and_directory_list")
        .frame(Frame {
            inner_margin: Margin {
                left: 20.0,
                right: 20.0,
                top: 10.0,
                bottom: 10.0,
            },
            fill: app.background_color,
            ..Default::default()
        })
        .resizable(false)
        .show_separator_line(false)
        .show(ctx, |ui| {
            ui.set_width(300.0);

            let available_height: f32 = ui.available_size().y;
            let scroll_height = if app.show_directory_list ^ app.show_group_list {
                available_height - 100.
            } else {
                (available_height - 100.) / 2.
            };

            //
            // Quota.
            //
            if let Some(quota) = &app.quota {
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

            ui.add_space(20.0);

            if app.show_directory_list {
                //
                // Refresh directory list button.
                //
                ui.horizontal_top(|ui| {
                    ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                        ui.label(
                            egui::RichText::new("my root directories")
                                .size(20.0)
                                .color(Color32::from_rgb(60, 179, 113)),
                        );
                    });

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                        let button = egui::Button::new(format!("{} reload", AF_REFRESH_CODE));
                        if ui.add_sized([30., 30.], button).clicked() {
                            app.get_directories_promise = Some(
                                api::directory::get_root_directories(ctx, app.api_url.clone()),
                            );
                        }
                    });
                });

                //
                // Directories buttons.
                //
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

                ui.add_space(20.0);
            }

            if app.show_group_list {
                //
                // Refresh group list button.
                //
                ui.horizontal_top(|ui| {
                    ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                        ui.label(
                            egui::RichText::new("my storm groups")
                                .size(20.0)
                                .color(Color32::from_rgb(60, 179, 113)),
                        );
                    });

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                        let button = egui::Button::new(format!("{} reload", AF_REFRESH_CODE));
                        if ui.add_sized([30., 30.], button).clicked() {
                            app.get_groups_promise =
                                Some(api::group::get_groups(ctx, app.api_url.clone()));
                        }
                    });
                });

                //
                // Groups buttons.
                //
                egui::ScrollArea::vertical()
                    .id_source("group_scroll")
                    .max_height(scroll_height)
                    .show(ui, |ui| {
                        if app.groups.is_some() {
                            egui::Grid::new("group_list").num_columns(2).show(ui, |ui| {
                                for group in app.groups.as_ref().unwrap().iter() {
                                    let is_group_auto =
                                        group.cn.eq(app.group_prefix.as_ref().unwrap());
                                    let is_group_invite = group.cn.eq(&format!(
                                        "{}-invite",
                                        app.group_prefix.as_ref().unwrap()
                                    ));

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
                                            button_label =
                                                format!("{} {}", AF_HALF_LOCK_CODE, group.cn)
                                        }

                                        let button = egui::Button::new(button_label);

                                        // Save the clicked group name.
                                        if ui.add_sized([200., 30.], button).clicked() {
                                            app.group_button_clicked =
                                                Some(Box::new(group.clone()));

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
            }
        });
}
