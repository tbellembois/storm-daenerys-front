use super::state::{ApplicationState, Page};
use crate::api;
use crate::error::apperror::AppError;
use crate::ui::pages::main;
// use crate::worker::builder::Worker;
// use crate::worker::message::{ToApp, ToWorker};
use eframe::{egui, CreationContext};
use egui::Vec2;
use egui_aesthetix::themes::{CarlDark, StandardDark, StandardLight};
use egui_aesthetix::{self, Aesthetix};
use poll_promise::Promise;
use regex::Regex;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::Display;
use std::rc::Rc;
// use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::Once;
// use std::thread;
use storm_daenerys_common::defines::{
    DIRECTORY_NAME_RE_STRING, GROUP_CN_RE_STRING, QUOTA_FORMAT_RE_STRING,
};
use storm_daenerys_common::types::acl::Qualifier;
use storm_daenerys_common::types::config::Config;
use storm_daenerys_common::types::directory::Quota;
use storm_daenerys_common::types::group::Group;
use storm_daenerys_common::types::quota::QuotaUnit;
use storm_daenerys_common::types::user::User;
use storm_daenerys_common::types::{acl::AclEntry, directory::Directory};

static START: Once = Once::new();

#[derive(PartialEq)]
pub enum Action {
    Home,
    DiskUsage,
    DirectoryEdit,
    DirectoryCreate,
    DirectoryEditAcl,
    DirectoryEditQuota,
    DirectoryEditAclAddUser,
    DirectoryEditAclAddGroup,
    GroupEdit,
    GroupCreate,
    GroupEditDeleteConfirm,
    GroupEditAddUser,
    GroupEditUsers,
}

impl Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Action::Home => write!(f, "home"),
            Action::DirectoryCreate => write!(f, "directory_create"),
            Action::DirectoryEdit => write!(f, "directory_edit"),
            Action::DirectoryEditAcl => write!(f, "directory_edit_acl"),
            Action::DirectoryEditQuota => write!(f, "directory_edit_quota"),
            Action::DirectoryEditAclAddUser => write!(f, "directory_edit_acl_add_user"),
            Action::DirectoryEditAclAddGroup => write!(f, "directory_edit_acl_add_group"),
            Action::GroupEdit => write!(f, "group_edit"),
            Action::GroupCreate => write!(f, "group_create"),
            Action::GroupEditDeleteConfirm => write!(f, "group_edit_delete_confirm"),
            Action::GroupEditAddUser => write!(f, "group_edit_add_user"),
            Action::GroupEditUsers => write!(f, "group_edit_users"),
            Action::DiskUsage => write!(f, "disk_usage"),
        }
    }
}

pub struct DaenerysApp {
    // Application state.
    pub state: ApplicationState,
    // Action in progress.
    pub active_action: Action,
    // Holds the supported themes that the user can switch between.
    pub themes: Vec<Rc<dyn Aesthetix>>,
    // Application version.
    pub app_version: String,
    // API URL.
    pub api_url: String,
    // Group name regex.
    pub group_cn_re: Regex,
    // Directory name regex.
    pub directory_name_re: Regex,
    // Quota format regex.
    pub quota_format_re: Regex,
    // Central panel available size.
    pub central_panel_available_size: Vec2,
    // Disk usage.
    pub du: Option<String>,
    // Quota.
    pub quota: Option<Quota>,
    // Admin of the STORM space.
    pub admin: Option<String>,
    // Connected user.
    pub connected_user: Option<String>,
    // Admin restriction of the connected user.
    pub current_admin_restriction: Option<String>,
    // Group prefix.
    pub group_prefix: Option<String>,
    // Root groups.
    pub root_groups: Option<Vec<String>>,
    // Directory list.
    pub directories: Option<Vec<Directory>>,
    // Group list.
    pub groups: Option<Vec<Group>>,
    // User list.
    pub users: Option<Vec<User>>,

