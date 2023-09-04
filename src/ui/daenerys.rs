use std::collections::HashMap;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::Once;
use std::thread;

use storm_daenerys_common::types::acl::Qualifier;
use storm_daenerys_common::types::group::Group;
use storm_daenerys_common::types::user::User;
use storm_daenerys_common::types::{acl::AclEntry, directory::Directory};

use eframe::CreationContext;
use egui::{FontFamily, FontId, TextStyle, Visuals};
use poll_promise::Promise;
use tracing::debug;

use crate::api;
use crate::error::apperror::AppError;
use crate::ui::pages::main;
use crate::worker::{
    message::{ToApp, ToAppMessage, ToWorker},
    worker::Worker,
};

static START: Once = Once::new();

// Applications pages.
#[derive(Default)]
enum Page {
    #[default]
    Main,
}

#[derive(Default)]
pub struct DaenerysApp {
    // Current active page.
    page: Page,

    // Current theme:
    pub theme: Visuals,

    // Directory list.
    pub directories: Option<Vec<Directory>>,
    // The same list as a map.
    // pub directories_map: HashMap<String, Vec<AclEntry>>,

    // Group list.
    pub groups: Option<Vec<Group>>,
    // The same list as a map.
    // pub groups_map: HashMap<String, Group>,

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

    // Channels for communication beetween
    // application (GUI) and worker.
    pub sender: Option<Sender<ToWorker>>,
    receiver: Option<Receiver<ToApp>>,

    // Current error if one.
    pub current_error: Option<AppError>,
    // Current info if one.
    pub current_info: Option<String>,

    // Icons.
    pub storm_logo: Option<egui_extras::RetainedImage>,

    //
    // UI widget states
    //
    // Directory button clicked.
    pub display_directory_button_clicked: Option<Directory>,
    // Group button clicked.
    pub display_group_button_clicked: Option<Group>,

    // Edit directory clicked.
    pub edit_directory_clicked: Option<Box<Directory>>,
    // Edit group clicked.
    pub edit_group_clicked: Option<Group>,
    // Add user clicked.
    pub edit_directory_add_user_clicked: bool,
    // Add group clicked.
    pub edit_directory_add_group_clicked: bool,
    // Add user clicked.
    pub edit_group_add_user_clicked: bool,

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
}

impl DaenerysApp {
    pub fn new(cc: &CreationContext) -> Self {
        // Create application.
        let mut app = DaenerysApp {
            storm_logo: Some(
                egui_extras::RetainedImage::from_svg_bytes(
                    "storm.svg",
                    include_bytes!("media/storm.svg"),
                )
                .unwrap(),
            ),
            ..Default::default()
        };

        // Create channels.
        let (app_tx, app_rx) = mpsc::channel();
        let (worker_tx, worker_rx) = mpsc::channel();

        let context = cc.egui_ctx.clone();

        tracing::info!("Spawning new worker.");

        // Spawn a thread with a new worker.
        thread::spawn(move || {
            Worker::new(worker_tx, app_rx, context).init();
        });

        tracing::info!("New worker spawned.");

        app.sender = Some(app_tx);
        app.receiver = Some(worker_rx);

        // Set custom fonts and styles.
        setup_custom_fonts(&cc.egui_ctx);
        setup_custom_styles(&cc.egui_ctx);

        // Set default theme.
        app.theme = Visuals::dark();

        app
    }
}

impl eframe::App for DaenerysApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // Check for ToApp messages.
        if let Some(receiver) = &self.receiver {
            let maybe_message = match receiver.try_recv() {
                Ok(message) => Some(message),
                Err(e) => match e {
                    mpsc::TryRecvError::Empty => None,
                    mpsc::TryRecvError::Disconnected => {
                        self.current_error = Some(AppError::ChannelClosed);
                        None
                    }
                },
            };

            if let Some(message) = maybe_message {
                println!("received = {:?}", message);
                match message.message {
                    ToAppMessage::Pong => self.current_info = Some("pong".to_string()),
                    ToAppMessage::Error(e) => self.current_error = Some(e), //FIXME: handle fatal errors
                }
            }
        }

        // Get directories promises.
        if let Some(p) = &self.get_directories_promise {
            println!("get_directories_promise");

            match p.ready() {
                None => (),
                Some(try_directories) => {
                    match try_directories {
                        Ok(directories) => {
                            self.directories = directories.clone();
                            // self.directories_map = self
                            //     .directories
                            //     .as_ref()
                            //     .unwrap()
                            //     .iter()
                            //     .map(|d| (d.name.to_owned(), d.acls.to_owned()))
                            //     .collect();
                            self.display_directory_button_clicked = None;

                            // tracing::debug!("directories_map: {:?}", self.directories_map);

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
                            // self.groups_map = self
                            //     .groups
                            //     .as_ref()
                            //     .unwrap()
                            //     .iter()
                            //     .map(|g| (g.cn.to_owned(), g.to_owned()))
                            //     .collect();
                            self.display_group_button_clicked = None;

                            self.get_groups_promise = None;
                        }
                        Err(e) => self.current_error = Some(AppError::InternalError(e.to_string())),
                    };
                }
            }
        }

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
        .insert(3, "Font-Awesome-6-Brands-Regular-400".to_owned());
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(3, "Font-Awesome-6-Free-Regular-400".to_owned());
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(3, "Font-Awesome-6-Free-Solid-900".to_owned());

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
        (TextStyle::Heading, FontId::new(25.0, Proportional)),
        (TextStyle::Body, FontId::new(20.0, Proportional)),
        (TextStyle::Button, FontId::new(22.0, Proportional)),
    ]
    .into();
    ctx.set_style(style);
}
