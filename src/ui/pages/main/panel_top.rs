use eframe::{
    egui::{self, Context, RichText, Visuals},
    epaint::Color32,
};
use egui::{Frame, Margin};

use crate::{
    defines::{
        AF_ERROR_CODE, AF_INFO_CODE, AF_MOON_CODE, AF_SUN_CODE, DARK_BACKGROUND_COLOR,
        LIGHT_BACKGROUND_COLOR,
    },
    ui::daenerys::DaenerysApp,
};

pub fn display_top_panel(app: &mut DaenerysApp, ctx: &Context, _frame: &mut eframe::Frame) {
    let background: Color32 = if app.theme.dark_mode {
        DARK_BACKGROUND_COLOR
    } else {
        LIGHT_BACKGROUND_COLOR
    };

    egui::TopBottomPanel::top("error_info_panel")
        .min_height(140.)
        .max_height(140.)
        .show_separator_line(false)
        .frame(Frame {
            inner_margin: Margin {
                left: 10.0,
                right: 10.0,
                top: 10.0,
                bottom: 50.0,
            },
            fill: background,
            ..Default::default()
        })
        .show(ctx, |ui| {
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

            // Switch theme.
            if app.theme.dark_mode {
                let button_label = format!("{}", AF_SUN_CODE);

                let button = egui::Button::new(button_label);

                if ui.add_sized([30., 30.], button).clicked() {
                    app.theme = Visuals::light();
                    ctx.set_visuals(Visuals::light());
                }
            } else {
                let button_label = format!("{}", AF_MOON_CODE);

                let button = egui::Button::new(button_label);

                if ui.add_sized([30., 30.], button).clicked() {
                    app.theme = Visuals::dark();
                    ctx.set_visuals(Visuals::dark());
                }
            }

            // Logo.
            // ui.vertical_centered(|ui| {
            if app.theme.dark_mode {
                ui.image(egui::include_image!("../../media/storm-dark.svg"));
            } else {
                ui.image(egui::include_image!("../../media/storm-light.svg"));
            }
            // });
        });
}
