use super::{acl::ui::render_show_edit_acl, quota::ui::render_edit_quota};
use crate::{
    defines::{AF_EDIT_CODE, AF_FOLDER_CODE, AF_QUOTA},
    ui::daenerys::{Action, DaenerysApp},
};
use egui::Ui;
use human_bytes::human_bytes;

pub fn render_show_directory(app: &mut DaenerysApp, ctx: &egui::Context, ui: &mut Ui) {
    // Directory name.
    ui.heading(format!(
        "{} {}",
        AF_FOLDER_CODE,
        app.active_directory.as_ref().unwrap().name
    ));
    if let Some(quota) = app.active_directory.as_ref().unwrap().quota {
        if quota.ne(&0) {
            ui.label(format!("{} {}", AF_QUOTA, human_bytes(quota as f64)));
        }
    }

    // ACLs details and edition.
    if app.active_action.to_string().starts_with("directory_edit") {
        render_show_edit_acl(app, ctx, ui);
    }

    // ACLs and quota edit buttons.
    if app.active_action == Action::DirectoryEdit {
        ui.add_space(20.0);

        ui.horizontal_top(|ui| {
            let button_label = format!("{} {}", AF_EDIT_CODE, "edit ACLs");
            let button = egui::Button::new(button_label);

            if ui.add_sized([150., 30.], button).clicked() {
                app.active_action = Action::DirectoryEditAcl;
            }

            let button_label = format!("{} {}", AF_EDIT_CODE, "edit quota");
            let button = egui::Button::new(button_label);

            if ui.add_sized([150., 30.], button).clicked() {
                app.active_action = Action::DirectoryEditQuota;
            }
        });
    }

    // Quota edition.
    if app.active_action == Action::DirectoryEditQuota {
        render_edit_quota(app, ctx, ui);
    }
}
