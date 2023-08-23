
use std::collections::HashMap;

use egui::{RichText, Color32};
use storm_daenerys_common::types::acl::Qualifier;
use tracing::debug;
use tracing_subscriber::fmt::format;

use crate::{ui::daenerys::DaenerysApp, defines::{AF_USER_CODE, AF_GROUP_CODE}};
use crate::api;

pub fn update(app: &mut DaenerysApp, ctx: &egui::Context, _frame: &mut eframe::Frame) {

    egui::TopBottomPanel::top("error_info_panel").show(ctx, |ui| {

        // Current error label.
        if let Some(current_error) = &app.current_error { 
            ui.label(RichText::new(current_error.to_string()).color(Color32::RED));
        }

        // Current info label.
        if let Some(current_info) = &app.current_info { 
            ui.label(RichText::new(current_info.to_string()));
        }

    });

    egui::SidePanel::left("group_and_directory_list").resizable(false)
    .show(ctx, |ui| {

        ui.set_width(200.0);

            ui.horizontal_top(|ui| {

                // Refresh directory list button.
                if ui.button(crate::defines::AF_REFRESH_CODE.to_string()).clicked() {
                    app.get_directories_promise = Some(api::directory::get_root_directories(ctx));
                }

            });

            ui.separator();

            ui.vertical_centered_justified(|ui| {

                // Directories buttons.
                if app.directories.is_some() {                         

                    for directory in app.directories.as_ref().unwrap().iter() {
                    
                        let button_label = format!("{} {}", crate::defines::AF_FOLDER_CODE, &directory.name);

                        // Save the clicked directory name.
                        if ui.button(button_label).clicked() {
                            app.directory_button_clicked = Some(directory.name.clone());
                            app.group_button_clicked = None;
                        }

                    }
        
                }
            });

            ui.horizontal_top(|ui| {

                // Refresh group list button.
                if ui.button(crate::defines::AF_REFRESH_CODE.to_string()).clicked() {
                    app.get_groups_promise = Some(api::group::get_groups(ctx));
                }

            });

            ui.separator();

            ui.vertical_centered_justified(|ui| {

                // Groups buttons.
                if app.groups.is_some() {                         

                    for group in app.groups.as_ref().unwrap().iter() {
                    
                        let button_label = format!("{} {}", crate::defines::AF_GROUP_CODE, &group.cn);

                        // Save the clicked group name.
                        if ui.button(button_label).clicked() {
                            app.group_button_clicked = Some(group.cn.clone());
                            app.directory_button_clicked = None;
                        }

                    }
        
                }
            });
          
    });

    egui::CentralPanel::default().show(ctx, |ui| {

        // If a directory is clicked then display its acls.
        if let Some(d) = &app.directory_button_clicked {

            ui.heading(d);

            ui.separator();

            let acls = app.directories_map.get(d).unwrap(); // this should never panic as the key always exists

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

                        },
                        storm_daenerys_common::types::acl::Qualifier::Group(_) => {

                            ui.label(AF_GROUP_CODE.to_string());
                            ui.label(acl.qualifier_cn.as_ref().unwrap());

                            match acl.perm {
                                4 | 5 | 7 => ui.label(egui::RichText::new("(read only)").italics()),
                                _ => ui.label(""), 
                            };

                            ui.end_row();

                        },
                        _ => (),
                    }

                }

            });

            ui.separator();

            // Edit directory button.
            let button_label = format!("{} {}", crate::defines::AF_EDIT_CODE, "edit directory");

            if ui.button(button_label).clicked() {

                app.edit_directory_clicked = Some(d.to_string());
                app.edit_group_clicked = None;

                // Initialize the edited_directory_permissions hashmap.
                let mut edited_directory_widget_state: HashMap<String, (Qualifier, bool)> = HashMap::new();
                let acls = app.directories_map.get(d).unwrap(); // this should never panic as the key always exists

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

                    edited_directory_widget_state.insert(acl.qualifier_cn.clone().unwrap(),  (acl.qualifier.clone(), is_read_only));

                }

                app.edited_directory_widget_state = Some(edited_directory_widget_state);
            
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
                },
                None => {
                    ui.label("no members".to_string());
                },
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
        if let Some(d) = &app.edit_directory_clicked {

            ui.heading(d);

            ui.separator();

            let acls = app.directories_map.get(d).unwrap(); // this should never panic as the key always exists

            egui::Grid::new("acl_list_edit").num_columns(3).show(ui, |ui| {

                for acl in acls {
                    match acl.qualifier {
                        storm_daenerys_common::types::acl::Qualifier::User(_) => {

                                ui.label(AF_USER_CODE.to_string());
                                ui.label(acl.qualifier_cn.as_ref().unwrap());

                                let (_, mut read_only) = app.edited_directory_widget_state.as_ref().unwrap().get(acl.qualifier_cn.as_ref().unwrap()).unwrap();

                                ui.checkbox(&mut read_only, "read only".to_string());

                                ui.end_row();

                        },
                        storm_daenerys_common::types::acl::Qualifier::Group(_) => {

                            ui.label(AF_GROUP_CODE.to_string());
                            ui.label(acl.qualifier_cn.as_ref().unwrap());

                            match acl.perm {
                                4 | 5 | 7 => ui.label(egui::RichText::new("(read only)").italics()),
                                _ => ui.label(""), 
                            };

                            ui.end_row();

                        },
                        _ => (),
                    }

                }

            });

        }

        // Group edition.
        if let Some(g) = &app.edit_group_clicked {
        }

    });

}