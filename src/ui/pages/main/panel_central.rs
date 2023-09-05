use storm_daenerys_common::types::{
    acl::{Qualifier, SetAcl},
    directory::Directory,
    group::{CreateGroup, Group},
};

use crate::{
    api::{self, acl::save_acl, group::save_group},
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
        // Create group.
        //
        if app.create_group_clicked {
            // Group name form.
            ui.horizontal_top(|ui| {
                ui.add_sized(
                    [400., 30.],
                    egui::TextEdit::singleline(&mut app.create_group_name)
                        .hint_text("group name (no space, no accent or special character)"),
                );

                // Create.
                let button_label = format!("{} {}", crate::defines::AF_CREATE_CODE, "create");

                let button = egui::Button::new(button_label);

                if ui.add_sized([150., 30.], button).clicked() {
                    app.current_info =
                        Some(format!("creating group {}", app.create_group_name.clone()));

                    let create_group = CreateGroup {
                        cn: app.create_group_name.clone(),
                        description: "".to_string(),
                        owner: "storm".to_string(), // FIXME
                    };

                    app.create_group_promise = Some(api::group::create_group(ctx, create_group));
                }
            });
        }

        //
        // Directory details.
        //
        if let Some(directory_button_clicked) = &app.display_directory_button_clicked {
            ui.heading(&directory_button_clicked.name);
            ui.separator();
            app.separator_image.as_ref().unwrap().show(ui);

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

            app.separator_image.as_ref().unwrap().show(ui);
            ui.separator();

            // Edit directory button.
            let button_label = format!("{} {}", crate::defines::AF_EDIT_CODE, "edit directory");
            let button = egui::Button::new(button_label);

            if ui.add_sized([150., 30.], button).clicked() {
                app.edit_directory_clicked = Some(Box::new(Directory {
                    ..directory_button_clicked.clone()
                }));
                app.display_directory_button_clicked = None;
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
            app.separator_image.as_ref().unwrap().show(ui);

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
                        let button = egui::Button::new(button_label);

                        if ui.add_sized([150., 25.], button).clicked() {
                            app.edited_directory_remove_acl =
                                Some(acl.qualifier_cn.as_ref().unwrap().to_string());
                        }

                        ui.end_row();
                    }
                });

            app.separator_image.as_ref().unwrap().show(ui);
            ui.separator();

            ui.horizontal_top(|ui| {
                // Add user button.
                let button_label = format!("{} {}", crate::defines::AF_ADD_CODE, "add user");

                let button = egui::Button::new(button_label);

                if !app.edit_directory_add_user_clicked
                    && !app.edit_directory_add_group_clicked
                    && ui.add_sized([150., 30.], button).clicked()
                {
                    app.edit_directory_add_user_clicked = true;
                    app.edit_directory_add_group_clicked = false;
                }

                // Add group button.
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

            //
            // Add user.
            //
            if app.edit_directory_add_user_clicked {
                // Search user form.
                ui.horizontal_top(|ui| {
                    ui.add_sized(
                        [400., 30.],
                        egui::TextEdit::singleline(&mut app.user_search)
                            .hint_text("enter at least 2 characters and click search"),
                    );

                    // Search user button.
                    let button_label = format!("{} {}", crate::defines::AF_SEARCH_CODE, "search");

                    let button = egui::Button::new(button_label);

                    if ui.add_sized([150., 30.], button).clicked() {
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
                let button_label = format!("{} {}", crate::defines::AF_CANCEL_CODE, "done");

                let button = egui::Button::new(button_label);

                if ui.add_sized([150., 30.], button).clicked() {
                    app.edit_directory_add_user_clicked = false;
                }
            }

            //
            // Add group.
            //
            if app.edit_directory_add_group_clicked {
                // Group list.
                if app.groups.is_some() {
                    for group in app.groups.as_ref().unwrap() {
                        if ui.link(group.clone().cn).clicked() {
                            app.edited_directory_add_group = Some(group.cn.clone());
                        }
                    }
                }

                // Done button.
                let button_label = format!("{} {}", crate::defines::AF_CANCEL_CODE, "done");

                let button = egui::Button::new(button_label);

                if ui.add_sized([150., 30.], button).clicked() {
                    app.edit_directory_add_group_clicked = false;
                }
            }

            //
            // Save button.
            //
            if !app.edit_directory_add_group_clicked && !app.edit_directory_add_user_clicked {
                let button_label = format!("{} {}", crate::defines::AF_SAVE_CODE, "save");

                let button = egui::Button::new(button_label);

                if ui.add_sized([150., 30.], button).clicked() {
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
        if let Some(display_group_button_clicked) = &app.display_group_button_clicked {
            ui.heading(display_group_button_clicked.cn.clone());

            ui.label(
                egui::RichText::new(display_group_button_clicked.description.clone()).italics(),
            );

            ui.separator();
            app.separator_image.as_ref().unwrap().show(ui);

            match &display_group_button_clicked.member {
                Some(members) => {
                    egui::Grid::new("group_detail")
                        .num_columns(1)
                        .show(ui, |ui| {
                            for member in members {
                                ui.label(member);

                                ui.end_row();
                            }
                        });
                }
                None => {
                    ui.label("no members".to_string());
                }
            }

            app.separator_image.as_ref().unwrap().show(ui);
            ui.separator();

            // Edit group button.
            let button_label = format!("{} {}", crate::defines::AF_EDIT_CODE, "edit group");
            let button = egui::Button::new(button_label);

            if ui.add_sized([150., 30.], button).clicked() {
                app.edit_group_clicked = Some(Group {
                    ..display_group_button_clicked.clone()
                });
                app.edit_group_clicked_backup = Some(Group {
                    ..display_group_button_clicked.clone()
                });
                app.edit_directory_clicked = None;
                app.display_group_button_clicked = None;
            }
        };

        //
        // Group edition.
        //
        if let Some(edit_group_clicked) = &app.edit_group_clicked {
            ui.heading(edit_group_clicked.cn.clone());
            ui.label(egui::RichText::new(edit_group_clicked.description.clone()).italics());

            ui.separator();
            app.separator_image.as_ref().unwrap().show(ui);

            match &edit_group_clicked.member {
                Some(members) => {
                    egui::Grid::new("group_member_edit")
                        .num_columns(2)
                        .show(ui, |ui| {
                            for member in members {
                                ui.label(member);

                                // Delete member button.
                                let button_label = format!(
                                    "{} {}",
                                    crate::defines::AF_DELETE_CODE,
                                    "delete member"
                                );
                                let button = egui::Button::new(button_label);

                                if ui.add_sized([150., 25.], button).clicked() {
                                    app.edited_group_remove_member = Some(member.to_string());
                                }

                                ui.end_row();
                            }
                        });
                }
                None => {
                    ui.label("no members".to_string());
                }
            }

            app.separator_image.as_ref().unwrap().show(ui);
            ui.separator();

            ui.horizontal_top(|ui| {
                // Add user button.
                let button_label = format!("{} {}", crate::defines::AF_ADD_CODE, "add user");

                let button = egui::Button::new(button_label);

                if !app.edit_group_add_user_clicked && ui.add_sized([150., 30.], button).clicked() {
                    app.edit_group_add_user_clicked = true;
                }
            });

            //
            // Add user.
            //
            if app.edit_group_add_user_clicked {
                // Search user form.
                ui.horizontal_top(|ui| {
                    ui.add_sized(
                        [400., 30.],
                        egui::TextEdit::singleline(&mut app.user_search)
                            .hint_text("enter at least 2 characters and click search"),
                    );
                    // Search user button.
                    let button_label = format!("{} {}", crate::defines::AF_SEARCH_CODE, "search");

                    let button = egui::Button::new(button_label);

                    if ui.add_sized([150., 30.], button).clicked() {
                        app.get_users_promise =
                            Some(api::user::get_users(ctx, app.user_search.clone()));
                    }
                });

                // User list.
                if app.users.is_some() {
                    for user in app.users.as_ref().unwrap() {
                        if ui.link(user.clone().display).clicked() {
                            app.edited_group_add_user = Some(user.id.clone());
                        }
                    }
                }

                // Done button.
                let button_label = format!("{} {}", crate::defines::AF_CANCEL_CODE, "done");

                let button = egui::Button::new(button_label);

                if ui.add_sized([150., 30.], button).clicked() {
                    app.edit_group_add_user_clicked = false;
                }
            }

            //
            // Save button.
            //
            if !app.edit_group_add_user_clicked {
                let button_label = format!("{} {}", crate::defines::AF_SAVE_CODE, "save");

                let button = egui::Button::new(button_label);

                if ui.add_sized([150., 30.], button).clicked() {
                    app.current_info = Some(format!("saving group {}", edit_group_clicked.cn));

                    app.save_group_promises = Some(save_group(
                        ctx,
                        app.edit_group_clicked_backup.as_ref().unwrap().clone(),
                        edit_group_clicked.clone(),
                    ));
                }
            }
        }
    });
}
