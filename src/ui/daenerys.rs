use std::sync::Once;

use storm_daenerys_common::defines::GROUP_CN_RE_STRING;
use storm_daenerys_common::types::acl::Qualifier;
use storm_daenerys_common::types::group::Group;
use storm_daenerys_common::types::user::User;
use storm_daenerys_common::types::{acl::AclEntry, directory::Directory};

use eframe::CreationContext;
use egui::{FontFamily, FontId, TextStyle, Visuals};
use poll_promise::Promise;

use crate::api;
use crate::error::apperror::AppError;
use crate::ui::pages::main;

use regex::Regex;

static START: Once = Once::new();

// Applications pages.
#[derive(Default)]
enum Page {
    #[default]
    Main,
}

pub struct DaenerysApp {
    // Group name regex.
    pub group_cn_re: Regex,

    // Current active page.
    page: Page,

    // Current theme:
    pub theme: Visuals,

    // Directory list.
    pub directories: Option<Vec<Directory>>,

    // Group list.
    pub groups: Option<Vec<Group>>,

    // User list.
    pub users: Option<Vec<User>>,

    // Promise returned when calling the backend GET /folders endpoint.
    pub get_directories_promise: Option<Promise<Result<Option<Vec<Directory>>, String>>>,
    // Promise returned when calling the backend GET /groups endpoint.
    pub get_groups_promise: Option<Promise<Result<Option<Vec<Group>>, String>>>,
    // Promise returned when calling the backend GET /users endpoint.
    pub get_users_promise: Option<Promise<Result<Option<Vec<User>>, String>>>,

    // Promise returned when calling the backend POST /acls endpoint.
    pub save_directory_acl_promise: Option<Promise<Result<(), String>>>,

    // Promises returned when calling save_group.
    pub save_group_promises: Option<Vec<Promise<Result<(), std::string::String>>>>,
    // Promises returned when calling the backend POST /group endpoint.
    pub create_group_promise: Option<Promise<Result<(), String>>>,
    // Promise return when calling the backend DELETE /groups/:cn endpoint.
    pub delete_group_promise: Option<Promise<Result<(), String>>>,

    // Channels for communication beetween
    // application (GUI) and worker.
    // pub sender: Option<Sender<ToWorker>>,
    // receiver: Option<Receiver<ToApp>>,

    // Current error if one.
    pub current_error: Option<AppError>,
    // Current info if one.
    pub current_info: Option<String>,

    // Icons.
    pub storm_logo: Option<egui_extras::RetainedImage>,
    pub storm_logo_dark: Option<egui_extras::RetainedImage>,
    pub separator_image: Option<egui_extras::RetainedImage>,

    //
    // UI widget states
    //
    // Directory button clicked.
    pub display_directory_button_clicked: Option<Directory>,
    // Group button clicked.
    pub display_group_button_clicked: Option<Group>,

    // Create group.
    pub create_group_clicked: bool,
    // Create directory.
    pub create_directory_clicked: bool,

    // Edit directory clicked.
    pub edit_directory_clicked: Option<Box<Directory>>,
    // Edit group clicked.
    pub edit_group_clicked: Option<Group>,
    // Edit group clicked - backup before edition.
    pub edit_group_clicked_backup: Option<Group>,
    // Add user clicked.
    pub edit_directory_add_user_clicked: bool,
    // Add group clicked.
    pub edit_directory_add_group_clicked: bool,
    // Add user clicked.
    pub edit_group_add_user_clicked: bool,
    // Confirm delete.
    pub edit_group_delete_confirm: bool,

    // Clicking on the delete ACL button: ACL qualifier_cn to remove of the edited directory.
    pub edited_directory_remove_acl: Option<String>,
    // Clicking on the ACL read_only checkbox: ACL qualifier_cn of the read_only to set of the edited directory.
    pub edited_directory_toogle_read_only: Option<(String, bool)>,
    // Clicking on a user (after user search click): user id to add in the edited directory.
    pub edited_directory_add_user: Option<String>,
    // Clicking on a group : group cn to add in the edited directory.
    pub edited_directory_add_group: Option<String>,
    // Clicking on a user (after user search click): user id to add in the edited group.
    pub edited_group_add_user: Option<String>,

