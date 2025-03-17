use super::add_user::render_add_user;
use crate::{
    api::{self, group::save_group},
    defines::{AF_ADD_CODE, AF_DELETE_CODE, AF_SAVE_CODE},
    ui::daenerys::{Action, DaenerysApp},
};
use egui::{Color32, Ui};

pub fn render_show_edit_member(app: &mut DaenerysApp, ctx: &egui::Context, ui: &mut Ui) {
    match &app.current_group.as_ref().unwrap().clone().member {
        Some(members) => {
            let scroll_height = ui.available_height() - 150.;

            egui::ScrollArea::vertical()
                .id_salt("group_detail_scroll")
                .max_height(scroll_height)
                .show(ui, |ui| {
                    egui::Grid::new("group_detail")
                        .num_columns(2)
                        .show(ui, |ui| {
                            let mut sorted_members = members.clone();
                            sorted_members.sort_by(|a, b| {
                                let a_display = app
                                    .user_display_cache
                                    .get(a)
                                    .unwrap_or(&Some(a.clone()))
                                    .clone()
                                    .unwrap_or_default();
                                let b_display = app
                                    .user_display_cache
                                    .get(b)
                                    .unwrap_or(&Some(b.clone()))
                                    .clone()
                                    .unwrap_or_default();

                                a_display.to_lowercase().cmp(&b_display.to_lowercase())
                            });

                            for member in sorted_members {
                                let (display, color) = match app.user_display_cache.get(&member) {
                                    Some(maybe_display_name) => match maybe_display_name {
                                        Some(display_name) => (
                                            format!("{} ({})", display_name, member),
                                            Color32::WHITE,
                                        ),
                                        None => (
                                            format!("<invalid account> ({})", member),
                                            Color32::RED,
                                        ),
                                    },
                                    None => {
                                        if !app.get_user_display_promises.contains_key(&member) {
                                            app.get_user_display_promises.insert(
                                                member.to_string(),
                                                Some(api::user::get_user_display(
                                                    ctx,
                                                    member.clone(),
                                                    app.api_url.clone(),
                                                )),
                                            );
                                        }

                                        (member.to_string(), Color32::WHITE)
                                    }
                                };

                                ui.label(egui::RichText::new(display).color(color));

                                if app.active_action == Action::GroupEditUsers {
                                    // Delete member button.
                                    let button_label =
                                        format!("{} {}", AF_DELETE_CODE, "delete member");
                                    let button = egui::Button::new(button_label);

                                    if ui.add_sized([150., 25.], button).clicked()
                                        && app.current_group.as_ref().unwrap().member.is_some()
                                    {
                                        app.current_group
                                            .as_mut()
                                            .unwrap()
                                            .member
                                            .as_mut()
                                            .unwrap()
                                            .retain(|u| u.ne(&member.to_string()));
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

    // Add user button.
    if !app.is_working && app.active_action == Action::GroupEditUsers {
        ui.add_space(20.0);

        ui.horizontal_top(|ui| {
            let button_label = format!("{} {}", AF_ADD_CODE, "add user");
            let button = egui::Button::new(button_label);

            if ui.add_sized([150., 30.], button).clicked() {
                app.active_action = Action::GroupEditAddUser;
            }
        });

        ui.add_space(20.0);

        // Save button.
        let button_label = format!("{} {}", AF_SAVE_CODE, "save");
        let button = egui::Button::new(button_label);

        if ui.add_sized([150., 30.], button).clicked() {
            app.current_info = Some(format!(
                "saving group {}",
                app.current_group.as_ref().unwrap().cn
            ));

            app.is_working = true;
            app.save_group_promises = Some(save_group(
                ctx,
                *app.current_group_backup.as_ref().unwrap().clone(),
                *app.current_group.as_ref().unwrap().clone(),
                app.api_url.clone(),
            ));
        }
    }

    // User add.
    if app.active_action == Action::GroupEditAddUser {
        render_add_user(app, ctx, ui)
    }
}
