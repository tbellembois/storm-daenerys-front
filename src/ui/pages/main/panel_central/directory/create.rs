use egui::Ui;
use storm_daenerys_common::types::directory::CreateDirectory;

use crate::{api, defines::AF_CREATE_CODE, ui::daenerys::DaenerysApp};

pub fn render_create_directory(app: &mut DaenerysApp, ctx: &egui::Context, ui: &mut Ui) {
    // Directory name.
    ui.vertical(|ui| {
        ui.horizontal(|ui| {
            if let Some(admin_restriction) = &app.current_admin_restriction {
                ui.label(format!("{}@_", admin_restriction.to_uppercase()));
            }
            ui.add(
                egui::TextEdit::singleline(&mut app.create_directory_name).hint_text(
                    "directory name (no space, no accent or special character except - and _)",
                ),
            );
        });

        // Create directory button.
        // Validate name, disable create button until valid.
        let mut enabled: bool = true;
        if app.create_directory_name.clone().len() < 2
            || !app
                .directory_name_re
                .is_match(app.create_directory_name.clone().as_str())
        {
            enabled = false;
        }
        ui.add_enabled_ui(enabled, |ui| {
            let button_label = format!("{} {}", AF_CREATE_CODE, "create");
            let button = egui::Button::new(button_label);

            if ui.add_sized([150., 30.], button).clicked() {
                app.current_info = Some(format!(
                    "creating directory {}",
                    app.create_directory_name.clone()
                ));

                let create_directory = CreateDirectory {
                    name: app.create_directory_name.clone(),
                };

                app.is_working = true;
                app.create_directory_promise = Some(api::directory::create_directory(
                    ctx,
                    create_directory,
                    app.api_url.clone(),
                ));

                app.create_directory_name.clear();
            }
        });
    });
}
