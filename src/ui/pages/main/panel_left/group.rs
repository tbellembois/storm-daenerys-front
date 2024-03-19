use egui::{Color32, Ui};

use crate::{
    api,
    defines::{AF_GROUP_CODE, AF_HALF_LOCK_CODE, AF_LOCK_CODE, AF_REFRESH_CODE},
    ui::daenerys::DaenerysApp,
};

pub fn render_group_list(
    app: &mut DaenerysApp,
    ctx: &egui::Context,
    ui: &mut Ui,
    scroll_height: f32,
) {
    // Refresh button.
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
                app.get_groups_promise = Some(api::group::get_groups(ctx, app.api_url.clone()));
            }
        });
    });

    // Group list.
    egui::ScrollArea::vertical()
        .id_source("group_scroll")
        .max_height(scroll_height)
        .show(ui, |ui| {
            if app.groups.is_some() {
                egui::Grid::new("group_list").num_columns(2).show(ui, |ui| {
                    for group in app.groups.as_ref().unwrap().iter() {
                        let mut is_group_auto: bool = false;
                        let mut is_group_invite: bool = false;

                        match &app.root_groups {
                            Some(root_groups) => {
                                for root_group in root_groups {
                                    if group.cn.eq(&format!(
                                        "{}-{}",
                                        app.group_prefix.as_ref().unwrap(),
                                        root_group,
                                    )) {
                                        is_group_auto = true;
                                        break;
                                    }

                                    if group.cn.eq(&format!(
                                        "{}-{}-invite",
                                        app.group_prefix.as_ref().unwrap(),
                                        root_group,
                                    )) {
                                        is_group_invite = true;
                                        break;
                                    }
                                }
                            }
                            None => {
                                is_group_auto = group.cn.eq(app.group_prefix.as_ref().unwrap());
                                is_group_invite = group
                                    .cn
                                    .eq(&format!("{}-invite", app.group_prefix.as_ref().unwrap()));
                            }
                        }

                        ui.add_sized([30., 30.], egui::Label::new(format!("{}", AF_GROUP_CODE)));

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
                                app.is_directory_acl_editing = false;
                                app.is_group_editing = false;
                                app.edit_directory_add_user_clicked = false;
                                app.edit_directory_add_group_clicked = false;
                                app.create_group_clicked = false;
                                app.create_directory_clicked = false;
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
        app.create_directory_clicked = false;
        app.directory_button_clicked = None;
        app.group_button_clicked = None;
        app.is_directory_acl_editing = false;
        app.is_group_editing = false;
        app.du = None;

        app.create_group_name.clear();
        app.create_group_description.clear();
    }
}
