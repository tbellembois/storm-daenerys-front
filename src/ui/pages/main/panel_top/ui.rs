use eframe::egui::{self, Context, RichText};
use egui::{Color32, Frame};

use crate::{
    api,
    defines::{
        AF_CONNECTED_USER_CODE, AF_ERROR_CODE, AF_GAUGE_CODE, AF_INFO_CODE, AF_TOGGLE_OFF_CODE,
    },
    ui::daenerys::DaenerysApp,
};

use super::quota::render_quota;

pub fn render_top_panel(app: &mut DaenerysApp, ctx: &Context, _frame: &mut eframe::Frame) {
    egui::TopBottomPanel::top("error_info_panel")
        .min_height(40.)
        .max_height(200.)
        .show_separator_line(false)
        .frame(Frame {
            inner_margin: 15.0.into(),
            // inner_margin: app.state.active_theme.margin_style().into(),
            // fill: app.state.active_theme.bg_secondary_color_visuals(),
            // stroke: egui::Stroke::new(1.0, app.state.active_theme.bg_secondary_color_visuals()),
            ..Default::default()
        })
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    // STORM logo.
                    ui.add_sized(
                        [200., 70.],
                        egui::Image::new(egui::include_image!("../../../media/storm-logo.svg")),
                    );
                });

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.vertical(|ui| {
                        // Connected user.
                        if let Some(connected_user) = &app.connected_user {
                            ui.label(egui::RichText::new(format!(
                                "{} {}",
                                AF_CONNECTED_USER_CODE, connected_user
                            )));
                        }

                        // Root quota.
                        render_quota(app, ui);
                    })
                });
            });

            ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                // Switch theme.
                // egui::ComboBox::from_id_salt("settings_theme_combo_box")
                //     .width(200.0)
                //     .selected_text(app.state.active_theme.name())
                //     .show_ui(ui, |ui_combobox| {
                //         for theme in app.themes.iter() {
                //             let res: egui::Response = ui_combobox.selectable_value(
                //                 &mut app.state.active_theme,
                //                 theme.clone(),
                //                 theme.name(),
                //             );
                //             if res.changed() {
                //                 ui_combobox
                //                     .ctx()
                //                     .set_style(app.state.active_theme.custom_style());
                //             }
                //         }
                //     });

                // Disk usage button.
                let button = egui::Button::new(format!("{} show disk usage", AF_GAUGE_CODE));

                if ui.add_sized([150., 30.], button).clicked() {
                    app.is_working = true;

                    app.get_du_promise = Some(api::root::get_du(ctx, app.api_url.clone()));
                };

                // Toggle show/hide side panels.
                let button =
                    egui::Button::new(format!("{} show/hide side panels", AF_TOGGLE_OFF_CODE));

                if ui.add_sized([150., 30.], button).clicked() {
                    app.toggle_side_panels = !app.toggle_side_panels;
                };

                // Current error label.
                if let Some(current_error) = &app.current_error {
                    ui.label(
                        RichText::new(format!("{} {}", AF_ERROR_CODE, current_error))
                            .color(Color32::RED),
                    );
                }

                // Current info label.
                if let Some(current_info) = &app.current_info {
                    ui.label(
                        RichText::new(format!("{} {}", AF_INFO_CODE, current_info))
                            .color(Color32::GREEN),
                    );
                }
            });
        });
}
