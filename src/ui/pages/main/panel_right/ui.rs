use crate::ui::{daenerys::DaenerysApp, pages::main::panel_left::group::render_group_list};
use eframe::egui::{self, Context};
use egui::Frame;

pub fn render_right_panel(app: &mut DaenerysApp, ctx: &Context) {
    egui::SidePanel::right("group_list")
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

            // Group list.
            render_group_list(app, ctx, ui, scroll_height);
        });
}