    // Clicking on the delete member button: user_cn to remove of the edited group.
    pub edited_group_remove_member: Option<String>,

    // User search input of the add user form.
    pub user_search: String,
    // Groupe name and description input of the create group form.
    pub create_group_name: String,
    pub create_group_description: String,
}

impl Default for DaenerysApp {
    fn default() -> Self {
        Self {
            group_cn_re: Regex::new(GROUP_CN_RE_STRING).unwrap(),
            page: Default::default(),
            theme: Default::default(),
            directories: Default::default(),
            groups: Default::default(),
            users: Default::default(),
            get_directories_promise: Default::default(),
            get_groups_promise: Default::default(),
            get_users_promise: Default::default(),
            save_directory_acl_promise: Default::default(),
            save_group_promises: Default::default(),
            create_group_promise: Default::default(),
            delete_group_promise: Default::default(),
            current_error: Default::default(),
            current_info: Default::default(),
            storm_logo: Default::default(),
            storm_logo_dark: Default::default(),
            separator_image: Default::default(),
            display_directory_button_clicked: Default::default(),
            display_group_button_clicked: Default::default(),
            create_group_clicked: Default::default(),
            create_directory_clicked: Default::default(),
            edit_directory_clicked: Default::default(),
            edit_group_clicked: Default::default(),
            edit_group_clicked_backup: Default::default(),
            edit_directory_add_user_clicked: Default::default(),
            edit_directory_add_group_clicked: Default::default(),
            edit_group_add_user_clicked: Default::default(),
            edit_group_delete_confirm: Default::default(),
            edited_directory_remove_acl: Default::default(),
            edited_directory_toogle_read_only: Default::default(),
            edited_directory_add_user: Default::default(),
            edited_directory_add_group: Default::default(),
            edited_group_add_user: Default::default(),
            edited_group_remove_member: Default::default(),
            user_search: Default::default(),
            create_group_name: Default::default(),
            create_group_description: Default::default(),
        }
    }
}

impl DaenerysApp {
    pub fn new(cc: &CreationContext) -> Self {
        // Create application.
        let mut app = DaenerysApp {
            separator_image: Some(
                egui_extras::RetainedImage::from_svg_bytes(
                    "separator.svg",
                    include_bytes!("media/separator.svg"),
                )
                .unwrap(),
            ),
            storm_logo: Some(
                egui_extras::RetainedImage::from_svg_bytes(
                    "storm.svg",
                    include_bytes!("media/storm.svg"),
                )
                .unwrap(),
            ),
            storm_logo_dark: Some(
                egui_extras::RetainedImage::from_svg_bytes(
                    "storm-dark.svg",
                    include_bytes!("media/storm-dark.svg"),
                )
                .unwrap(),
            ),
            group_cn_re: Regex::new(GROUP_CN_RE_STRING).unwrap(),
            ..Default::default()
        };

        // Create channels.
        // let (app_tx, app_rx) = mpsc::channel();
        // let (worker_tx, worker_rx) = mpsc::channel();

        // let context = cc.egui_ctx.clone();

        //info!("Spawning new worker.");

        // Spawn a thread with a new worker.
        // thread::spawn(move || {
        //     Worker::new(worker_tx, app_rx, context).init();
        // });

        // info!("New worker spawned.");

        // app.sender = Some(app_tx);
        // app.receiver = Some(worker_rx);

        // Set custom fonts and styles.
        setup_custom_fonts(&cc.egui_ctx);
        setup_custom_styles(&cc.egui_ctx);

        // Set default theme.
        app.theme = Visuals::light();
        cc.egui_ctx.set_visuals(Visuals::light());

        app
    }
}

