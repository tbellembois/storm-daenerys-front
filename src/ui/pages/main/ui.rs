use crate::ui::daenerys::DaenerysApp;

use super::panel_bottom::display_bottom_panel;
use super::panel_central::display_central_panel;
use super::panel_left::display_left_panel;
use super::panel_top::display_top_panel;

use eframe::egui;

pub fn update(app: &mut DaenerysApp, ctx: &egui::Context, frame: &mut eframe::Frame) {
    display_top_panel(app, ctx, frame);
    display_bottom_panel(app, ctx, frame);

    display_left_panel(app, ctx, frame);
    display_central_panel(app, ctx, frame);
}
