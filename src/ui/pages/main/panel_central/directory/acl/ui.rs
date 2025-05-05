use super::{add_group::render_add_group, add_user::render_add_user};
use crate::{
    api::acl::save_acl,
    defines::{
        AF_ADD_CODE, AF_ADMIN_CODE, AF_DELETE_CODE, AF_EYE_CODE, AF_GROUP_CODE, AF_SAVE_CODE,
        AF_USER_CODE,
    },
    ui::daenerys::{Action, DaenerysApp},
};
use egui::{Color32, Ui};
use storm_daenerys_common::types::acl::SetAcl;

pub fn render_show_edit_acl(app: &mut DaenerysApp, ctx: &egui::Context, ui: &mut Ui) {
    ui.add_space(20.0);

    let scroll_height = ui.available_height() - 50.;

    egui::ScrollArea::vertical()
        .id_salt("acl_list_scroll")
        .max_height(scroll_height)
        .show(ui, |ui| {
            egui::Grid::new("acl_list").num_columns(3).show(ui, |ui| {
                ui.add_enabled_ui(app.active_action == Action::DirectoryEditAcl, |ui| {
                    egui::Grid::new("acl_list_edit")
                        .num_columns(4)
                        .show(ui, |ui| {
                            // Sort current directory ACLs.
                            app.current_directory
                                .as_mut()
                                .unwrap()
                                .acls
                                .sort_by(|a, b| a.qualifier_display.cmp(&b.qualifier_display));

                            // Loop throught non mutable clone of ACLs.
                            for acl in app.current_directory.as_ref().unwrap().acls.clone() {
                                let is_admin: bool = acl.is_admin(&app.admin.clone().unwrap());
                                let mut read_only: bool = acl.is_readonly();

                                match acl.qualifier {
                                    storm_daenerys_common::types::acl::Qualifier::User(_) => {
                                        // User icon.
                                        ui.label(AF_USER_CODE.to_string());

                                        // User cn.
                                        let user_cn = acl.qualifier_display.as_ref().unwrap();
                                        let color = if user_cn.starts_with('<') {
                                            Color32::RED
                                        } else {
                                            Color32::WHITE
                                        };

                                        let qualifier_display = if read_only {
                                            format!("{} {}", user_cn, AF_EYE_CODE)
                                        } else {
                                            user_cn.to_string()
                                        };

                                        ui.label(
                                            egui::RichText::new(qualifier_display).color(color),
                                        );

                                        if app.active_action == Action::DirectoryEditAcl
                                            && !is_admin
                                        {
                                            if ui
                                                .checkbox(
                                                    &mut read_only,
                                                    egui::RichText::new("read only").italics(),
                                                )
                                                .changed()
                                            {
                                                // Loop throught mutable ACLs of current directory
                                                // and toggle read only on the one with the cn been checked.
                                                for app_current_directory_acl in app
                                                    .current_directory
                                                    .as_mut()
                                                    .unwrap()
                                                    .acls
                                                    .iter_mut()
                                                {
                                                    if app_current_directory_acl
                                                        .qualifier_cn
                                                        .as_ref()
                                                        .unwrap()
                                                        .eq(acl.qualifier_cn.as_ref().unwrap())
                                                    {
                                                        app_current_directory_acl
                                                            .toggle_read_only();
                                                    }
                                                }
                                            };
                                        } else if is_admin {
                                            // Admin icon.
                                            ui.label(AF_ADMIN_CODE.to_string());
                                        }
                                    }
                                    storm_daenerys_common::types::acl::Qualifier::Group(_) => {
                                        // Group icon.
                                        ui.label(AF_GROUP_CODE.to_string());

                                        // Group cn.
                                        let group_cn = acl.qualifier_cn.as_ref().unwrap();

                                        let qualifier_display = if acl.is_read_only() {
                                            format!("{} {}", group_cn, AF_EYE_CODE)
                                        } else {
                                            group_cn.to_string()
                                        };

                                        ui.label(qualifier_display);

                                        if app.active_action == Action::DirectoryEditAcl
                                            && !is_admin
                                        {
                                            if ui
                                                .checkbox(
                                                    &mut read_only,
                                                    egui::RichText::new("read only").italics(),
                                                )
                                                .changed()
                                            {
                                                // Loop throught mutable ACLs of current directory
                                                // and toggle read only on the one with the cn been checked.
                                                for app_active_directory_acl in app
                                                    .current_directory
                                                    .as_mut()
                                                    .unwrap()
                                                    .acls
                                                    .iter_mut()
                                                {
                                                    if app_active_directory_acl
                                                        .qualifier_cn
                                                        .as_ref()
                                                        .unwrap()
                                                        .eq(acl.qualifier_cn.as_ref().unwrap())
                                                    {
                                                        app_active_directory_acl.toggle_read_only();
                                                    }
                                                }
                                            };
                                        } else if is_admin {
                                            // Admin icon.
                                            ui.label(AF_ADMIN_CODE.to_string());
                                        }
                                    }
                                    _ => (),
                                }

                                // Delete acl button.
                                if app.active_action == Action::DirectoryEditAcl && !is_admin {
                                    let button_label =
                                        format!("{} {}", AF_DELETE_CODE, "delete entry");
                                    let button = egui::Button::new(button_label);

                                    if ui.add_sized([150., 25.], button).clicked() {
                                        app.current_directory.as_mut().unwrap().acls.retain(|a| {
                                            match a.qualifier_cn.clone() {
                                                Some(qualidier_cn) => qualidier_cn
                                                    .ne(acl.qualifier_cn.as_ref().unwrap()),
                                                None => true, // non User(u) or Group(g) acl
                                            }
                                        });
                                    }
                                }

                                ui.end_row();
                            }
                        });
                });
            });
        });

    // Add user, add group and save buttons.
    if app.active_action == Action::DirectoryEditAcl {
        ui.add_space(20.0);

        ui.horizontal_top(|ui| {
            let button_label = format!("{} {}", AF_ADD_CODE, "add user");
            let button = egui::Button::new(button_label);

            if ui.add_sized([150., 30.], button).clicked() {
                app.active_action = Action::DirectoryEditAclAddUser;
                app.users = None;
            }

            let button_label = format!("{} {}", AF_ADD_CODE, "add group");
            let button = egui::Button::new(button_label);

            if ui.add_sized([150., 30.], button).clicked() {
                app.active_action = Action::DirectoryEditAclAddGroup;
            }
        });

        ui.add_space(20.0);

        // Save button.
        if app.active_action == Action::DirectoryEditAcl {
            let button_label = format!("{} {}", AF_SAVE_CODE, "save");
            let button = egui::Button::new(button_label);

            if ui.add_sized([150., 30.], button).clicked() {
                let directory_name = app.current_directory.as_ref().unwrap().name.clone();

                app.current_info = Some(format!("saving acl for {}", directory_name));

                let set_acl = SetAcl {
                    name: directory_name,
                    acls: app.current_directory.as_ref().unwrap().acls.clone(),
                };

                app.is_working = true;
                app.save_directory_acl_promise = Some(save_acl(ctx, set_acl, app.api_url.clone()));
            }
        }
    }

    // User add.
    if app.active_action == Action::DirectoryEditAclAddUser {
        render_add_user(app, ctx, ui)
    }

    // Add group.
    if app.active_action == Action::DirectoryEditAclAddGroup {
        render_add_group(app, ctx, ui)
    }
}
