use super::{
    directory::{create::render_create_directory, ui::render_show_directory},
    du::ui::render_disk_usage,
    group::{create::render_create_group, ui::render_show_group},
    home::ui::render_home,
};
use crate::ui::daenerys::{Action, DaenerysApp};
use eframe::egui::{self, Context};
use egui::Frame;

pub fn render_central_panel(app: &mut DaenerysApp, ctx: &Context, _frame: &mut eframe::Frame) {
    egui::CentralPanel::default()
        .frame(Frame {
            inner_margin: 15.0.into(),
            // inner_margin: app.state.active_theme.margin_style().into(),
            // fill: app.state.active_theme.bg_primary_color_visuals(),
            // stroke: egui::Stroke::new(1.0, app.state.active_theme.bg_secondary_color_visuals()),
            ..Default::default()
        })
        .show(ctx, |ui| {
            app.central_panel_available_size = ui.available_size();

            // Show a spinner on REST requests or any other time consuming work.
            if app.is_working {
                ui.add_sized([0., 40.], egui::widgets::Spinner::new());
            } else {
                ui.add_sized([0., 40.], egui::Label::new(""));
            }

            // Disk usage.
            if app.active_action == Action::DiskUsage {
                render_disk_usage(app, ui);
            }

            // Home.
            if app.active_action == Action::Home {
                render_home(ui);
            }

            // Create directory form.
            if app.active_action == Action::DirectoryCreate {
                render_create_directory(app, ctx, ui);
            }

            // Create group form.
            if app.active_action == Action::GroupCreate {
                render_create_group(app, ctx, ui);
            }

            // Directory details and edition.
            if app.active_action.to_string().starts_with("directory_edit") {
                render_show_directory(app, ctx, ui);
            }

            // Group details and edition.
            if app.active_action.to_string().starts_with("group_edit") {
                render_show_group(app, ctx, ui);
            };
        });
}
