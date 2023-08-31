use storm_daenerys_common::types::{
    acl::{Qualifier, SetAcl},
    directory::Directory,
};

use crate::{
    api::{self, acl::save_acl},
    defines::{AF_GROUP_CODE, AF_USER_CODE},
    ui::daenerys::DaenerysApp,
};

pub fn display_central_panel(
    app: &mut DaenerysApp,
    ctx: &egui::Context,
    _frame: &mut eframe::Frame,
) {
    egui::CentralPanel::default().show(ctx, |ui| {
        //
        // Directory details
        //
        if let Some(directory_button_clicked) = &app.directory_button_clicked {
            ui.heading(&directory_button_clicked.name);
            ui.separator();

            let acls = &directory_button_clicked.acls;

            egui::Grid::new("acl_list").num_columns(3).show(ui, |ui| {
                for acl in acls {
                    match acl.qualifier {
                        storm_daenerys_common::types::acl::Qualifier::User(_) => {
                            ui.label(AF_USER_CODE.to_string());
                            ui.label(acl.qualifier_cn.as_ref().unwrap());

                            match acl.perm {
                                0 | 1 | 4 | 5 => {
                                    ui.label(egui::RichText::new("(read only)").italics())
                                }
                                _ => ui.label(""),
                            };

                            ui.end_row();
                        }
                        storm_daenerys_common::types::acl::Qualifier::Group(_) => {
                            ui.label(AF_GROUP_CODE.to_string());
                            ui.label(acl.qualifier_cn.as_ref().unwrap());

                            match acl.perm {
                                0 | 1 | 4 | 5 => {
                                    ui.label(egui::RichText::new("(read only)").italics())
                                }
                                _ => ui.label(""),
                            };

                            ui.end_row();
                        }
                        _ => (),
                    }
                }
            });

            ui.separator();

            // Edit directory button.
            let button_label = format!("{} {}", crate::defines::AF_EDIT_CODE, "edit directory");

            if ui.button(button_label).clicked() {
                app.edit_directory_clicked = Some(Box::new(Directory {
                    ..directory_button_clicked.clone()
                }));
                app.directory_button_clicked = None;
                app.edit_group_clicked = None;
            }
        }

        //
        // Directory edition.
        //
        if let Some(edit_directory_clicked) = &app.edit_directory_clicked {
            // Directory title.
            ui.heading(edit_directory_clicked.name.clone());

            ui.separator();

            // Get acls.
            let acls = &edit_directory_clicked.acls;

            egui::Grid::new("acl_list_edit")
                .num_columns(4)
                .show(ui, |ui| {
                    for acl in acls {
                        // FIXME
                        // Keep only necessary acls.
                        match acl.qualifier {
                            Qualifier::User(_) => (),
                            Qualifier::Group(_) => (),
                            _ => continue,
                        }

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
                                ui.label(acl.qualifier_cn.as_ref().unwrap());

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
                            }
                            storm_daenerys_common::types::acl::Qualifier::Group(_) => {
                                // Group icon.
                                ui.label(AF_GROUP_CODE.to_string());
                                // Group cn.
                                ui.label(acl.qualifier_cn.as_ref().unwrap());

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
                            }
                            _ => (),
                        }

                        // Delete acl button.
                        let button_label =
                            format!("{} {}", crate::defines::AF_DELETE_CODE, "delete entry");

                        if ui.button(button_label).clicked() {
                            app.edited_directory_remove_acl =
                                Some(acl.qualifier_cn.as_ref().unwrap().to_string());
                        }

                        ui.end_row();
                    }
                });

            ui.separator();

            ui.horizontal_top(|ui| {
                // Add user button.
                let button_label = format!("{} {}", crate::defines::AF_ADD_CODE, "add user");

                if !app.add_user_clicked && ui.button(button_label).clicked() {
                    app.add_user_clicked = true;
                }

                // Add group button.
                let button_label = format!("{} {}", crate::defines::AF_ADD_CODE, "add group");

                if !app.add_user_clicked && ui.button(button_label).clicked() {
                    app.add_group_clicked = true;
                }
            });

            //
            // Add user.
            //
            if app.add_user_clicked {
                // Search user form.
                ui.horizontal_top(|ui| {
                    ui.add(
                        egui::TextEdit::singleline(&mut app.user_search)
                            .hint_text("enter at least 2 characters and click search"),
                    );
                    // Search user button.
                    let button_label = format!("{} {}", crate::defines::AF_SEARCH_CODE, "search");

                    if ui.button(button_label).clicked() {
                        app.get_users_promise =
                            Some(api::user::get_users(ctx, app.user_search.clone()));
                    }
                });

                // User list.
                if app.users.is_some() {
                    for user in app.users.as_ref().unwrap() {
                        if ui.link(user.clone().display).clicked() {
                            app.edited_directory_add_user = Some(user.id.clone());
                        }
                    }
                }

                // Done button.
                let done_label = format!("{} {}", crate::defines::AF_CANCEL_CODE, "done");

                if ui.button(done_label).clicked() {
                    app.add_user_clicked = false;
                }
            }

            //
            // Add group.
            //
            if app.add_group_clicked {
                // Group list.
                if app.groups.is_some() {
                    for group in app.groups.as_ref().unwrap() {
                        if ui.link(group.clone().cn).clicked() {
                            app.edited_directory_add_group = Some(group.cn.clone());
                        }
                    }
                }

                // Done button.
                let done_label = format!("{} {}", crate::defines::AF_CANCEL_CODE, "done");

                if ui.button(done_label).clicked() {
                    app.add_group_clicked = false;
                }
            }

            //
            // Save button.
            //
            if !app.add_group_clicked && !app.add_user_clicked {
                let button_label = format!("{} {}", crate::defines::AF_SAVE_CODE, "save");

                if ui.button(button_label).clicked() {
                    app.current_info =
                        Some(format!("saving acl for {}", edit_directory_clicked.name));

                    let set_acl = SetAcl {
                        name: edit_directory_clicked.name.clone(),
                        acls: edit_directory_clicked.acls.clone(),
                    };

                    app.save_directory_acl_promise = Some(save_acl(ctx, set_acl));
                }
            }
        }

        //
        // Group details
        //
        if let Some(g) = &app.group_button_clicked {
            ui.heading(g);

            ui.separator();

            let group = app.groups_map.get(g).unwrap(); // this should never panic as the key always exists

            ui.label(egui::RichText::new(group.description.clone()).italics());

            match &group.member {
                Some(members) => {
                    for member in members {
                        ui.label(member);
                    }
                }
                None => {
                    ui.label("no members".to_string());
                }
            }

            ui.separator();

            // Edit group button.
            let button_label = format!("{} {}", crate::defines::AF_EDIT_CODE, "edit group");

            if ui.button(button_label).clicked() {
                app.edit_group_clicked = Some(g.to_string());
                app.edit_directory_clicked = None;
            }
        };

        //
        // Group edition.
        //
        if let Some(group_clicked) = &app.edit_group_clicked {}
    });
}
