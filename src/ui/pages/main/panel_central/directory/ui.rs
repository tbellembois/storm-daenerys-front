use crate::ui::daenerys::DaenerysApp;
use egui::Ui;
use storm_daenerys_common::types::directory::Directory;

use super::{acl::ui::render_show_edit_acl, quota::ui::render_edit_quota};

pub fn render_show_directory(
    app: &mut DaenerysApp,
    ctx: &egui::Context,
    ui: &mut Ui,
    directory_button_clicked: Box<Directory>,
) {
    app.application_just_loaded = false;

    // Directory name.
    ui.heading(format!(
        "{} {}",
        crate::defines::AF_FOLDER_CODE,
        directory_button_clicked.name
    ));

    ui.add_space(20.0);

    // ACLs details and edition.
    if !app.is_directory_quota_editing {
        render_show_edit_acl(
            app,
            ctx,
            ui,
            &directory_button_clicked.acls,
            directory_button_clicked.clone(),
        );
    }

    // ACLs and quota edit buttons.
    ui.horizontal_top(|ui| {
        if !app.is_directory_acl_editing && !app.is_directory_quota_editing {
            ui.add_space(20.0);

            let button_label = format!("{} {}", crate::defines::AF_EDIT_CODE, "edit ACLs");
            let button = egui::Button::new(button_label);

            if ui.add_sized([150., 30.], button).clicked() {
                app.is_directory_acl_editing = true;
                app.is_group_editing = false;
            }
        }

        if !app.is_directory_acl_editing && !app.is_directory_quota_editing {
            ui.add_space(20.0);

            let button_label = format!("{} {}", crate::defines::AF_EDIT_CODE, "edit quota");
            let button = egui::Button::new(button_label);

            if ui.add_sized([150., 30.], button).clicked() {
                app.is_directory_quota_editing = true;
                app.is_group_editing = false;
            }
        }
    });

    // Quota edition.
    if app.is_directory_quota_editing {
        render_edit_quota(app, ctx, ui, directory_button_clicked.clone());
    }
}
