use egui::{Color32, RichText};

use crate::ui::daenerys::DaenerysApp;

pub fn display_top_panel(app: &mut DaenerysApp, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    egui::TopBottomPanel::top("error_info_panel").show(ctx, |ui| {
        // Current error label.
        if let Some(current_error) = &app.current_error {
            ui.label(RichText::new(current_error.to_string()).color(Color32::RED));
        }

        // Current info label.
        if let Some(current_info) = &app.current_info {
            ui.label(RichText::new(current_info.to_string()));
        }
    });
}
