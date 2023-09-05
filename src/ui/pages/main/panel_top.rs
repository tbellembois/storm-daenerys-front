use std::thread::current;

use egui::{Color32, RichText, Visuals};

use crate::{
    defines::{AF_ERROR_CODE, AF_INFO_CODE, AF_MOON_CODE, AF_SUN_CODE},
    ui::daenerys::DaenerysApp,
};

pub fn display_top_panel(app: &mut DaenerysApp, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    egui::TopBottomPanel::top("error_info_panel").show(ctx, |ui| {
        // Current error label.
        if let Some(current_error) = &app.current_error {
            ui.label(
                RichText::new(format!("{} {}", AF_ERROR_CODE, current_error)).color(Color32::RED),
            );
        }

        // Current info label.
        if let Some(current_info) = &app.current_info {
            ui.label(
                RichText::new(format!("{} {}", AF_INFO_CODE, current_info)).color(Color32::GREEN),
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
        ui.vertical_centered(|ui| {
            app.storm_logo.as_ref().unwrap().show(ui);
        });

        app.separator_image.as_ref().unwrap().show(ui);
    });
}
