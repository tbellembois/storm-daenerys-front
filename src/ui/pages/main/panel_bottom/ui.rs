use crate::ui::daenerys::DaenerysApp;
use egui::Context;
use egui::Frame;

pub fn render_bottom_panel(app: &mut DaenerysApp, ctx: &Context, _frame: &mut eframe::Frame) {
    egui::TopBottomPanel::bottom("footer")
        .min_height(60.)
        .show_separator_line(true)
        .frame(Frame {
            inner_margin: app.state.active_theme.margin_style().into(),
            fill: app.state.active_theme.bg_secondary_color_visuals(),
            stroke: egui::Stroke::new(1.0, app.state.active_theme.bg_secondary_color_visuals()),
            ..Default::default()
        })
        .show(ctx, |ui| {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                // Logo associates.
                ui.image(egui::include_image!("../../../media/partenaires.png"));

                // STORM logo.
                ui.add_sized(
                    [200., 70.],
                    egui::Image::new(egui::include_image!("../../../media/storm-logo.svg")),
                );

                // Application version.
                ui.label(app.app_version.clone());
                ui.label(egui::RichText::new("version:").italics());
            });
        });
}
