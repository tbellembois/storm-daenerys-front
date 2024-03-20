use egui::Ui;
use storm_daenerys_common::types::{
    directory::Directory,
    quota::{QuotaUnit, SetQuota},
};

use crate::{api::quota::save_quota, error::apperror::AppError, ui::daenerys::DaenerysApp};

pub fn render_edit_quota(
    app: &mut DaenerysApp,
    ctx: &egui::Context,
    ui: &mut Ui,
    directory_button_clicked: Box<Directory>,
) {
    ui.label("Set quota to 0 to remove it.");

    ui.add_space(10.0);

    ui.horizontal_top(|ui| {
        ui.add_sized(
            [200., 30.],
            egui::TextEdit::singleline(&mut app.edited_directory_quota).hint_text("enter quota"),
        );
        egui::ComboBox::from_label("")
            .selected_text(format!("{}", app.edited_directory_quota_unit))
            .show_ui(ui, |ui| {
                ui.selectable_value(
                    &mut app.edited_directory_quota_unit,
                    QuotaUnit::Megabyte,
                    "MiB",
                );
                ui.selectable_value(
                    &mut app.edited_directory_quota_unit,
                    QuotaUnit::Gigabyte,
                    "GiB",
                );
                ui.selectable_value(
                    &mut app.edited_directory_quota_unit,
                    QuotaUnit::Terabyte,
                    "TiB",
                );
            });
    });

    ui.add_space(20.0);

    // Save button.
    let mut enabled: bool = true;
    if app.edited_directory_quota.clone().is_empty()
        || !app
            .quota_format_re
            .is_match(app.edited_directory_quota.clone().as_str())
    {
        enabled = false;
    }

    ui.add_enabled_ui(enabled, |ui| {
        let button_label = format!("{} {}", crate::defines::AF_SAVE_CODE, "save");
        let button = egui::Button::new(button_label);

        if ui.add_sized([150., 30.], button).clicked() {
            let directory_name = directory_button_clicked.name.clone();

            app.current_info = Some(format!("saving quota for {}", directory_name));

            let maybe_quota = app.edited_directory_quota.parse::<u64>();

            if let Err(e) = maybe_quota {
                app.current_error = Some(AppError::InternalError(e.to_string()));
            } else {
                let new_quota = match app.edited_directory_quota_unit {
                    QuotaUnit::Megabyte => maybe_quota.unwrap() * 1024 * 1024,
                    QuotaUnit::Gigabyte => maybe_quota.unwrap() * 1024 * 1024 * 1024,
                    QuotaUnit::Terabyte => maybe_quota.unwrap() * 1024 * 1024 * 1024 * 1024,
                };
                let set_quota = SetQuota {
                    name: directory_name,
                    quota: new_quota,
                };

                app.is_working = true;
                app.save_directory_quota_promise =
                    Some(save_quota(ctx, set_quota, app.api_url.clone()));
            }
        }
    });
}
