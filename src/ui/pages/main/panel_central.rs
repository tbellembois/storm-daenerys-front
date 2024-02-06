use crate::{
    api::{
        self,
        acl::save_acl,
        group::{delete_group, save_group},
    },
    defines::{
        AF_ADMIN_CODE, AF_GROUP_CODE, AF_USER_CODE, AF_LOCK_CODE, AF_HALF_LOCK_CODE,
    },
    ui::daenerys::DaenerysApp,
};
use eframe::egui::{self, Context};
use egui::{Frame, Key, Margin};

use storm_daenerys_common::types::{
    acl::{Qualifier, SetAcl},
    group::Group, directory::CreateDirectory,
};

pub fn display_central_panel(app: &mut DaenerysApp, ctx: &Context, _frame: &mut eframe::Frame) {
    egui::CentralPanel::default()
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
        .show(ctx, |ui| {
            app.central_panel_available_size = ui.available_size();

            //
            // Home.
            //
            if app.application_just_loaded {
                ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                    ui.add_sized(
                        [40., 40.],
                        egui::Image::new(egui::include_image!(
                            "../../media/circle-question-regular.svg"
                        )),
                    );
                });

                ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
                    ui.label(
                        egui::RichText::new("FAQ").heading(),
                    );

                    ui.add_space(20.0);
                    
                    ui.label(
                        egui::RichText::new("Why are some directories disabled on the left panel?").underline(),
                    );

                    ui.add_space(20.0);

                    ui.label("The root directory names can only contain letters (lower and upper case), digits and the characters '_' and '-'.");
                    ui.label("This rule is strictly enforced. You won't be able to manage directories not respecting this naming convention.");

                    ui.add_space(20.0);

                    ui.label(
                        egui::RichText::new("Can I set ACLs on subdirectories?")
                            .underline(),
                    );

                    ui.add_space(20.0);

                    ui.label("No, for technical reasons it is not possible.");

                    ui.add_space(20.0);
                    
                    ui.label(
                        egui::RichText::new("I have given permission to a person on a directory but he/she can't access it").underline(),
                    );

                    ui.add_space(20.0);

                    ui.label(format!("Check that this person is member of one of the group with a {} or a {}.", AF_LOCK_CODE, AF_HALF_LOCK_CODE));
                });

                ui.add_space(20.0);

                ui.separator();
                
                ui.add_space(20.0);

                ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                    ui.add_sized(
                        [40., 40.],
                        egui::Image::new(egui::include_image!(
                            "../../media/rust.svg"
                        )),
                    );
                });

                ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
                    ui.label(
                        egui::RichText::new("Credits").heading(),
                    );

                    ui.add_space(20.0);

                    ui.hyperlink("https://www.rust-lang.org/");
                    ui.hyperlink("https://github.com/emilk/egui");
                    ui.hyperlink("https://github.com/tokio-rs/axum"); 

                    ui.add_space(20.0);

                    ui.label(
                        egui::RichText::new("Copyright").underline(),
                    );
                    ui.label("Université Clermont Auvergne");
                });
            }

            //
            // Spinner.
            //
            if app.is_working {
                ui.add_sized([0., 40.], egui::widgets::Spinner::new());
            } else {
                ui.add_sized([0., 40.], egui::Label::new(""));
            }

            //
            // Disk usage.
            //
            if app.du.is_some() {
                app.application_just_loaded = false;

                let available_height: f32 = ui.available_size().y;
                let scroll_height: f32 = available_height - 50.;

                egui::ScrollArea::vertical()
                    .id_source("du_scroll")
                    .max_height(scroll_height)
                    .show(ui, |ui| {
                        ui.label(
                            egui::RichText::new(app.du.as_ref().unwrap())
                                .text_style(egui::TextStyle::Monospace),
                        );
                    });
            }

            //
            // Create directory form.
            //
            if app.create_directory_clicked {
                app.application_just_loaded = false;

                // Directory name.
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        
                        if let Some(admin_restriction) = &app.current_admin_restriction {
                            ui.label(format!("{}@_", admin_restriction));
                        }
                        ui.add_sized(
                            [400., 30.],
                            egui::TextEdit::singleline(&mut app.create_directory_name)
                                .hint_text("directory name (no space, no accent or special character except - and _)"),
                        );
                    });

                    // Create directory button.
                    // Validate name, disable create button until valid.
                    let mut enabled: bool = true;
                    if app.create_directory_name.clone().len() < 2
                        || !app
                            .directory_name_re
                            .is_match(app.create_directory_name.clone().as_str())
                    {
                        enabled = false;
                    }
                    ui.add_enabled_ui(enabled, |ui| {
                        let button_label =
                            format!("{} {}", crate::defines::AF_CREATE_CODE, "create");

                        let button = egui::Button::new(button_label);

                        if ui.add_sized([150., 30.], button).clicked() {
                            app.current_info =
                                Some(format!("creating directory {}", app.create_directory_name.clone()));

                            let create_directory = CreateDirectory {
                                name: app.create_directory_name.clone(),
                            };

                            app.is_working = true;
                            app.create_directory_promise = Some(api::directory::create_directory(
                                ctx,
                                create_directory,
                                app.api_url.clone(),
                            ));

                            app.create_directory_name.clear();
                        }
                    });
                });
            }

            //
            // Create group form.
            //
            if app.create_group_clicked {
                app.application_just_loaded = false;

                // Group name.
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.label(format!("{}-", app.group_prefix.as_ref().unwrap()));
                        ui.add_sized(
                            [400., 30.],
                            egui::TextEdit::singleline(&mut app.create_group_name)
                                .hint_text("group name (no space, no accent or special character except _)"),
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

                            app.is_working = true;
                            app.create_group_promise = Some(api::group::create_group(
                                ctx,
                                create_group,
                                app.api_url.clone(),
                            ));

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
                app.application_just_loaded = false;

                // Directory name.
                ui.heading(format!(
                    "{} {}",
                    crate::defines::AF_FOLDER_CODE,
                    directory_button_clicked.name
                ));

                
                ui.add_space(20.0);

                let acls = &directory_button_clicked.acls;

                // ACLs.
                egui::Grid::new("acl_list").num_columns(3).show(ui, |ui| {
                    ui.add_enabled_ui(app.is_directory_editing, |ui| {
                        egui::Grid::new("acl_list_edit")
                            .num_columns(4)
                            .show(ui, |ui| {

                                let mut sorted_acls = acls.clone();
                                sorted_acls.sort_by(|a,b| {

                                    let a_qualifier_cn = a.qualifier_cn.clone().unwrap_or_default();
                                    let b_qualifier_cn = b.qualifier_cn.clone().unwrap_or_default();

                                    let a_display = app.user_display_cache.get(&a_qualifier_cn).unwrap_or(&Some("".to_string())).clone().unwrap_or_default();
                                    let b_display = app.user_display_cache.get(&b_qualifier_cn).unwrap_or(&Some("".to_string())).clone().unwrap_or_default();

                                    a_display.to_lowercase().cmp(&b_display.to_lowercase())
                                }
                                    );

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
                                                    Some(display_name) => (format!("{} ({})", display_name, member), egui::Color32::from_rgb(0, 0, 0)),
                                                    None => (format!("<invalid account> ({})", member), egui::Color32::from_rgb(255, 0, 0)),
                                                },
                                                None => {
                                                    if !app.get_user_display_promises.contains_key(member) {
                                                    app.get_user_display_promises.insert(member.to_string(),  Some(api::user::get_user_display(
                                                        ctx,
                                                        member.clone(),
                                                        app.api_url.clone(),
                                                    )));
                                                    }

                                                    (member.to_string(), egui::Color32::from_rgb(255, 165, 0))
                                                },
                                            };

                                            ui.label(egui::RichText::new(display).color(color));
                                            // ui.label(acl.qualifier_cn.as_ref().unwrap());

                                            if !is_admin {
                                                if ui
                                                    .checkbox(
                                                        &mut read_only,
                                                        egui::RichText::new("read only").italics(),
                                                    )
                                                    .changed()
                                                {
                                                    app.edited_directory_toogle_read_only = Some((
                                                        acl.qualifier_cn
                                                            .as_ref()
                                                            .unwrap()
                                                            .to_string(),
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
                                                        acl.qualifier_cn
                                                            .as_ref()
                                                            .unwrap()
                                                            .to_string(),
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
                                    if app.is_directory_editing && !is_admin {
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
                    
                    ui.add_space(20.0);

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
                            app.users = None;
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
                    
                    ui.add_space(20.0);

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
                            app.is_working = true;
                            app.get_users_promise = Some(api::user::get_users(
                                ctx,
                                app.user_search.clone(),
                                app.api_url.clone(),
                            ));
                        }

                        if ctx.input(|i| i.key_pressed(Key::Enter)) {
                            app.is_working = true;
                            app.get_users_promise = Some(api::user::get_users(
                                ctx,
                                app.user_search.clone(),
                                app.api_url.clone(),
                            ));
                        }
                    });

                    // User list.
                    let scroll_height = ui.available_height() - 50.;

                    if app.users.is_some() {
                        egui::ScrollArea::vertical()
                            .id_source("directory_search_user_scroll")
                            .max_height(scroll_height)
                            .show(ui, |ui| {
                                ui.add_space(20.0);

                                for user in app.users.as_ref().unwrap() {
                                    if ui
                                        .link(format!(
                                            "{} [{}]",
                                            user.clone().display,
                                            user.clone().id
                                        ))
                                        .clicked()
                                    {
                                        app.edited_directory_add_user = Some(user.id.clone());
                                    }
                                }
                            });
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
                        
                        ui.add_space(20.0);

                        for group in app.groups.as_ref().unwrap() {
                            if ui.link(group.clone().cn).clicked() {
                                app.edited_directory_add_group = Some(group.cn.clone());
                            }
                        }

                        
                        ui.add_space(20.0);
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
                    
                    ui.add_space(20.0);

                    let button_label = format!("{} {}", crate::defines::AF_SAVE_CODE, "save");

                    let button = egui::Button::new(button_label);

                    if ui.add_sized([150., 30.], button).clicked() {
                        
                        let directory_name = directory_button_clicked.name.clone();

                        // if let Some(admin_restriction) = &app.current_admin_restriction {
                        //     directory_name = format!("{}@_{}", admin_restriction, directory_name);
                        // }
                        
                        app.current_info =
                            Some(format!("saving acl for {}", directory_name));

                        let set_acl = SetAcl {
                            name: directory_name,
                            acls: directory_button_clicked.acls.clone(),
                        };

                        app.is_working = true;
                        app.save_directory_acl_promise =
                            Some(save_acl(ctx, set_acl, app.api_url.clone()));
                    }
                }
            }

            //
            // Group details and edition.
            //
            if let Some(group_button_clicked) = &app.group_button_clicked {
                app.application_just_loaded = false;

                let mut is_group_auto: bool = false;
                let mut is_group_invite: bool = false;

                match &app.root_groups {
                    Some(root_groups) => {
                        for root_group in root_groups {
                            if group_button_clicked.cn.eq(&format!(
                                "{}-{}",
                                app.group_prefix.as_ref().unwrap(),
                                root_group,
                            )) {
                                is_group_auto = true;
                                break;
                            }

                            if group_button_clicked.cn.eq(&format!(
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
                        is_group_auto =
                        group_button_clicked.cn.eq(app.group_prefix.as_ref().unwrap());
                        is_group_invite = group_button_clicked.cn.eq(&format!(
                            "{}-invite",
                            app.group_prefix.as_ref().unwrap()
                        ));
                    }
                }

                // Group name.
                ui.heading(format!(
                    "{} {}",
                    crate::defines::AF_GROUP_CODE,
                    group_button_clicked.cn.clone()
                ));
                ui.label(egui::RichText::new(group_button_clicked.description.clone()).italics());

                
                ui.add_space(20.0);

                match &group_button_clicked.member {
                    Some(members) => {
                        let scroll_height = ui.available_height() - 150.;

                        egui::ScrollArea::vertical()
                            .id_source("group_detail_scroll")
                            .max_height(scroll_height)
                            .show(ui, |ui| {
                                egui::Grid::new("group_detail")
                                    .num_columns(2)
                                    .show(ui, |ui| {

                                        let mut sorted_members = members.clone();
                                        sorted_members.sort_by(|a,b| {

                                            let a_display = app.user_display_cache.get(a).unwrap_or(&Some(a.clone())).clone().unwrap_or_default();
                                            let b_display = app.user_display_cache.get(b).unwrap_or(&Some(b.clone())).clone().unwrap_or_default();
    
                                            a_display.to_lowercase().cmp(&b_display.to_lowercase())
                                        }
                                            );

                                        for member in sorted_members {
                                    
                                            let (display, color) = match app.user_display_cache.get(&member) {
                                                Some(maybe_display_name) => match maybe_display_name {
                                                    Some(display_name) => (format!("{} ({})", display_name, member), egui::Color32::from_rgb(0, 0, 0)),
                                                    None => (format!("<invalid account> ({})", member), egui::Color32::from_rgb(255, 0, 0)),
                                                },
                                                None => {
                                                    if !app.get_user_display_promises.contains_key(&member) {
                                                    app.get_user_display_promises.insert(member.to_string(),  Some(api::user::get_user_display(
                                                        ctx,
                                                        member.clone(),
                                                        app.api_url.clone(),
                                                    )));
                                                    }

                                                    (member.to_string(), egui::Color32::from_rgb(255, 165, 0))
                                                },
                                            };

                                            ui.label(egui::RichText::new(display).color(color));
                                            // ui.label(display);

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
                            });
                    }
                    None => {
                        ui.label("no members".to_string());
                    }
                }

                
                ui.add_space(20.0);

                // Edit group button.
                let button_label = format!("{} {}", crate::defines::AF_EDIT_CODE, "edit group");
                let button = egui::Button::new(button_label);

                if !is_group_auto
                    && !app.is_group_editing
                    && ui.add_sized([150., 30.], button).clicked()
                {
                    app.edit_group_clicked_backup = Some(Box::new(Group {
                        ..*group_button_clicked.clone()
                    }));
                    app.is_directory_editing = false;
                    app.is_group_editing = true;
                }

                ui.horizontal_top(|ui| {
                    // Add user button.
                    if !app.is_working && app.is_group_editing {
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

                    if !is_group_invite
                        && !app.is_working
                        && app.is_group_editing
                        && !app.edit_group_delete_confirm
                        && !app.edit_group_add_user_clicked
                        && ui.add_sized([150., 30.], button).clicked()
                    {
                        app.edit_group_delete_confirm = true;
                    }

                    if !app.is_working && app.edit_group_delete_confirm {
                        let button_label =
                            format!("{} {}", crate::defines::AF_CONFIRM_CODE, "confirm deletion");
                        let button = egui::Button::new(button_label);
                        if ui.add_sized([150., 30.], button).clicked() {
                            app.is_working = true;
                            app.delete_group_promise = Some(delete_group(
                                ctx,
                                app.group_button_clicked.as_ref().unwrap().cn.clone(),
                                app.api_url.clone(),
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
                            app.is_working = true;
                            app.get_users_promise = Some(api::user::get_users(
                                ctx,
                                app.user_search.clone(),
                                app.api_url.clone(),
                            ));
                        }

                        if ctx.input(|i| i.key_pressed(Key::Enter)) {
                            app.is_working = true;
                            app.get_users_promise = Some(api::user::get_users(
                                ctx,
                                app.user_search.clone(),
                                app.api_url.clone(),
                            ));
                        }
                    });

                    // User list.
                    let scroll_height = ui.available_height() - 50.;

                    if app.users.is_some() {
                        egui::ScrollArea::vertical()
                            .id_source("group_search_user_scroll")
                            .max_height(scroll_height)
                            .show(ui, |ui| {
                                ui.add_space(20.0);

                                for user in app.users.as_ref().unwrap() {
                                    if ui
                                        .link(format!(
                                            "{} [{}]",
                                            user.clone().display,
                                            user.clone().id
                                        ))
                                        .clicked()
                                    {
                                        app.edited_group_add_user = Some(user.id.clone());
                                    }
                                }
                            });
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
                    
                    ui.add_space(20.0);

                    let button_label = format!("{} {}", crate::defines::AF_SAVE_CODE, "save");

                    let button = egui::Button::new(button_label);

                    if ui.add_sized([150., 30.], button).clicked() {
                        app.current_info =
                            Some(format!("saving group {}", group_button_clicked.cn));

                        app.is_working = true;
                        app.save_group_promises = Some(save_group(
                            ctx,
                            *app.edit_group_clicked_backup.as_ref().unwrap().clone(),
                            *group_button_clicked.clone(),
                            app.api_url.clone(),
                        ));
                    }
                }
            };
        });
}
