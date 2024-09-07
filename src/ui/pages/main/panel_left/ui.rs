use super::directory::render_directory_list;
use crate::ui::daenerys::DaenerysApp;
use eframe::egui::{self, Context};
use egui::Frame;

pub fn render_left_panel(app: &mut DaenerysApp, ctx: &Context) {
    egui::SidePanel::left("directory_list")
        .frame(Frame {
            inner_margin: app.state.active_theme.margin_style().into(),
            fill: app.state.active_theme.bg_secondary_color_visuals(),
            stroke: egui::Stroke::new(1.0, app.state.active_theme.bg_secondary_color_visuals()),
            ..Default::default()
        })
        .resizable(false)
        .show_separator_line(true)
        .show(ctx, |ui| {
            ui.set_width(400.0);

            // Calculate scroll height.
            let available_height: f32 = ui.available_size().y;
            let scroll_height = available_height - 100.;

            // Directory list.
            render_directory_list(app, ctx, ui, scroll_height)
        });
}
