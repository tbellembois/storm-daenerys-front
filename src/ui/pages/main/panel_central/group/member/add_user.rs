use egui::{Key, Ui};

use crate::{api, ui::daenerys::DaenerysApp};

pub fn render_add_user(app: &mut DaenerysApp, ctx: &egui::Context, ui: &mut Ui) {
    // Search user form.
    ui.horizontal_top(|ui| {
        ui.add_sized(
            [400., 30.],
            egui::TextEdit::singleline(&mut app.user_search)
                .hint_text("enter at least 2 characters and click search"),
        );
        // Search user button.
        let button_label = format!("{} {}", crate::defines::AF_SEARCH_CODE, "search");
        let button = egui::Button::new(button_label);

        if ui.add_sized([150., 30.], button).clicked() {
            app.is_working = true;
            app.get_users_promise = Some(api::user::get_users(
                ctx,
                app.user_search.clone(),
                app.api_url.clone(),
            ));
        }

        if ctx.input(|i| i.key_pressed(Key::Enter)) {
            app.is_working = true;
            app.get_users_promise = Some(api::user::get_users(
                ctx,
                app.user_search.clone(),
                app.api_url.clone(),
            ));
        }
    });

    // User list.
    let scroll_height = ui.available_height() - 50.;

    if app.users.is_some() {
        egui::ScrollArea::vertical()
            .id_source("group_search_user_scroll")
            .max_height(scroll_height)
            .show(ui, |ui| {
                ui.add_space(20.0);

                for user in app.users.as_ref().unwrap() {
                    if ui
                        .link(format!("{} [{}]", user.clone().display, user.clone().id))
                        .clicked()
                    {
                        app.edited_group_add_user = Some(user.id.clone());
                    }
                }
            });
    }

    // Done button.
    let button_label = format!("{} {}", crate::defines::AF_CANCEL_CODE, "done");

    let button = egui::Button::new(button_label);

    if ui.add_sized([150., 30.], button).clicked() {
        app.edit_group_add_user_clicked = false;
    }
}
