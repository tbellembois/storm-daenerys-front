use crate::ui::daenerys::DaenerysApp;
use egui::Ui;

pub fn render_add_group(app: &mut DaenerysApp, ui: &mut Ui) {
    // Group list.
    if app.groups.is_some() {
        for group in app.groups.as_ref().unwrap() {
            if ui.link(group.clone().cn).clicked() {
                app.edited_directory_add_group = Some(group.cn.clone());
            }
        }
    }

    ui.add_space(20.0);

    // Done button.
    let button_label = format!("{} {}", crate::defines::AF_CANCEL_CODE, "done");
    let button = egui::Button::new(button_label);

    if ui.add_sized([150., 30.], button).clicked() {
        app.edit_directory_add_group_clicked = false;
    }
}
