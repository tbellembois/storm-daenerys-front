use crate::{
    api::{self, acl::save_acl},
    defines::{AF_SAVE_CODE, AF_SEARCH_CODE},
    ui::daenerys::DaenerysApp,
};
use egui::{Key, Ui};
use storm_daenerys_common::types::acl::{AclEntry, Qualifier, SetAcl};

pub fn render_add_user(app: &mut DaenerysApp, ctx: &egui::Context, ui: &mut Ui) {
    ui.add_space(20.0);

    // Search user form.
    ui.horizontal_top(|ui| {
        ui.add(
            egui::TextEdit::singleline(&mut app.user_search)
                .hint_text("enter at least 2 characters and click search"),
        );

        // Search user button.
        let button_label = format!("{} {}", AF_SEARCH_CODE, "search");
        let button = egui::Button::new(button_label);

        if ui.add_sized([150., 30.], button).clicked() || ctx.input(|i| i.key_pressed(Key::Enter)) {
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
            .id_salt("directory_search_user_scroll")
            .max_height(scroll_height)
            .show(ui, |ui| {
                for user in app.users.as_ref().unwrap().clone() {
                    if ui
                        .link(format!("{} [{}]", user.clone().display, user.clone().id))
                        .clicked()
                    {
                        // Find already exist.
                        let mut found: bool = false;
                        for acl in &app.current_directory.as_ref().unwrap().acls {
                            if let Qualifier::User(_) = acl.qualifier {
                                if acl.qualifier_cn.as_ref().unwrap().eq(&user.id.clone()) {
                                    found = true;
                                }
                            }
                        }

                        if !found {
                            app.current_directory.as_mut().unwrap().acls.push(AclEntry {
                                qualifier: storm_daenerys_common::types::acl::Qualifier::User(0), // FIXME
                                qualifier_cn: Some(user.id.clone()),
                                qualifier_display: Some(user.id.clone()),
                                perm: 7,
                            });

                            app.user_search = "".to_string();
                            app.users = None;
                        }
                    }
                }
            });
    }

    // Done button.
    // let button_label = format!("{} {}", AF_CANCEL_CODE, "done");
    // let button = egui::Button::new(button_label);

    // if ui.add_sized([150., 30.], button).clicked() {
    //     app.active_action = Action::DirectoryEditAcl;
    // }

    // Save button.
    let button_label = format!("{} {}", AF_SAVE_CODE, "save");
    let button = egui::Button::new(button_label);

    if ui.add_sized([150., 30.], button).clicked() {
        let directory_name = app.current_directory.as_ref().unwrap().name.clone();

        app.current_info = Some(format!("saving acl for {}", directory_name));

        let set_acl = SetAcl {
            name: directory_name,
            acls: app.current_directory.as_ref().unwrap().acls.clone(),
        };

        app.is_working = true;
        app.save_directory_acl_promise = Some(save_acl(ctx, set_acl, app.api_url.clone()));
    }
}
