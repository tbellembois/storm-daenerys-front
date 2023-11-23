use egui::{Color32, Context};

use crate::{
    defines::{DARK_BACKGROUND_COLOR, LIGHT_BACKGROUND_COLOR},
    ui::daenerys::DaenerysApp,
};

use egui::{Frame, Margin};

pub fn display_bottom_panel(app: &mut DaenerysApp, ctx: &Context, _frame: &mut eframe::Frame) {
    let background: Color32 = if app.theme.dark_mode {
        DARK_BACKGROUND_COLOR
    } else {
        LIGHT_BACKGROUND_COLOR
    };

    egui::TopBottomPanel::bottom("footer")
        .min_height(60.)
        .max_height(60.)
        .show_separator_line(false)
        .frame(Frame {
            inner_margin: Margin {
                left: 10.0,
                right: 10.0,
                top: 5.0,
                bottom: 5.0,
            },
            fill: background,
            ..Default::default()
        })
        .show(ctx, |ui| {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                // Compilation time.
                ui.label(app.compilation_time.clone());
                ui.label(egui::RichText::new("build:").italics());

                // Logo associates.
                ui.image(egui::include_image!("../../media/partenaires.png"));
            });
        });
}
