use super::member::ui::render_show_edit_member;
use crate::{
    api::group::delete_group,
    defines::{AF_CONFIRM_CODE, AF_DELETE_CODE, AF_EDIT_CODE, AF_GROUP_CODE},
    ui::daenerys::{Action, DaenerysApp},
};
use egui::Ui;
use storm_daenerys_common::types::group::Group;

pub fn render_show_group(app: &mut DaenerysApp, ctx: &egui::Context, ui: &mut Ui) {
    let mut is_group_auto: bool = false;
    let mut is_group_invite: bool = false;

    // Check if group is auto or invite.
    match &app.root_groups {
        Some(root_groups) => {
            for root_group in root_groups {
                if app.active_group.as_ref().unwrap().cn.eq(&format!(
                    "{}-{}",
                    app.group_prefix.as_ref().unwrap(),
                    root_group,
                )) {
                    is_group_auto = true;
                    break;
                }

                if app.active_group.as_ref().unwrap().cn.eq(&format!(
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
            is_group_auto = app
                .active_group
                .as_ref()
                .unwrap()
                .cn
                .eq(app.group_prefix.as_ref().unwrap());
            is_group_invite = app
                .active_group
                .as_ref()
                .unwrap()
                .cn
                .eq(&format!("{}-invite", app.group_prefix.as_ref().unwrap()));
        }
    }

    // Group name.
    ui.heading(format!(
        "{} {}",
        AF_GROUP_CODE,
        app.active_group.as_ref().unwrap().cn
    ));
    ui.label(egui::RichText::new(app.active_group.as_ref().unwrap().description.clone()).italics());

    ui.add_space(20.0);

    // Members details.
    render_show_edit_member(app, ctx, ui);

    ui.add_space(20.0);

    // Edit members and delete group buttons.
    if app.active_action.to_string().starts_with("group_edit") {
        ui.horizontal_top(|ui| {
            if !app.is_working && app.active_action == Action::GroupEdit {
                let button_label = format!("{} {}", AF_EDIT_CODE, "edit members");
                let button = egui::Button::new(button_label);

                if !is_group_auto && ui.add_sized([150., 30.], button).clicked() {
                    app.edit_group_clicked_backup = Some(Box::new(Group {
                        ..*app.active_group.as_ref().unwrap().clone()
                    }));
                    app.active_action = Action::GroupEditUsers;
                }

                let button_label = format!("{} {}", AF_DELETE_CODE, "delete group");
                let button = egui::Button::new(button_label);

                if !is_group_invite
                    && !is_group_auto
                    && !app.is_working
                    && ui.add_sized([150., 30.], button).clicked()
                {
                    app.active_action = Action::GroupEditDeleteConfirm;
                }
            }

            if !app.is_working && app.active_action == Action::GroupEditDeleteConfirm {
                let button_label = format!("{} {}", AF_CONFIRM_CODE, "confirm deletion");
                let button = egui::Button::new(button_label);
                if ui.add_sized([150., 30.], button).clicked() {
                    app.is_working = true;
                    app.delete_group_promise = Some(delete_group(
                        ctx,
                        app.active_group.as_ref().unwrap().cn.clone(),
                        app.api_url.clone(),
                    ));

                    app.active_action = Action::Home;
                }
            }
        });
    }
}
