use eframe::{
    egui::{self, Context, RichText, Visuals},
    epaint::Color32,
};
use egui::{Frame, Margin};

use crate::{
    defines::{
        AF_ERROR_CODE, AF_INFO_CODE, AF_MOON_CODE, AF_RUST_CODE, AF_SUN_CODE,
        DARK_BACKGROUND_COLOR, LIGHT_BACKGROUND_COLOR,
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
        .min_height(120.)
        .max_height(120.)
        .show_separator_line(false)
        .frame(Frame {
            inner_margin: Margin {
                left: 10.0,
                right: 10.0,
                top: 10.0,
                bottom: 0.0,
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

            ui.horizontal(|ui| {
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

                // Rust !
                let button_label = format!("{}", AF_RUST_CODE);

                let button = egui::Button::new(button_label);

                if ui.add_sized([30., 30.], button).clicked() {
                    app.rust = true;
                }
            });

            if app.rust {
                egui::Window::new("Rust credits")
                    .collapsible(false)
                    .movable(true)
                    .show(ctx, |ui| {
                        ui.label("Developped with Rust.");
                        ui.image(egui::include_image!("../../media/ferris.svg"));
                        ui.hyperlink("https://www.rust-lang.org/");
                        ui.hyperlink("https://github.com/emilk/egui");
                        ui.hyperlink("https://github.com/tokio-rs/axum");
                    });
            }

            // Logo STORM.
            // ui.vertical_centered_justified(|ui| {
            if app.theme.dark_mode {
                ui.image(egui::include_image!("../../media/storm-dark.svg"));
            } else {
                ui.image(egui::include_image!("../../media/storm-light.svg"));
            }
            // });
        });
}
