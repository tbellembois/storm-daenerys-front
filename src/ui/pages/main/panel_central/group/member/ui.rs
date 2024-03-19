use egui::Ui;
use storm_daenerys_common::types::group::Group;

use crate::{
    api::{self, group::delete_group},
    ui::daenerys::DaenerysApp,
};

pub fn render_show_edit_member(
    app: &mut DaenerysApp,
    ctx: &egui::Context,
    ui: &mut Ui,
    group_button_clicked: Box<Group>,
    is_group_invite: bool,
) {
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
                                            egui::Color32::from_rgb(0, 0, 0),
                                        ),
                                        None => (
                                            format!("<invalid account> ({})", member),
                                            egui::Color32::from_rgb(255, 0, 0),
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

                                        (member.to_string(), egui::Color32::from_rgb(255, 165, 0))
                                    }
                                };

                                ui.label(egui::RichText::new(display).color(color));

                                if (app.is_group_editing && !app.edit_group_add_user_clicked) {
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
    if !app.is_working && app.is_group_editing {
        ui.horizontal_top(|ui| {
            let button_label = format!("{} {}", crate::defines::AF_ADD_CODE, "add user");

            let button = egui::Button::new(button_label);

            if !app.edit_group_add_user_clicked && ui.add_sized([150., 30.], button).clicked() {
                app.edit_group_add_user_clicked = true;
            }
        });
    }
}