impl eframe::App for DaenerysApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // Check for ToApp messages.
        // if let Some(receiver) = &self.receiver {
        //     let maybe_message = match receiver.try_recv() {
        //         Ok(message) => Some(message),
        //         Err(e) => match e {
        //             mpsc::TryRecvError::Empty => None,
        //             mpsc::TryRecvError::Disconnected => {
        //                 self.current_error = Some(AppError::ChannelClosed);
        //                 None
        //             }
        //         },
        //     };

        //     if let Some(message) = maybe_message {
        //         println!("received = {:?}", message);
        //         match message.message {
        //             ToAppMessage::Pong => self.current_info = Some("pong".to_string()),
        //             ToAppMessage::Error(e) => self.current_error = Some(e), //FIXME: handle fatal errors
        //         }
        //     }
        // }

        // Get directories promises.
        if let Some(p) = &self.get_directories_promise {
            println!("get_directories_promise");

            match p.ready() {
                None => (),
                Some(try_directories) => {
                    match try_directories {
                        Ok(directories) => {
                            self.directories = directories.clone();

                            self.display_directory_button_clicked = None;
                            self.get_directories_promise = None;
                        }
                        Err(e) => self.current_error = Some(AppError::InternalError(e.to_string())),
                    };
                }
            }
        }

        // Save acl promise.
        if let Some(p) = &self.save_directory_acl_promise {
            println!("save_acl_promise");

            match p.ready() {
                None => (),
                Some(try_result) => {
                    match try_result {
                        Ok(_) => {
                            self.current_info = Some("acl set successfully".to_string());
                            self.save_directory_acl_promise = None;

                            self.get_directories_promise =
                                Some(api::directory::get_root_directories(ctx));
                        }
                        Err(e) => {
                            self.current_error = Some(AppError::InternalError(e.to_string()));
                            self.current_info = None;
                        }
                    };
                }
            }
        }

        // Save group promise.
        if let Some(p) = &self.save_group_promises {
            let mut count = 0;
            let total_promises = p.len();
            let mut is_error: bool = false;

            for promise in p.iter() {
                match promise.ready() {
                    None => (),
                    Some(try_result) => {
                        match try_result {
                            Ok(_) => {
                                count += 1;
                            }
                            Err(e) => {
                                self.current_error = Some(AppError::InternalError(e.to_string()));
                                self.current_info = None;

                                is_error = true;
                                break;
                            }
                        };
                    }
                }
            }

            if is_error || count == total_promises {
                self.save_group_promises = None;
            }

            if count == total_promises {
                self.current_info = Some("group updated successfully".to_string());

                self.get_groups_promise = Some(api::group::get_groups(ctx));
            }
        }

        // Create group promise.
        if let Some(p) = &self.create_group_promise {
            println!("create_group_promise");

            match p.ready() {
                None => (),
                Some(try_result) => {
                    match try_result {
                        Ok(_) => {
                            self.current_info = Some("group created successfully".to_string());
                            self.create_group_promise = None;

                            self.get_groups_promise = Some(api::group::get_groups(ctx));
                        }
                        Err(e) => {
                            self.current_error = Some(AppError::InternalError(e.to_string()));
                            self.current_info = None;
                        }
                    };
                }
            }
        }

        // Delete group promise.
        if let Some(p) = &self.delete_group_promise {
            println!("delete_group_promise");

            match p.ready() {
                None => (),
                Some(try_result) => {
                    match try_result {
                        Ok(_) => {
                            self.current_info = Some("group deleted successfully".to_string());
                            self.delete_group_promise = None;

                            self.get_groups_promise = Some(api::group::get_groups(ctx));
                        }
                        Err(e) => {
                            self.current_error = Some(AppError::InternalError(e.to_string()));
                            self.current_info = None;
                        }
                    };
                }
            }
        }

        // Get groups promises.
        if let Some(p) = &self.get_groups_promise {
            println!("get_groups_promise");

            match p.ready() {
                None => (),
                Some(try_groups) => {
                    match try_groups {
                        Ok(groups) => {
                            self.groups = groups.clone();

                            self.display_group_button_clicked = None;
                            self.get_groups_promise = None;
                        }
                        Err(e) => self.current_error = Some(AppError::InternalError(e.to_string())),
                    };
                }
            }
        }

        // Get users promise.
        if let Some(p) = &self.get_users_promise {
            println!("get_users_promise");

            match p.ready() {
                None => (),
                Some(try_users) => {
                    match try_users {
                        Ok(users) => {
                            self.users = users.clone();

                            self.get_users_promise = None;
                        }
                        Err(e) => self.current_error = Some(AppError::InternalError(e.to_string())),
                    };
                }
            }
        }

        // Check directory remove user or group (acl).
        if let Some(edited_directory_remove_acl) = &self.edited_directory_remove_acl {
            self.edit_directory_clicked
                .as_mut()
                .unwrap()
                .acls
                .retain(|a| match a.qualifier_cn.clone() {
                    Some(qualidier_cn) => qualidier_cn.ne(edited_directory_remove_acl),
                    None => true, // non User(u) or Group(g) acl
                });

            self.edited_directory_remove_acl = None;
        }

        // Check directory add user.
        if let Some(edited_directory_add_user) = &self.edited_directory_add_user {
            // Find already exist.
            let mut found: bool = false;
            for acl in &self.edit_directory_clicked.as_ref().unwrap().acls {
                if let Qualifier::User(_) = acl.qualifier {
                    if acl
                        .qualifier_cn
                        .as_ref()
                        .unwrap()
                        .eq(edited_directory_add_user)
                    {
                        found = true;
                    }
                }
            }

            if !found {
                self.edit_directory_clicked
                    .as_mut()
                    .unwrap()
                    .acls
                    .push(AclEntry {
                        qualifier: storm_daenerys_common::types::acl::Qualifier::User(0), // FIXME
                        qualifier_cn: Some(edited_directory_add_user.to_string()),
                        perm: 7,
                    });

                self.user_search = "".to_string();
                self.users = None;
            }

            self.edited_directory_add_user = None;
        }

        // Check directory add group.
        if let Some(edited_directory_add_group) = &self.edited_directory_add_group {
            // Find already exist.
            let mut found: bool = false;
            for acl in &self.edit_directory_clicked.as_ref().unwrap().acls {
                if let Qualifier::Group(_) = acl.qualifier {
                    if acl
                        .qualifier_cn
                        .as_ref()
                        .unwrap()
                        .eq(edited_directory_add_group)
                    {
                        found = true;
                    }
                }
            }

            if !found {
                self.edit_directory_clicked
                    .as_mut()
                    .unwrap()
                    .acls
                    .push(AclEntry {
                        qualifier: storm_daenerys_common::types::acl::Qualifier::Group(0), // FIXME
                        qualifier_cn: Some(edited_directory_add_group.to_string()),
                        perm: 7,
                    });
            }

            self.edited_directory_add_group = None;
        }

        // Check directory acl read_only change.
        if let Some(edited_directory_toogle_read_only) = &self.edited_directory_toogle_read_only {
            if self.edit_directory_clicked.is_some() {
                let (qualifier_cn, read_only) = edited_directory_toogle_read_only;

                for acl in self
                    .edit_directory_clicked
                    .as_mut()
                    .unwrap()
                    .acls
                    .iter_mut()
                {
                    // FIXME
                    // Keep only necessary acls.
                    match acl.qualifier {
                        Qualifier::User(_) => (),
                        Qualifier::Group(_) => (),
                        _ => continue,
                    }

                    if acl.qualifier_cn.as_ref().unwrap().eq(qualifier_cn) {
                        if *read_only {
                            acl.perm = 5;
                        } else {
                            acl.perm = 7;
                        }
                    }
                }
            };
        }

        // Check group add user.
        if let Some(edited_group_add_user) = &self.edited_group_add_user {
            // Find already exist.
            let mut found: bool = false;

            if self.edit_group_clicked.as_ref().unwrap().member.is_some() {
                for m in self
                    .edit_group_clicked
                    .as_ref()
                    .unwrap()
                    .member
                    .as_ref()
                    .unwrap()
                {
                    if m.eq(edited_group_add_user) {
                        found = true;
                    }
                }
            } else {
                self.edit_group_clicked.as_mut().unwrap().member = Some(Vec::new());
            }

            if !found {
                self.edit_group_clicked
                    .as_mut()
                    .unwrap()
                    .member
                    .as_mut()
                    .unwrap()
                    .push(edited_group_add_user.to_string());

                self.user_search = "".to_string();
                self.users = None;
            }

            self.edited_group_add_user = None;
        }

        // Check group remove user.
        if let Some(edited_group_remove_member) = &self.edited_group_remove_member {
            if self.edit_group_clicked.as_ref().unwrap().member.is_some() {
                self.edit_group_clicked
                    .as_mut()
                    .unwrap()
                    .member
                    .as_mut()
                    .unwrap()
                    .retain(|u| u.ne(edited_group_remove_member));
            }

            self.edited_group_remove_member = None;
        }

        // Render page.
        match self.page {
            Page::Main => main::ui::update(self, ctx, frame),
        }

        // Get initial directory and group list.
        START.call_once(|| {
            self.get_directories_promise = Some(api::directory::get_root_directories(ctx));
            self.get_groups_promise = Some(api::group::get_groups(ctx));
        });
    }
}