    // Promise returned when calling the backend GET /du endpoint.
    pub get_du_promise: Option<Promise<Result<Option<String>, String>>>,
    // Promise returned when calling the backend GET /config endpoint.
    pub get_config_prefix_promise: Option<Promise<Result<Config, String>>>,
    // Promise returned when calling the backend GET /folders endpoint.
    pub get_directories_promise: Option<Promise<Result<Option<Vec<Directory>>, String>>>,
    // Promise returned when calling the backend GET /groups endpoint.
    pub get_groups_promise: Option<Promise<Result<Option<Vec<Group>>, String>>>,
    // Promise returned when calling the backend GET /users endpoint.
    pub get_users_promise: Option<Promise<Result<Option<Vec<User>>, String>>>,
    // Promises returned when calling the backend POST /directories endpoint.
    pub create_directory_promise: Option<Promise<Result<(), String>>>,
    // Promise returned when calling the backend POST /acls endpoint.
    pub save_directory_acl_promise: Option<Promise<Result<(), String>>>,
    // Promise returned when calling the backend POST /quota endpoint.
    pub save_directory_quota_promise: Option<Promise<Result<(), String>>>,
    // Promises returned when calling the backend GET /userdisplay endpoint.
    pub get_user_display_promises: HashMap<String, Option<Promise<Result<Option<String>, String>>>>,
    // Promises returned when calling save_group.
    pub save_group_promises: Option<Vec<Promise<Result<(), std::string::String>>>>,
    // Promise returned when calling the backend POST /group endpoint.
    pub create_group_promise: Option<Promise<Result<(), String>>>,
    // Promise return when calling the backend DELETE /groups/:cn endpoint.
    pub delete_group_promise: Option<Promise<Result<(), String>>>,

    // User display name cache.
    pub user_display_cache: HashMap<String, Option<String>>,

    // Channels for communication beetween
    // application (GUI) and worker.
    // pub sender: Option<Sender<ToWorker>>,
    // receiver: Option<Receiver<ToApp>>,

    // Current error if one.
    pub current_error: Option<AppError>,
    // Current info if one.
    pub current_info: Option<String>,

    // Active directory been showned/edited.
    pub active_directory: Option<Box<Directory>>,
    // Active group been showned/edited.
    pub active_group: Option<Box<Group>>,

    // Edit group clicked - backup before edition.
    pub edit_group_clicked_backup: Option<Box<Group>>,

    // Directory name input of the create directory form.
    pub create_directory_name: String,
    // Directory quota.
    pub edited_directory_quota: String,
    // Directory quota unit.
    pub edited_directory_quota_unit: QuotaUnit,
    // User search input of the add user form.
    pub user_search: String,
    // Groupe name and description input of the create group form.
    pub create_group_name: String,
    pub create_group_description: String,

    // Spinner? shown on API calls.
    pub is_working: bool,
    // Show/hide directory list.
    pub show_directory_list: bool,
    // Show/hide group list.
    pub show_group_list: bool,
}

impl Default for DaenerysApp {
    fn default() -> Self {
        Self {
            app_version: Default::default(),
            is_working: Default::default(),
            group_cn_re: Regex::new(GROUP_CN_RE_STRING).unwrap(),
            directory_name_re: Regex::new(DIRECTORY_NAME_RE_STRING).unwrap(),
            quota_format_re: Regex::new(QUOTA_FORMAT_RE_STRING).unwrap(),
            directories: Default::default(),
            groups: Default::default(),
            root_groups: Default::default(),
            users: Default::default(),
            get_directories_promise: Default::default(),
            get_groups_promise: Default::default(),
            get_users_promise: Default::default(),
            get_config_prefix_promise: Default::default(),
            save_directory_acl_promise: Default::default(),
            save_directory_quota_promise: Default::default(),
            save_group_promises: Default::default(),
            create_group_promise: Default::default(),
            create_directory_promise: Default::default(),
            delete_group_promise: Default::default(),
            current_error: Default::default(),
            current_info: Default::default(),
            edit_group_clicked_backup: Default::default(),
            user_search: Default::default(),
            create_group_name: Default::default(),
            create_group_description: Default::default(),
            create_directory_name: Default::default(),
            active_directory: Default::default(),
            active_group: Default::default(),
            admin: Default::default(),
            current_admin_restriction: Default::default(),
            api_url: "http://localhost:3000".to_string(),
            get_du_promise: Default::default(),
            du: Default::default(),
            quota: Default::default(),
            central_panel_available_size: Default::default(),
            group_prefix: Default::default(),
            show_directory_list: true,
            show_group_list: true,
            get_user_display_promises: HashMap::new(),
            user_display_cache: HashMap::new(),
            connected_user: Default::default(),
            edited_directory_quota: Default::default(),
            edited_directory_quota_unit: QuotaUnit::Megabyte,
            state: Default::default(),
            themes: Default::default(),
            // sender: Default::default(),
            // receiver: Default::default(),
            active_action: Action::Home,
        }
    }
}

