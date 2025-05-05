use crate::{api::acl::save_acl, defines::AF_SAVE_CODE, ui::daenerys::DaenerysApp};
use egui::Ui;
use storm_daenerys_common::types::acl::{AclEntry, Qualifier, SetAcl};

pub fn render_add_group(app: &mut DaenerysApp, ctx: &egui::Context, ui: &mut Ui) {
    // Group list.
    if app.groups.is_some() {
        for group in app.groups.as_ref().unwrap() {
            if ui.link(group.clone().cn).clicked() {
                // Find already exist.
                let mut found: bool = false;
                for acl in &app.current_directory.as_ref().unwrap().acls {
                    if let Qualifier::Group(_) = acl.qualifier {
                        if acl.qualifier_cn.as_ref().unwrap().eq(&group.cn.clone()) {
                            found = true;
                        }
                    }
                }

                if !found {
                    app.current_directory.as_mut().unwrap().acls.push(AclEntry {
                        qualifier: storm_daenerys_common::types::acl::Qualifier::Group(0), // FIXME
                        qualifier_cn: Some(group.cn.clone().to_string()),
                        qualifier_display: Some(group.cn.clone().to_string()),
                        perm: 7,
                    });
                }
            }
        }
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
