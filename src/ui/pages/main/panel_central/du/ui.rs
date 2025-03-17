use egui::Ui;

use crate::ui::daenerys::DaenerysApp;

pub fn render_disk_usage(app: &mut DaenerysApp, ui: &mut Ui) {
    let available_height: f32 = ui.available_size().y;
    let scroll_height: f32 = available_height - 50.;

    egui::ScrollArea::vertical()
        .id_salt("du_scroll")
        .max_height(scroll_height)
        .show(ui, |ui| {
            ui.label("disk usage is updated every two hours");
            ui.label(
                egui::RichText::new(app.du.as_ref().unwrap())
                    .text_style(egui::TextStyle::Monospace),
            );
        });
}
