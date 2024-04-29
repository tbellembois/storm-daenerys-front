use crate::{
    defines::AF_CANCEL_CODE,
    ui::daenerys::{Action, DaenerysApp},
};
use egui::Ui;
use storm_daenerys_common::types::acl::{AclEntry, Qualifier};

pub fn render_add_group(app: &mut DaenerysApp, ui: &mut Ui) {
    // Group list.
    if app.groups.is_some() {
        for group in app.groups.as_ref().unwrap() {
            if ui.link(group.clone().cn).clicked() {
                // Find already exist.
                let mut found: bool = false;
                for acl in &app.active_directory.as_ref().unwrap().acls {
                    if let Qualifier::Group(_) = acl.qualifier {
                        if acl.qualifier_cn.as_ref().unwrap().eq(&group.cn.clone()) {
                            found = true;
                        }
                    }
                }

                if !found {
                    app.active_directory.as_mut().unwrap().acls.push(AclEntry {
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
    let button_label = format!("{} {}", AF_CANCEL_CODE, "done");
    let button = egui::Button::new(button_label);

    if ui.add_sized([150., 30.], button).clicked() {
        app.active_action = Action::DirectoryEditAcl;
    }
}
