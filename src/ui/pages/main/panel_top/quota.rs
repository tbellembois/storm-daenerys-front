use egui::Ui;
use number_prefix::NumberPrefix;

use crate::ui::daenerys::DaenerysApp;

pub fn render_quota(app: &mut DaenerysApp, ui: &mut Ui) {
    // Root quota.
    if let Some(quota) = &app.quota {
        let used_space = quota.total_space - quota.available_space;
        let percent_used: f32 = (used_space * 100 / quota.total_space) as f32;
        let float_used: f32 = percent_used / 100.;

        let formated_total = match NumberPrefix::binary(quota.total_space as f32) {
            NumberPrefix::Standalone(bytes) => {
                format!("{} bytes", bytes)
            }
            NumberPrefix::Prefixed(prefix, n) => {
                format!("{:.1} {}B", n, prefix)
            }
        };
        let formated_used = match NumberPrefix::binary(used_space as f32) {
            NumberPrefix::Standalone(bytes) => {
                format!("{} bytes", bytes)
            }
            NumberPrefix::Prefixed(prefix, n) => {
                format!("{:.1} {}B", n, prefix)
            }
        };
        ui.vertical(|ui| {
            ui.horizontal_top(|ui| {
                ui.label("quota:");
                ui.label(formated_total);

                ui.label("used:");
                ui.label(formated_used);
            });
            ui.horizontal_top(|ui| {
                ui.add(egui::ProgressBar::new(float_used).show_percentage());
            });
        });
    }
}
