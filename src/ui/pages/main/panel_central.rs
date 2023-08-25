use std::collections::HashMap;

use storm_daenerys_common::types::acl::Qualifier;

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
        // If a directory is clicked then display its acls.
        if let Some(d) = &app.directory_button_clicked {
            ui.heading(&d.name);

            ui.separator();

            let acls = &app.directory_button_clicked.as_ref().unwrap().acls;

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
                app.edit_directory_clicked = Some(d.clone());
                app.edit_group_clicked = None;

                // Initialize the edited_directory_permissions hashmap.
                let mut edited_directory_acl_widget: HashMap<String, bool> = HashMap::new();
                let acls = &app.directory_button_clicked.as_ref().unwrap().acls;

                for acl in acls {
                    tracing::debug!("acl: {:?}", acl);

                    // FIXME
                    // Keep only necessary acls.
                    match acl.qualifier {
                        Qualifier::User(_) => (),
                        Qualifier::Group(_) => (),
                        _ => continue,
                    }

                    let mut is_read_only = false;
                    match acl.perm {
                        4 | 5 | 7 => is_read_only = true,
                        _ => (),
                    };

                    edited_directory_acl_widget
                        .insert(acl.qualifier_cn.clone().unwrap(), is_read_only);
                }

                app.edited_directory_acl_widget = Some(edited_directory_acl_widget);
            }
        }

        // If a group is clicked then display its members.
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

        // Directory edition.
        if let Some(directory_clicked) = &app.edit_directory_clicked {
            // Directory title.
            ui.heading(directory_clicked.name.clone());

            ui.separator();

            // Get acls.
            let acls = &app.edit_directory_clicked.as_ref().unwrap().acls;

            egui::Grid::new("acl_list_edit")
                .num_columns(3)
                .show(ui, |ui| {
                    for acl in acls {
                        // FIXME
                        // Keep only necessary acls.
                        match acl.qualifier {
                            Qualifier::User(_) => (),
                            Qualifier::Group(_) => (),
                            _ => continue,
                        }

                        // Get the read_only bool from the edited_directory_acl_widget hashmap.
                        let maybe_edited_directory_acl_widget =
                            app.edited_directory_acl_widget.as_mut();

                        let edited_directory_acl_widget = match maybe_edited_directory_acl_widget {
                            Some(edited_directory_acl_widget) => edited_directory_acl_widget,
                            None => {
                                app.current_error = Some(AppError::InternalError(
                                    "unexpected None value".to_string(),
                                ));
                                return;
                            }
                        };

                        let maybe_read_only =
                            edited_directory_acl_widget.get_mut(acl.qualifier_cn.as_ref().unwrap());

                        let read_only = match maybe_read_only {
                            Some(read_only) => read_only,
                            None => {
                                app.current_error = Some(AppError::InternalError(
                                    "unexpected None value".to_string(),
                                ));
                                return;
                            }
                        };

                        match acl.qualifier {
                            storm_daenerys_common::types::acl::Qualifier::User(_) => {
                                // User icon.
                                ui.label(AF_USER_CODE.to_string());
                                // User cn.
                                ui.label(acl.qualifier_cn.as_ref().unwrap());

                                // FIXME: explain me
                                // let read_only = app
                                //     .edited_directory_acl_widget
                                //     .as_mut()
                                //     .unwrap()
                                //     .get_mut(acl.qualifier_cn.as_ref().unwrap())
                                //     .unwrap();

                                ui.checkbox(read_only, "read only".to_string());

                                ui.end_row();
                            }
                            storm_daenerys_common::types::acl::Qualifier::Group(_) => {
                                // Group icon.
                                ui.label(AF_GROUP_CODE.to_string());
                                // Group cn.
                                ui.label(acl.qualifier_cn.as_ref().unwrap());

                                ui.checkbox(read_only, "read only".to_string());

                                ui.end_row();
                            }
                            _ => (),
                        }
                    }
                });
        }

        // Group edition.
        if let Some(group_clicked) = &app.edit_group_clicked {}
    });
}