impl DaenerysApp {
    pub fn new(cc: &CreationContext, api_url: String, app_version: String) -> Self {
        // Create channels.
        // let (app_tx, app_rx) = mpsc::channel();
        // let (worker_tx, worker_rx) = mpsc::channel();

        // dbg!("Spawning new worker.");

        // Spawn a thread with a new worker.
        // let context = cc.egui_ctx.clone();
        // thread::spawn(move || {
        //     Worker::new(worker_tx, app_rx, context).init();
        // });

        // dbg!("New worker spawned.");

        // Load custom fonts and styles.
        setup_custom_fonts(&cc.egui_ctx);

        // Load themes.
        let themes: Vec<Rc<dyn Aesthetix>> = vec![
            Rc::new(StandardDark),
            Rc::new(StandardLight),
            Rc::new(CarlDark),
        ];
        let active_theme: Rc<dyn Aesthetix> = match themes.first() {
            Some(theme) => theme.clone(),
            None => panic!("The first theme in the list of available themes could not be loaded."),
        };

        // Create application state.
        let state = ApplicationState::new(active_theme);

        // Initialize the custom theme/styles for egui.
        cc.egui_ctx.set_style(state.active_theme.custom_style());

        // Create application.
        DaenerysApp {
            group_cn_re: Regex::new(GROUP_CN_RE_STRING).unwrap(),
            app_version,
            api_url,
            state,
            themes,
            // sender: Some(app_tx),
            // receiver: Some(worker_rx),
            ..Default::default()
        }
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

        // Get user display promises.
        let mut user_display_promises_done: Vec<String> = vec![];
        for (username, maybe_p) in self.get_user_display_promises.iter() {
            if let Some(p) = maybe_p {
                if let Some(try_display) = p.ready() {
                    match try_display {
                        Ok(display) => {
                            self.user_display_cache
                                .insert(username.to_string(), display.clone());
                            user_display_promises_done.push(username.to_string());
                        }
                        Err(e) => self.current_error = Some(AppError::InternalError(e.to_string())),
                    }
                }
            }
        }

        for username in user_display_promises_done.iter() {
            self.get_user_display_promises.remove(username);
        }

        // Get du promise.
        if let Some(p) = &self.get_du_promise {
            match p.ready() {
                None => (),
                Some(try_du) => {
                    self.is_working = false;

                    match try_du {
                        Ok(du) => {
                            self.du = du.clone();

                            self.active_action = Action::DiskUsage;
                            self.get_du_promise = None;
                        }
                        Err(e) => self.current_error = Some(AppError::InternalError(e.to_string())),
                    };
                }
            }
        }

        // Get config promise.
        if let Some(p) = &self.get_config_prefix_promise {
            match p.ready() {
                None => (),
                Some(try_config) => {
                    self.is_working = false;

                    match try_config {
                        Ok(config) => {
                            self.admin = Some(config.admin.clone());
                            self.connected_user = Some(config.connected_user.clone());
                            self.current_admin_restriction =
                                config.current_admin_restriction.clone();
                            self.group_prefix = Some(config.users_dsi_api_group_prefix.clone());
                            self.root_groups = config.root_groups.clone();
                            self.quota = Some(config.quota.clone());

                            self.get_config_prefix_promise = None;

                            self.get_directories_promise = Some(
                                api::directory::get_root_directories(ctx, self.api_url.clone()),
                            );
                            self.get_groups_promise =
                                Some(api::group::get_groups(ctx, self.api_url.clone()));
                        }
                        Err(e) => self.current_error = Some(AppError::InternalError(e.to_string())),
                    };
                }
            }
        }

        // Get directories promises.
        if let Some(p) = &self.get_directories_promise {
            match p.ready() {
                None => (),
                Some(try_directories) => {
                    self.is_working = false;

                    match try_directories {
                        Ok(directories) => {
                            self.directories = directories.clone();

                            if self.directories.is_some() {
                                // Filter directory ACLs.
                                for directory in self.directories.as_mut().unwrap() {
                                    directory.acls.retain(|acl| {
                                        if acl.perm == 0 {
                                            false
                                        } else {
                                            matches!(
                                                acl.qualifier,
                                                Qualifier::User(_) | Qualifier::Group(_)
                                            )
                                        }
                                    });
                                }

                                // Get display name for each user of the ACLs.
                                for directory in self.directories.as_mut().unwrap() {
                                    for acl in directory.acls.iter_mut() {
                                        let qualifier_cn = acl.qualifier_cn.clone().unwrap();

                                        acl.qualifier_display = match self
                                            .user_display_cache
                                            .get(&qualifier_cn)
                                        {
                                            Some(maybe_display_name) => match maybe_display_name {
                                                Some(display_name) => {
                                                    Some(display_name.to_string())
                                                }
                                                None => Some(format!(
                                                    "<invalid account> ({})",
                                                    &qualifier_cn
                                                )),
                                            },
                                            None => {
                                                if !self
                                                    .get_user_display_promises
                                                    .contains_key(&qualifier_cn)
                                                {
                                                    self.get_user_display_promises.insert(
                                                        qualifier_cn.clone(),
                                                        Some(api::user::get_user_display(
                                                            ctx,
                                                            qualifier_cn.clone(),
                                                            self.api_url.clone(),
                                                        )),
                                                    );
                                                }

                                                Some(qualifier_cn)
                                            }
                                        }
                                    }
                                }

                                // Sort directories.
                                self.directories.as_mut().unwrap().sort();
                            }

                            self.get_directories_promise = None;
                        }
                        Err(e) => self.current_error = Some(AppError::InternalError(e.to_string())),
                    };
                }
            }
        }

        // Save acl promise.
        if let Some(p) = &self.save_directory_acl_promise {
            match p.ready() {
                None => (),
                Some(try_result) => {
                    self.is_working = false;

                    match try_result {
                        Ok(_) => {
                            self.current_info = Some("acl set successfully".to_string());
                            self.save_directory_acl_promise = None;

                            self.get_directories_promise = Some(
                                api::directory::get_root_directories(ctx, self.api_url.clone()),
                            );
                        }
                        Err(e) => {
                            self.current_error = Some(AppError::InternalError(e.to_string()));
                            self.current_info = None;
                        }
                    };
                }
            }
        }

        // Save quota promise.
        if let Some(p) = &self.save_directory_quota_promise {
            match p.ready() {
                None => (),
                Some(try_result) => {
                    self.is_working = false;

                    match try_result {
                        Ok(_) => {
                            self.current_info = Some("quota set successfully".to_string());
                            self.save_directory_quota_promise = None;

                            self.get_directories_promise = Some(
                                api::directory::get_root_directories(ctx, self.api_url.clone()),
                            );
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
                self.is_working = false;

                self.current_info = Some("group updated successfully".to_string());

                self.get_groups_promise = Some(api::group::get_groups(ctx, self.api_url.clone()));
            }
        }

        // Create directory promise.
        if let Some(p) = &self.create_directory_promise {
            match p.ready() {
                None => (),
                Some(try_result) => {
                    self.is_working = false;

                    match try_result {
                        Ok(_) => {
                            self.current_info = Some("directory created successfully".to_string());
                            self.create_directory_promise = None;

                            self.get_directories_promise = Some(
                                api::directory::get_root_directories(ctx, self.api_url.clone()),
                            );
                        }
                        Err(e) => {
                            self.current_error = Some(AppError::InternalError(e.to_string()));
                            self.current_info = None;
                        }
                    };
                }
            }
        }

        // Create group promise.
        if let Some(p) = &self.create_group_promise {
            match p.ready() {
                None => (),
                Some(try_result) => {
                    self.is_working = false;

                    match try_result {
                        Ok(_) => {
                            self.current_info = Some("group created successfully".to_string());
                            self.create_group_promise = None;

                            self.get_groups_promise =
                                Some(api::group::get_groups(ctx, self.api_url.clone()));
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
            match p.ready() {
                None => (),
                Some(try_result) => {
                    self.is_working = false;

                    match try_result {
                        Ok(_) => {
                            self.current_info = Some("group deleted successfully".to_string());
                            self.delete_group_promise = None;

                            self.get_groups_promise =
                                Some(api::group::get_groups(ctx, self.api_url.clone()));
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
            match p.ready() {
                None => (),
                Some(try_groups) => {
                    self.is_working = false;

                    match try_groups {
                        Ok(groups) => {
                            self.groups = groups.clone();
                            if self.groups.is_some() {
                                self.groups.as_mut().unwrap().sort_by(|groupa, groupb| {
                                    let auto_group = self.group_prefix.as_ref().unwrap();
                                    let invite_group =
                                        format!("{}-invite", self.group_prefix.as_ref().unwrap());

                                    if groupa.cn.eq(auto_group) && groupb.cn.eq(&invite_group) {
                                        Ordering::Less
                                    } else if groupb.cn.eq(auto_group)
                                        && groupa.cn.eq(&invite_group)
                                    {
                                        Ordering::Greater
                                    } else if groupa.cn.eq(auto_group) {
                                        Ordering::Less
                                    } else if groupb.cn.eq(auto_group) {
                                        Ordering::Greater
                                    } else if groupa.cn.eq(&invite_group) {
                                        Ordering::Less
                                    } else if groupb.cn.eq(&invite_group) {
                                        Ordering::Greater
                                    } else {
                                        groupa.cmp(groupb)
                                    }
                                })
                            }

                            self.get_groups_promise = None;
                        }
                        Err(e) => self.current_error = Some(AppError::InternalError(e.to_string())),
                    };
                }
            }
        }

        // Get users promise.
        if let Some(p) = &self.get_users_promise {
            match p.ready() {
                None => (),
                Some(try_users) => {
                    self.is_working = false;

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

        // Render page only when admin and group prefix are retrieved.
        if self.admin.is_some() && self.group_prefix.is_some() {
            match self.state.active_page {
                Page::Main => main::ui::update(self, ctx, frame),
            }
        }

        // Get initial directory and group list and admin.
        START.call_once(|| {
            self.is_working = true;

            self.get_config_prefix_promise = Some(api::root::get_config(ctx, self.api_url.clone()));
        });
    }
}

fn setup_custom_fonts(ctx: &egui::Context) {
    // Start with the default fonts (we will be adding to them rather than replacing them).
    let mut fonts = egui::FontDefinitions::default();

    // Install custom fonts.
    // .ttf and .otf files supported.
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

    // Start at 1 not 0 to keep the default font.
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(1, "Font-Awesome-6-Brands-Regular-400".to_owned());
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(2, "Font-Awesome-6-Free-Regular-400".to_owned());
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(3, "Font-Awesome-6-Free-Solid-900".to_owned());

    // Tell egui to use these fonts:
    ctx.set_fonts(fonts);
}
