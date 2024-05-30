use super::{acl::ui::render_show_edit_acl, name::ui::render_rename, quota::ui::render_edit_quota};
use crate::{
    api,
    defines::{AF_DELETE_CODE, AF_EDIT_CODE, AF_FOLDER_CODE, AF_QUOTA_CODE, AF_RENAME_CODE},
    ui::daenerys::{Action, DaenerysApp},
};
use egui::Ui;
use human_bytes::human_bytes;
use storm_daenerys_common::types::directory::CreateDirectory;

pub fn render_show_directory(app: &mut DaenerysApp, ctx: &egui::Context, ui: &mut Ui) {
    // Directory name.
    ui.heading(format!(
        "{} {}",
        AF_FOLDER_CODE,
        app.current_directory.as_ref().unwrap().name
    ));
    if let Some(quota) = app.current_directory.as_ref().unwrap().quota {
        if quota.ne(&0) {
            ui.label(format!("{} {}", AF_QUOTA_CODE, human_bytes(quota as f64)));
        }
    }

    // ACLs details and edition.
    if app.active_action.to_string().starts_with("directory_edit") {
        render_show_edit_acl(app, ctx, ui);
    }

    // ACLs, quota edit, rename and delete buttons.
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

            if let Some(admin_restriction) = &app.current_admin_restriction {
                if app
                    .current_directory
                    .as_ref()
                    .unwrap()
                    .name
                    .starts_with(admin_restriction)
                {
                    let button_label = format!("{} {}", AF_RENAME_CODE, "rename");
                    let button = egui::Button::new(button_label);

                    if ui.add_sized([150., 30.], button).clicked() {
                        app.active_action = Action::DirectoryEditRename;
                    }

                    let button_label = format!("{} {}", AF_DELETE_CODE, "delete");
                    let button = egui::Button::new(button_label);

                    if ui.add_sized([150., 30.], button).clicked() {
                        let current_directory =
                            app.current_directory.as_ref().unwrap().name.clone();

                        app.current_info =
                            Some(format!("deleting directory {}", current_directory));

                        let delete_directory = CreateDirectory {
                            name: current_directory,
                        };

                        app.is_working = true;
                        app.delete_directory_promise = Some(api::directory::delete_directory(
                            ctx,
                            delete_directory,
                            app.api_url.clone(),
                        ));
                    }
                }
            }
        });
    }

    // Quota edition.
    if app.active_action == Action::DirectoryEditQuota {
        render_edit_quota(app, ctx, ui);
    }

    // Name edition.
    if app.active_action == Action::DirectoryEditRename {
        render_rename(app, ctx, ui);
    }
}
