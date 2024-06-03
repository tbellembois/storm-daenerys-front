use crate::{
    api,
    defines::{AF_ADD_CODE, AF_GROUP_CODE, AF_HALF_LOCK_CODE, AF_LOCK_CODE, AF_REFRESH_CODE},
    ui::daenerys::{Action, DaenerysApp},
};
use egui::Ui;

pub fn render_group_list(
    app: &mut DaenerysApp,
    ctx: &egui::Context,
    ui: &mut Ui,
    scroll_height: f32,
) {
    ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
        ui.label(
            egui::RichText::new("STORM GROUPS").size(18.0).color(
                app.state
                    .active_theme
                    .fg_primary_text_color_visuals()
                    .unwrap(),
            ),
        );
    });

    ui.horizontal_top(|ui| {
        ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
            // Reload button.
            let button = egui::Button::new(format!("{} reload", AF_REFRESH_CODE));
            if ui.add_sized([30., 30.], button).clicked() {
                app.get_groups_promise = Some(api::group::get_groups(ctx, app.api_url.clone()));
            }

            // Create group button.
            let button_label = format!("{} {}", AF_ADD_CODE, "create group");
            let button = egui::Button::new(button_label);

            if ui.add_sized([150., 30.], button).clicked() {
                app.active_action = Action::GroupCreate;

                app.current_directory = None;
                app.current_group = None;
                app.du = None;

                app.create_group_name.clear();
                app.create_group_description.clear();
            }
        });
    });

    ui.separator();

    // Group list.
    egui::ScrollArea::vertical()
        .id_source("group_scroll")
        .max_height(scroll_height)
        .show(ui, |ui| {
            if app.groups.is_some() {
                egui::Grid::new("group_list").num_columns(2).show(ui, |ui| {
                    for group in app.groups.as_ref().unwrap().iter() {
                        let mut is_group_auto: bool = false;
                        let mut is_group_invite: bool = false;

                        match &app.root_groups {
                            Some(root_groups) => {
                                for root_group in root_groups {
                                    if group.cn.eq(&format!(
                                        "{}-{}",
                                        app.group_prefix.as_ref().unwrap(),
                                        root_group,
                                    )) {
                                        is_group_auto = true;
                                        break;
                                    }

                                    if group.cn.eq(&format!(
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
                                is_group_auto = group.cn.eq(app.group_prefix.as_ref().unwrap());
                                is_group_invite = group
                                    .cn
                                    .eq(&format!("{}-invite", app.group_prefix.as_ref().unwrap()));
                            }
                        }

                        ui.add_sized([30., 30.], egui::Label::new(format!("{}", AF_GROUP_CODE)));

                        ui.horizontal(|ui| {
                            let mut button_label = group.cn.to_string();

                            if is_group_auto {
                                button_label = format!("{} {}", AF_LOCK_CODE, group.cn)
                            }
                            if is_group_invite {
                                button_label = format!("{} {}", AF_HALF_LOCK_CODE, group.cn)
                            }

                            let button = egui::Button::new(button_label);

                            // Save the clicked group name.
                            if ui.add_sized([200., 30.], button).clicked() {
                                app.active_action = Action::GroupEdit;
                                app.current_group = Some(Box::new(group.clone()));

                                app.current_directory = None;
                                app.current_error = None;
                                app.current_info = None;
                                app.du = None;
                            }
                        });

                        ui.end_row()
                    }
                });
            }
        });
}
