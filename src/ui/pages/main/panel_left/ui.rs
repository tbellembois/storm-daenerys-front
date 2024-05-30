use super::{directory::render_directory_list, group::render_group_list, quota::render_quota};
use crate::ui::daenerys::DaenerysApp;
use eframe::egui::{self, Context};
use egui::Frame;

pub fn render_left_panel(app: &mut DaenerysApp, ctx: &Context) {
    egui::SidePanel::left("group_and_directory_list")
        .frame(Frame {
            inner_margin: app.state.active_theme.margin_style().into(),
            fill: app.state.active_theme.bg_secondary_color_visuals(),
            stroke: egui::Stroke::new(1.0, app.state.active_theme.bg_secondary_color_visuals()),
            ..Default::default()
        })
        .resizable(false)
        .show_separator_line(true)
        .show(ctx, |ui| {
            ui.set_width(300.0);

            // Calculate scroll height.
            let available_height: f32 = ui.available_size().y;
            let scroll_height = if app.show_directory_list ^ app.show_group_list {
                available_height - 100.
            } else {
                (available_height - 100.) / 2.
            };

            // Root quota.
            render_quota(app, ui);

            ui.add_space(20.0);

            // Directory list.
            if app.show_directory_list {
                render_directory_list(app, ctx, ui, scroll_height)
            }

            // Group list.
            if app.show_group_list {
                render_group_list(app, ctx, ui, scroll_height);
            }
        });
}
