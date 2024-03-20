use egui::Ui;
use storm_daenerys_common::types::group::Group;

use crate::{
    api::group::{delete_group, save_group},
    ui::daenerys::DaenerysApp,
};

use super::member::{add_user::render_add_user, ui::render_show_edit_member};

pub fn render_show_group(
    app: &mut DaenerysApp,
    ctx: &egui::Context,
    ui: &mut Ui,
    group_button_clicked: Box<Group>,
) {
    let mut is_group_auto: bool = false;
    let mut is_group_invite: bool = false;

    // Check if group is auto or invite.
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
            is_group_auto = group_button_clicked
                .cn
                .eq(app.group_prefix.as_ref().unwrap());
            is_group_invite = group_button_clicked
                .cn
                .eq(&format!("{}-invite", app.group_prefix.as_ref().unwrap()));
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

    // Members details.
    render_show_edit_member(app, ctx, ui, group_button_clicked.clone(), is_group_invite);

    ui.add_space(20.0);

    // Edit members and delete group buttons.
    ui.horizontal_top(|ui| {
        let button_label = format!("{} {}", crate::defines::AF_EDIT_CODE, "edit members");
        let button = egui::Button::new(button_label);

        if !is_group_auto && !app.is_group_editing && ui.add_sized([150., 30.], button).clicked() {
            app.edit_group_clicked_backup = Some(Box::new(Group {
                ..*group_button_clicked.clone()
            }));
            app.is_directory_acl_editing = false;
            app.is_group_editing = true;
        }

        let button_label = format!("{} {}", crate::defines::AF_DELETE_CODE, "delete group");
        let button = egui::Button::new(button_label);

        if !is_group_invite
            && !app.is_working
            && !app.edit_group_delete_confirm
            && !app.edit_group_add_user_clicked
            && !app.is_group_editing
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
}
