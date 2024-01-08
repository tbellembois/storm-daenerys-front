use eframe::{
    egui::{self, Context, RichText, Visuals},
    epaint::Color32,
};
use egui::{Frame, Margin};

use crate::{
    api,
    defines::{
        AF_ERROR_CODE, AF_FOLDER_CODE, AF_GAUGE_CODE, AF_GROUP_CODE, AF_INFO_CODE, AF_MOON_CODE,
        AF_SUN_CODE,
    },
    ui::daenerys::DaenerysApp,
};

pub fn display_top_panel(app: &mut DaenerysApp, ctx: &Context, _frame: &mut eframe::Frame) {
    egui::TopBottomPanel::top("error_info_panel")
        .min_height(40.)
        .max_height(40.)
        .show_separator_line(false)
        .frame(Frame {
            inner_margin: Margin {
                left: 10.0,
                right: 10.0,
                top: 10.0,
                bottom: 0.0,
            },
            fill: app.background_color,
            ..Default::default()
        })
        .show(ctx, |ui| {
            // Logo STORM.
            ui.image(egui::include_image!("../../media/storm-logo.svg"));
            ui.add_space(10.0);

            ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                //
                // Switch theme.
                //
                if app.theme.dark_mode {
                    let button = egui::Button::new(format!("{} light mode", AF_SUN_CODE));

                    if ui.add_sized([30., 30.], button).clicked() {
                        app.theme = Visuals::light();
                        ctx.set_visuals(Visuals::light());
                    }
                } else {
                    let button = egui::Button::new(format!("{} dark mode", AF_MOON_CODE));

                    if ui.add_sized([30., 30.], button).clicked() {
                        app.theme = Visuals::dark();
                        ctx.set_visuals(Visuals::dark());
                    }
                }

                // Disk usage button.
                let button = egui::Button::new(format!("{} show disk usage", AF_GAUGE_CODE));

                if ui.add_sized([150., 30.], button).clicked() {
                    app.is_working = true;
                    app.create_group_clicked = false;
                    app.directory_button_clicked = None;
                    app.group_button_clicked = None;
                    app.is_directory_editing = false;
                    app.is_group_editing = false;

                    let available_width = app.central_panel_available_size.x;
                    let du_width = available_width as u32 / 2;
                    app.get_du_promise =
                        Some(api::root::get_du(ctx, app.api_url.clone(), du_width));
                };

                // Hide directories button.
                let button = egui::Button::new(format!("{} toogle directory view", AF_FOLDER_CODE));

                if ui.add_sized([150., 30.], button).clicked() {
                    app.show_directory_list = !app.show_directory_list;
                }

                // Hide groups button.
                let button = egui::Button::new(format!("{} toogle group view", AF_GROUP_CODE));

                if ui.add_sized([150., 30.], button).clicked() {
                    app.show_group_list = !app.show_group_list;
                }

                // Current error label.
                if let Some(current_error) = &app.current_error {
                    ui.label(
                        RichText::new(format!("{} {}", AF_ERROR_CODE, current_error))
                            .color(Color32::DARK_RED),
                    );
                }

                // Current info label.
                if let Some(current_info) = &app.current_info {
                    ui.label(
                        RichText::new(format!("{} {}", AF_INFO_CODE, current_info))
                            .color(Color32::DARK_GREEN),
                    );
                }
            });

            ui.add_space(10.0);
        });
}
