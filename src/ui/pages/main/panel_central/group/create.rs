use egui::Ui;
use storm_daenerys_common::types::group::Group;

use crate::{api, ui::daenerys::DaenerysApp};

pub fn render_create_group(app: &mut DaenerysApp, ctx: &egui::Context, ui: &mut Ui) {
    app.application_just_loaded = false;

    // Group name.
    ui.vertical(|ui| {
        ui.horizontal(|ui| {
            ui.label(format!("{}-", app.group_prefix.as_ref().unwrap()));
            ui.add_sized(
                [400., 30.],
                egui::TextEdit::singleline(&mut app.create_group_name)
                    .hint_text("group name (no space, no accent or special character except _)"),
            );
        });

        // Group description.
        ui.add_sized(
            [400., 30.],
            egui::TextEdit::singleline(&mut app.create_group_description).hint_text("description"),
        );

        // Create group button.
        // Validate name, disable create button until valid.
        let mut enabled: bool = true;
        if app.create_group_name.clone().len() < 2
            || !app
                .group_cn_re
                .is_match(app.create_group_name.clone().as_str())
        {
            enabled = false;
        }
        ui.add_enabled_ui(enabled, |ui| {
            let button_label = format!("{} {}", crate::defines::AF_CREATE_CODE, "create");

            let button = egui::Button::new(button_label);

            if ui.add_sized([150., 30.], button).clicked() {
                app.current_info =
                    Some(format!("creating group {}", app.create_group_name.clone()));

                let create_group = Group {
                    cn: app.create_group_name.clone(),
                    description: app.create_group_description.clone(),
                    owner: None,
                    member: None,
                };

                app.is_working = true;
                app.create_group_promise = Some(api::group::create_group(
                    ctx,
                    create_group,
                    app.api_url.clone(),
                ));

                app.create_group_name.clear();
                app.create_group_description.clear();
            }
        });
    });
}
