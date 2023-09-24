use crate::{
    api::{
        self,
        acl::save_acl,
        group::{delete_group, save_group},
    },
    defines::{AF_GROUP_CODE, AF_USER_CODE, DARK_BACKGROUND_COLOR, LIGHT_BACKGROUND_COLOR},
    ui::daenerys::DaenerysApp,
};
use egui::{Color32, Frame, Key, Margin};

use storm_daenerys_common::types::{
    acl::{Qualifier, SetAcl},
    group::Group,
};

pub fn display_central_panel(
    app: &mut DaenerysApp,
    ctx: &egui::Context,
    _frame: &mut eframe::Frame,
) {
    let background: Color32 = if app.theme.dark_mode {
        DARK_BACKGROUND_COLOR
    } else {
        LIGHT_BACKGROUND_COLOR
    };

    egui::CentralPanel::default()
        .frame(Frame {
            inner_margin: Margin {
                left: 50.0,
                right: 10.0,
                top: 50.0,
                bottom: 10.0,
            },
            fill: background,
            ..Default::default()
        })
        .show(ctx, |ui| {
            //
            // Create group form.
            //
            if app.create_group_clicked {
                // Group name.
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.label("storm-".to_string());
                        ui.add_sized(
                            [400., 30.],
                            egui::TextEdit::singleline(&mut app.create_group_name)
                                .hint_text("group name (no space, no accent or special character)"),
                        );
                    });

                    // Group description.
                    ui.add_sized(
                        [400., 30.],
                        egui::TextEdit::singleline(&mut app.create_group_description)
                            .hint_text("description"),
                    );

                    // Create group button.
                    // Validate name, disable create button until valid.
                    let mut enabled: bool = true;
                    if app.create_group_name.clone().len() < 2
                        || !app
                            .group_cn_re
                            .is_match(app.create_group_name.clone().as_str())
                    {
                        enabled = false;
                    }
                    ui.add_enabled_ui(enabled, |ui| {
                        let button_label =
                            format!("{} {}", crate::defines::AF_CREATE_CODE, "create");

                        let button = egui::Button::new(button_label);

                        if ui.add_sized([150., 30.], button).clicked() {
                            app.current_info =
                                Some(format!("creating group {}", app.create_group_name.clone()));

                            let create_group = Group {
                                cn: app.create_group_name.clone(),
                                description: app.create_group_description.clone(),
                                owner: None,
                                member: None,
                            };

                            app.create_group_promise =
                                Some(api::group::create_group(ctx, create_group));

                            app.create_group_name.clear();
                            app.create_group_description.clear();
                        }
                    });
                });
            }

            //
            // Directory details and edition.
            //
            if let Some(directory_button_clicked) = &app.directory_button_clicked {
                // Directory name.
                ui.heading(format!(
                    "{} {}",
                    crate::defines::AF_FOLDER_CODE,
                    directory_button_clicked.name
                ));

                app.separator_image.as_ref().unwrap().show(ui);

                let acls = &directory_button_clicked.acls;

                // ACLs.
                egui::Grid::new("acl_list").num_columns(3).show(ui, |ui| {
                    ui.add_enabled_ui(app.is_directory_editing, |ui| {
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
                                    if app.is_directory_editing {
                                        let button_label = format!(
                                            "{} {}",
                                            crate::defines::AF_DELETE_CODE,
                                            "delete entry"
                                        );
                                        let button = egui::Button::new(button_label);

                                        if ui.add_sized([150., 25.], button).clicked() {
                                            app.edited_directory_remove_acl = Some(
                                                acl.qualifier_cn.as_ref().unwrap().to_string(),
                                            );
                                        }
                                    }

                                    ui.end_row();
                                }
                            });
                    });
                });

                // Display add user and add group buttons.
                if app.is_directory_editing {
                    app.separator_image.as_ref().unwrap().show(ui);

                    ui.horizontal_top(|ui| {
                        // Add user button.
                        let button_label =
                            format!("{} {}", crate::defines::AF_ADD_CODE, "add user");

                        let button = egui::Button::new(button_label);

                        if !app.edit_directory_add_user_clicked
                            && !app.edit_directory_add_group_clicked
                            && ui.add_sized([150., 30.], button).clicked()
                        {
                            app.edit_directory_add_user_clicked = true;
                            app.edit_directory_add_group_clicked = false;
                        }

                        // Add group button.
                        let button_label =
                            format!("{} {}", crate::defines::AF_ADD_CODE, "add group");

                        let button = egui::Button::new(button_label);

                        if !app.edit_directory_add_user_clicked
                            && !app.edit_directory_add_group_clicked
                            && ui.add_sized([150., 30.], button).clicked()
                        {
                            app.edit_directory_add_group_clicked = true;
                            app.edit_directory_add_user_clicked = false;
                        }
                    });
                }

                // Display edit button.
                if !app.is_directory_editing {
                    app.separator_image.as_ref().unwrap().show(ui);

                    let button_label = format!("{} {}", crate::defines::AF_EDIT_CODE, "edit ACLs");
                    let button = egui::Button::new(button_label);

                    if ui.add_sized([150., 30.], button).clicked() {
                        app.is_directory_editing = true;
                        app.is_group_editing = false;
                    }
                }

                //
                // Add user form.
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
                        let button_label =
                            format!("{} {}", crate::defines::AF_SEARCH_CODE, "search");

                        let button = egui::Button::new(button_label);

                        if ui.add_sized([150., 30.], button).clicked() {
                            app.get_users_promise =
                                Some(api::user::get_users(ctx, app.user_search.clone()));
                        }

                        if ctx.input(|i| i.key_pressed(Key::Enter)) {
                            app.get_users_promise =
                                Some(api::user::get_users(ctx, app.user_search.clone()));
                        }
                    });

                    // User list.
                    if app.users.is_some() {
                        app.separator_image.as_ref().unwrap().show(ui);

                        for user in app.users.as_ref().unwrap() {
                            if ui
                                .link(format!("{} [{}]", user.clone().display, user.clone().id))
                                .clicked()
                            {
                                app.edited_directory_add_user = Some(user.id.clone());
                            }
                        }

                        app.separator_image.as_ref().unwrap().show(ui);
                    }

                    // Done button.
                    let button_label = format!("{} {}", crate::defines::AF_CANCEL_CODE, "done");

                    let button = egui::Button::new(button_label);

                    if ui.add_sized([150., 30.], button).clicked() {
                        app.edit_directory_add_user_clicked = false;
                    }
                }

                //
                // Add group form.
                //
                if app.edit_directory_add_group_clicked {
                    // Group list.
                    if app.groups.is_some() {
                        app.separator_image.as_ref().unwrap().show(ui);

                        for group in app.groups.as_ref().unwrap() {
                            if ui.link(group.clone().cn).clicked() {
                                app.edited_directory_add_group = Some(group.cn.clone());
                            }
                        }

                        app.separator_image.as_ref().unwrap().show(ui);
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
                if app.is_directory_editing
                    && !app.edit_directory_add_group_clicked
                    && !app.edit_directory_add_user_clicked
                {
                    app.separator_image.as_ref().unwrap().show(ui);

                    let button_label = format!("{} {}", crate::defines::AF_SAVE_CODE, "save");

                    let button = egui::Button::new(button_label);

                    if ui.add_sized([150., 30.], button).clicked() {
                        app.current_info =
                            Some(format!("saving acl for {}", directory_button_clicked.name));

                        let set_acl = SetAcl {
                            name: directory_button_clicked.name.clone(),
                            acls: directory_button_clicked.acls.clone(),
                        };

                        app.save_directory_acl_promise = Some(save_acl(ctx, set_acl));
                    }
                }
            }

            //
            // Group details and edition.
            //
            if let Some(group_button_clicked) = &app.group_button_clicked {
                // Group name.
                ui.heading(format!(
                    "{} {}",
                    crate::defines::AF_FOLDER_CODE,
                    group_button_clicked.cn.clone()
                ));
                ui.label(egui::RichText::new(group_button_clicked.description.clone()).italics());

                app.separator_image.as_ref().unwrap().show(ui);

                match &group_button_clicked.member {
                    Some(members) => {
                        egui::Grid::new("group_detail")
                            .num_columns(2)
                            .show(ui, |ui| {
                                for member in members {
                                    ui.label(member);

                                    if app.is_group_editing {
                                        // Delete member button.
                                        let button_label = format!(
                                            "{} {}",
                                            crate::defines::AF_DELETE_CODE,
                                            "delete member"
                                        );
                                        let button = egui::Button::new(button_label);

                                        if ui.add_sized([150., 25.], button).clicked() {
                                            app.edited_group_remove_member =
                                                Some(member.to_string());
                                        }
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

                // Edit group button.
                let button_label = format!("{} {}", crate::defines::AF_EDIT_CODE, "edit group");
                let button = egui::Button::new(button_label);

                if !app.is_group_editing && ui.add_sized([150., 30.], button).clicked() {
                    app.edit_group_clicked_backup = Some(Box::new(Group {
                        ..*group_button_clicked.clone()
                    }));
                    app.is_directory_editing = false;
                    app.is_group_editing = true;
                }

                ui.horizontal_top(|ui| {
                    // Add user button.
                    if app.is_group_editing {
                        ui.horizontal_top(|ui| {
                            let button_label =
                                format!("{} {}", crate::defines::AF_ADD_CODE, "add user");

                            let button = egui::Button::new(button_label);

                            if !app.edit_group_add_user_clicked
                                && ui.add_sized([150., 30.], button).clicked()
                            {
                                app.edit_group_add_user_clicked = true;
                            }
                        });
                    }

                    // Delete group button.
                    let button_label =
                        format!("{} {}", crate::defines::AF_DELETE_CODE, "delete group");
                    let button = egui::Button::new(button_label);

                    if app.is_group_editing
                        && !app.edit_group_delete_confirm
                        && !app.edit_group_add_user_clicked
                        && ui.add_sized([150., 30.], button).clicked()
                    {
                        app.edit_group_delete_confirm = true;
                    }

                    if app.edit_group_delete_confirm {
                        let button_label =
                            format!("{} {}", crate::defines::AF_CONFIRM_CODE, "confirm deletion");
                        let button = egui::Button::new(button_label);
                        if ui.add_sized([150., 30.], button).clicked() {
                            app.delete_group_promise = Some(delete_group(
                                ctx,
                                app.group_button_clicked.as_ref().unwrap().cn.clone(),
                            ));

                            app.edit_group_delete_confirm = false;
                        }
                    }
                });

                //
                // Add user form.
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
                        let button_label =
                            format!("{} {}", crate::defines::AF_SEARCH_CODE, "search");

                        let button = egui::Button::new(button_label);

                        if ui.add_sized([150., 30.], button).clicked() {
                            app.get_users_promise =
                                Some(api::user::get_users(ctx, app.user_search.clone()));
                        }

                        if ctx.input(|i| i.key_pressed(Key::Enter)) {
                            app.get_users_promise =
                                Some(api::user::get_users(ctx, app.user_search.clone()));
                        }
                    });

                    // User list.
                    if app.users.is_some() {
                        app.separator_image.as_ref().unwrap().show(ui);

                        for user in app.users.as_ref().unwrap() {
                            if ui
                                .link(format!("{} [{}]", user.clone().display, user.clone().id))
                                .clicked()
                            {
                                app.edited_group_add_user = Some(user.id.clone());
                            }
                        }

                        app.separator_image.as_ref().unwrap().show(ui);
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
                if app.is_group_editing && !app.edit_group_add_user_clicked {
                    app.separator_image.as_ref().unwrap().show(ui);

                    let button_label = format!("{} {}", crate::defines::AF_SAVE_CODE, "save");

                    let button = egui::Button::new(button_label);

                    if ui.add_sized([150., 30.], button).clicked() {
                        app.current_info =
                            Some(format!("saving group {}", group_button_clicked.cn));

                        app.save_group_promises = Some(save_group(
                            ctx,
                            *app.edit_group_clicked_backup.as_ref().unwrap().clone(),
                            *group_button_clicked.clone(),
                        ));
                    }
                }
            };

            //
            // Group edition.
            //
            // if let Some(edit_group_clicked) = &app.edit_group_clicked {
            //     ui.heading(format!(
            //         "{} {}",
            //         crate::defines::AF_FOLDER_CODE,
            //         edit_group_clicked.cn.clone()
            //     ));

            //     app.separator_image.as_ref().unwrap().show(ui);

            //     match &edit_group_clicked.member {
            //         Some(members) => {
            //             egui::Grid::new("group_member_edit")
            //                 .num_columns(2)
            //                 .show(ui, |ui| {
            //                     for member in members {
            //                         ui.label(member);

            //                         // Delete member button.
            //                         let button_label = format!(
            //                             "{} {}",
            //                             crate::defines::AF_DELETE_CODE,
            //                             "delete member"
            //                         );
            //                         let button = egui::Button::new(button_label);

            //                         if ui.add_sized([150., 25.], button).clicked() {
            //                             app.edited_group_remove_member = Some(member.to_string());
            //                         }

            //                         ui.end_row();
            //                     }
            //                 });
            //         }
            //         None => {
            //             ui.label("no members".to_string());
            //         }
            //     }

            //     app.separator_image.as_ref().unwrap().show(ui);

            //     ui.horizontal_top(|ui| {
            //         // Add user button.
            //         let button_label = format!("{} {}", crate::defines::AF_ADD_CODE, "add user");

            //         let button = egui::Button::new(button_label);

            //         if !app.edit_group_add_user_clicked
            //             && ui.add_sized([150., 30.], button).clicked()
            //         {
            //             app.edit_group_add_user_clicked = true;
            //         }
            //     });

            //     //
            //     // Add user.
            //     //
            //     if app.edit_group_add_user_clicked {
            //         // Search user form.
            //         ui.horizontal_top(|ui| {
            //             ui.add_sized(
            //                 [400., 30.],
            //                 egui::TextEdit::singleline(&mut app.user_search)
            //                     .hint_text("enter at least 2 characters and click search"),
            //             );
            //             // Search user button.
            //             let button_label =
            //                 format!("{} {}", crate::defines::AF_SEARCH_CODE, "search");

            //             let button = egui::Button::new(button_label);

            //             if ui.add_sized([150., 30.], button).clicked() {
            //                 app.get_users_promise =
            //                     Some(api::user::get_users(ctx, app.user_search.clone()));
            //             }

            //             if ctx.input(|i| i.key_pressed(Key::Enter)) {
            //                 app.get_users_promise =
            //                     Some(api::user::get_users(ctx, app.user_search.clone()));
            //             }
            //         });

            //         // User list.
            //         if app.users.is_some() {
            //             for user in app.users.as_ref().unwrap() {
            //                 if ui
            //                     .link(format!("{} [{}]", user.clone().display, user.clone().id))
            //                     .clicked()
            //                 {
            //                     app.edited_group_add_user = Some(user.id.clone());
            //                 }
            //             }
            //         }

            //         // Done button.
            //         let button_label = format!("{} {}", crate::defines::AF_CANCEL_CODE, "done");

            //         let button = egui::Button::new(button_label);

            //         if ui.add_sized([150., 30.], button).clicked() {
            //             app.edit_group_add_user_clicked = false;
            //         }
            //     }

            //     //
            //     // Save button.
            //     //
            //     if !app.edit_group_add_user_clicked {
            //         let button_label = format!("{} {}", crate::defines::AF_SAVE_CODE, "save");

            //         let button = egui::Button::new(button_label);

            //         if ui.add_sized([150., 30.], button).clicked() {
            //             app.current_info = Some(format!("saving group {}", edit_group_clicked.cn));

            //             app.save_group_promises = Some(save_group(
            //                 ctx,
            //                 app.edit_group_clicked_backup.as_ref().unwrap().clone(),
            //                 edit_group_clicked.clone(),
            //             ));
            //         }
            //     }
            // }
        });
}
