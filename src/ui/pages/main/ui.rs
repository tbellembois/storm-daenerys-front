use super::{
    panel_bottom::ui::render_bottom_panel, panel_central::ui::render_central_panel,
    panel_left::ui::render_left_panel, panel_right::ui::render_right_panel,
    panel_top::ui::render_top_panel,
};
use crate::ui::daenerys::DaenerysApp;
use eframe::egui;

pub fn update(app: &mut DaenerysApp, ctx: &egui::Context, frame: &mut eframe::Frame) {
    render_top_panel(app, ctx, frame);
    render_bottom_panel(app, ctx, frame);
    if app.toggle_side_panels {
        render_left_panel(app, ctx);
        render_right_panel(app, ctx);
    }
    render_central_panel(app, ctx, frame);
}