fn setup_custom_fonts(ctx: &egui::Context) {
    // Start with the default fonts (we will be adding to them rather than replacing them).
    let mut fonts = egui::FontDefinitions::default();

    // Install custom fonts.
    // .ttf and .otf files supported.
    fonts.font_data.insert(
        "B612-Bold".to_owned(),
        egui::FontData::from_static(include_bytes!("fonts/B612-Bold.ttf")),
    );
    fonts.font_data.insert(
        "B612-BoldItalic".to_owned(),
        egui::FontData::from_static(include_bytes!("fonts/B612-BoldItalic.ttf")),
    );
    fonts.font_data.insert(
        "B612-Italic".to_owned(),
        egui::FontData::from_static(include_bytes!("fonts/B612-Italic.ttf")),
    );
    fonts.font_data.insert(
        "B612-Regular".to_owned(),
        egui::FontData::from_static(include_bytes!("fonts/B612-Regular.ttf")),
    );

    fonts.font_data.insert(
        "Font-Awesome-6-Brands-Regular-400".to_owned(),
        egui::FontData::from_static(include_bytes!(
            "fonts/Font-Awesome-6-Brands-Regular-400.otf"
        )),
    );
    fonts.font_data.insert(
        "Font-Awesome-6-Free-Regular-400".to_owned(),
        egui::FontData::from_static(include_bytes!("fonts/Font-Awesome-6-Free-Regular-400.otf")),
    );
    fonts.font_data.insert(
        "Font-Awesome-6-Free-Solid-900".to_owned(),
        egui::FontData::from_static(include_bytes!("fonts/Font-Awesome-6-Free-Solid-900.otf")),
    );

    fonts.font_data.insert(
        "LiberationSans-Regular".to_owned(),
        egui::FontData::from_static(include_bytes!("fonts/LiberationSans-Regular.ttf")),
    );
    fonts.font_data.insert(
        "LiberationSans-Bold".to_owned(),
        egui::FontData::from_static(include_bytes!("fonts/LiberationSans-Bold.ttf")),
    );
    // Put my font first (highest priority) for proportional text:
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "B612-Regular".to_owned());
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(1, "B612-Bold".to_owned());
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(2, "B612-BoldItalic".to_owned());
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(3, "B612-Italic".to_owned());

    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(4, "Font-Awesome-6-Brands-Regular-400".to_owned());
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(5, "Font-Awesome-6-Free-Regular-400".to_owned());
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(6, "Font-Awesome-6-Free-Solid-900".to_owned());

    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(7, "LiberationSans-Regular".to_owned());
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(8, "LiberationSans-Bold".to_owned());

    // Put my font as last fallback for monospace:
    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push("B612-Regular".to_owned());

    // Tell egui to use these fonts:
    ctx.set_fonts(fonts);
}

fn setup_custom_styles(ctx: &egui::Context) {
    use FontFamily::Proportional;

    let mut style = (*ctx.style()).clone();
    style.text_styles = [
        (TextStyle::Heading, FontId::new(18.0, Proportional)),
        (TextStyle::Body, FontId::new(14.0, Proportional)),
        (TextStyle::Button, FontId::new(14.0, Proportional)),
    ]
    .into();
    ctx.set_style(style);
}
