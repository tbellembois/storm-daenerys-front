use crate::ui::daenerys::DaenerysApp;
use eframe::egui::{self, Context};
use egui::{Frame, Margin};

use super::{
    directory::{create::render_create_directory, ui::render_show_directory},
    du::ui::render_disk_usage,
    group::{create::render_create_group, ui::render_show_group},
    home::ui::render_home,
};

pub fn render_central_panel(app: &mut DaenerysApp, ctx: &Context, _frame: &mut eframe::Frame) {
    egui::CentralPanel::default()
        .frame(Frame {
            inner_margin: Margin {
                left: 20.0,
                right: 20.0,
                top: 10.0,
                bottom: 10.0,
            },
            fill: app.background_color,
            ..Default::default()
        })
        .show(ctx, |ui| {
            app.central_panel_available_size = ui.available_size();

            // Render home at startup.
            if app.application_just_loaded {
                render_home(ui);
            }

            // Show a spinner on REST requests or any other time consuming work.
            if app.is_working {
                ui.add_sized([0., 40.], egui::widgets::Spinner::new());
            } else {
                ui.add_sized([0., 40.], egui::Label::new(""));
            }

            // Disk usage.
            if app.du.is_some() {
                app.application_just_loaded = false;
                render_disk_usage(app, ui);
            }

            // Create directory form.
            if app.create_directory_clicked {
                app.application_just_loaded = false;
                render_create_directory(app, ctx, ui);
            }

            // Create group form.
            if app.create_group_clicked {
                app.application_just_loaded = false;
                render_create_group(app, ctx, ui);
            }

            // Directory details and edition.
            if let Some(directory_button_clicked) = &app.directory_button_clicked {
                app.application_just_loaded = false;
                render_show_directory(app, ctx, ui, directory_button_clicked.clone());
            }

            // Group details and edition.
            if let Some(group_button_clicked) = &app.group_button_clicked {
                app.application_just_loaded = false;
                render_show_group(app, ctx, ui, group_button_clicked.clone());
            };
        });
}
