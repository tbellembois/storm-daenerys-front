use std::collections::HashMap;

use storm_daenerys_common::types::{acl::Qualifier, directory::Directory};

use crate::{
    defines::{AF_GROUP_CODE, AF_USER_CODE},
    error::apperror::AppError,
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
                                4 | 5 | 7 => ui.label(egui::RichText::new("(read only)").italics()),
                                _ => ui.label(""),
                            };

                            ui.end_row();
                        }
                        storm_daenerys_common::types::acl::Qualifier::Group(_) => {
                            ui.label(AF_GROUP_CODE.to_string());
                            ui.label(acl.qualifier_cn.as_ref().unwrap());

                            match acl.perm {
                                4 | 5 | 7 => ui.label(egui::RichText::new("(read only)").italics()),
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
                            4 | 5 | 7 => read_only = true,
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
        }

        //
        // Group edition.
        //
        if let Some(group_clicked) = &app.edit_group_clicked {}
    });
}
