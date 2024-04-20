use egui::Ui;
use storm_daenerys_common::types::{
    acl::{AclEntry, Qualifier, SetAcl},
    directory::Directory,
};

use crate::{
    api::{self, acl::save_acl},
    defines::{AF_ADMIN_CODE, AF_GROUP_CODE, AF_USER_CODE},
    ui::daenerys::DaenerysApp,
};

use super::{add_group::render_add_group, add_user::render_add_user};

pub fn render_show_edit_acl(
    app: &mut DaenerysApp,
    ctx: &egui::Context,
    ui: &mut Ui,
    acls: &[AclEntry],
    directory_button_clicked: Box<Directory>,
) {
    egui::Grid::new("acl_list").num_columns(3).show(ui, |ui| {
        ui.add_enabled_ui(app.is_directory_acl_editing, |ui| {
            egui::Grid::new("acl_list_edit")
                .num_columns(4)
                .show(ui, |ui| {
                    let sorted_acls = acls;
                    sorted_acls.to_owned().sort_by(|a, b| {
                        let a_qualifier_cn = a.qualifier_cn.clone().unwrap_or_default();
                        let b_qualifier_cn = b.qualifier_cn.clone().unwrap_or_default();

                        let a_display = app
                            .user_display_cache
                            .get(&a_qualifier_cn)
                            .unwrap_or(&Some("".to_string()))
                            .clone()
                            .unwrap_or_default();
                        let b_display = app
                            .user_display_cache
                            .get(&b_qualifier_cn)
                            .unwrap_or(&Some("".to_string()))
                            .clone()
                            .unwrap_or_default();

                        a_display.to_lowercase().cmp(&b_display.to_lowercase())
                    });

                    for acl in sorted_acls {
                        // FIXME
                        // Keep only necessary acls.
                        match acl.qualifier {
                            Qualifier::User(_) => (),
                            Qualifier::Group(_) => (),
                            _ => continue,
                        }

                        // Skip zero value acls.
                        if acl.perm == 0 {
                            continue;
                        }

                        let is_admin: bool = acl
                            .qualifier_cn
                            .as_ref()
                            .unwrap()
                            .eq(&app.admin.clone().unwrap());

                        let mut read_only: bool;

                        match acl.perm {
                            0 | 1 | 4 | 5 => read_only = true,
                            _ => read_only = false,
                        };

                        match acl.qualifier {
                            storm_daenerys_common::types::acl::Qualifier::User(_) => {
                                // User icon.
                                ui.label(AF_USER_CODE.to_string());
                                // User cn.
                                let member = acl.qualifier_cn.as_ref().unwrap();
                                let (display, color) = match app.user_display_cache.get(member) {
                                    Some(maybe_display_name) => match maybe_display_name {
                                        Some(display_name) => (
                                            format!("{} ({})", display_name, member),
                                            app.state
                                                .active_theme
                                                .fg_primary_text_color_visuals()
                                                .unwrap(),
                                        ),
                                        None => (
                                            format!("<invalid account> ({})", member),
                                            app.state.active_theme.fg_warn_text_color_visuals(),
                                        ),
                                    },
                                    None => {
                                        if !app.get_user_display_promises.contains_key(member) {
                                            app.get_user_display_promises.insert(
                                                member.to_string(),
                                                Some(api::user::get_user_display(
                                                    ctx,
                                                    member.clone(),
                                                    app.api_url.clone(),
                                                )),
                                            );
                                        }

                                        (
                                            member.to_string(),
                                            app.state
                                                .active_theme
                                                .fg_primary_text_color_visuals()
                                                .unwrap(),
                                        )
                                    }
                                };

                                ui.label(egui::RichText::new(display).color(color));

                                if !is_admin {
                                    if ui
                                        .checkbox(
                                            &mut read_only,
                                            egui::RichText::new("read only").italics(),
                                        )
                                        .changed()
                                    {
                                        app.edited_directory_toogle_read_only = Some((
                                            acl.qualifier_cn.as_ref().unwrap().to_string(),
                                            read_only,
                                        ));
                                    };
                                } else {
                                    // Admin icon.
                                    ui.label(AF_ADMIN_CODE.to_string());
                                }
                            }
                            storm_daenerys_common::types::acl::Qualifier::Group(_) => {
                                // Group icon.
                                ui.label(AF_GROUP_CODE.to_string());
                                // Group cn.
                                ui.label(acl.qualifier_cn.as_ref().unwrap());

                                if !is_admin {
                                    if ui
                                        .checkbox(
                                            &mut read_only,
                                            egui::RichText::new("read only").italics(),
                                        )
                                        .changed()
                                    {
                                        app.edited_directory_toogle_read_only = Some((
                                            acl.qualifier_cn.as_ref().unwrap().to_string(),
                                            read_only,
                                        ));
                                    };
                                } else {
                                    // Admin icon.
                                    ui.label(AF_ADMIN_CODE.to_string());
                                }
                            }
                            _ => (),
                        }

                        // Delete acl button.
                        if app.is_directory_acl_editing && !is_admin {
                            let button_label =
                                format!("{} {}", crate::defines::AF_DELETE_CODE, "delete entry");
                            let button = egui::Button::new(button_label);

                            if ui.add_sized([150., 25.], button).clicked() {
                                app.edited_directory_remove_acl =
                                    Some(acl.qualifier_cn.as_ref().unwrap().to_string());
                            }
                        }

                        ui.end_row();
                    }
                });
        });
    });

    // Add user, add group and save buttons.
    if app.is_directory_acl_editing {
        ui.add_space(20.0);

        ui.horizontal_top(|ui| {
            let button_label = format!("{} {}", crate::defines::AF_ADD_CODE, "add user");
            let button = egui::Button::new(button_label);

            if !app.edit_directory_add_user_clicked
                && !app.edit_directory_add_group_clicked
                && ui.add_sized([150., 30.], button).clicked()
            {
                app.edit_directory_add_user_clicked = true;
                app.edit_directory_add_group_clicked = false;
                app.users = None;
            }

            let button_label = format!("{} {}", crate::defines::AF_ADD_CODE, "add group");
            let button = egui::Button::new(button_label);

            if !app.edit_directory_add_user_clicked
                && !app.edit_directory_add_group_clicked
                && ui.add_sized([150., 30.], button).clicked()
            {
                app.edit_directory_add_group_clicked = true;
                app.edit_directory_add_user_clicked = false;
            }
        });

        ui.add_space(20.0);

        // Save button.
        if !app.edit_directory_add_user_clicked && !app.edit_directory_add_group_clicked {
            let button_label = format!("{} {}", crate::defines::AF_SAVE_CODE, "save");
            let button = egui::Button::new(button_label);

            if ui.add_sized([150., 30.], button).clicked() {
                let directory_name = directory_button_clicked.name.clone();

                app.current_info = Some(format!("saving acl for {}", directory_name));

                let set_acl = SetAcl {
                    name: directory_name,
                    acls: directory_button_clicked.acls.clone(),
                };

                app.is_working = true;
                app.save_directory_acl_promise = Some(save_acl(ctx, set_acl, app.api_url.clone()));
            }
        }
    }

    // User add.
    if app.edit_directory_add_user_clicked {
        render_add_user(app, ctx, ui, directory_button_clicked.clone())
    }

    // Add group.
    if app.edit_directory_add_group_clicked {
        render_add_group(app, ui)
    }
}
