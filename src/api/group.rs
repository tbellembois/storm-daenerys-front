use eframe::egui;
use log::debug;
use poll_promise::Promise;

use storm_daenerys_common::types::{
    error::CommonError,
    group::{AddDelUserToGroup, Group},
};

pub fn save_group(
    ctx: &egui::Context,
    group_backup: Group,
    group: Group,
    api_url: String,
) -> Vec<Promise<Result<(), std::string::String>>> {
    let mut result: Vec<Promise<Result<(), std::string::String>>> = Vec::new();

    // Delete former members.
    if group_backup.member.is_some() {
        for member_backup in group_backup.member.as_ref().unwrap() {
            let mut member_to_del = false;

            if group.member.is_some() {
                let member_find = group
                    .member
                    .as_ref()
                    .unwrap()
                    .iter()
                    .find(|m| m.eq(&member_backup));

                if member_find.is_none() {
                    member_to_del = true;
                }
            } else {
                member_to_del = true;
            }

            if member_to_del {
                debug!("deleting member {:?}", member_backup.to_string());

                result.push(del_user_from_group(
                    ctx,
                    AddDelUserToGroup {
                        group_cn: group.cn.clone(),
                        user_cn: member_backup.to_string(),
                        ..AddDelUserToGroup::default()
                    },
                    api_url.clone(),
                ));
            }
        }
    }

    // Add new members.
    if group.member.is_some() {
        for member in group.member.unwrap() {
            let mut member_to_add = false;

            if group_backup.member.is_some() {
                let member_find = group_backup
                    .member
                    .as_ref()
                    .unwrap()
                    .iter()
                    .find(|m| m.eq(&&member));

                if member_find.is_none() {
                    member_to_add = true;
                }
            } else {
                member_to_add = true;
            }

            if member_to_add {
                debug!("adding member {:?}", member.to_string());

                result.push(add_user_to_group(
                    ctx,
                    AddDelUserToGroup {
                        group_cn: group.cn.clone(),
                        user_cn: member.to_string(),
                        ..AddDelUserToGroup::default()
                    },
                    api_url.clone(),
                ));
            }
        }
    }

    result
}

pub fn del_user_from_group(
    ctx: &egui::Context,
    del_user_from_group: AddDelUserToGroup,
    api_url: String,
) -> Promise<Result<(), std::string::String>> {
    debug!("Del user from group: {:?}", &del_user_from_group);

    let ctx = ctx.clone();
    let (sender, promise) = Promise::new();

    let request_payload = match serde_json::to_string(&del_user_from_group) {
        Ok(request_payload) => request_payload,
        Err(e) => {
            sender.send(Err(e.to_string()));
            return promise;
        }
    };

    let request = ehttp::Request {
        method: "DELETE".to_owned(),
        url: format!("{}/groups/user", api_url),
        body: request_payload.as_bytes().to_vec(),
        headers: ehttp::Headers::new(&[("Accept", "*/*"), ("Content-Type", "application/json")]),
    };

    ehttp::fetch(request, move |response| {
        let del_user_from_group_result = response.and_then(parse_add_del_user_to_group_response);
        sender.send(del_user_from_group_result);
        ctx.request_repaint(); // wake up UI thread
    });

    promise
}

pub fn add_user_to_group(
    ctx: &egui::Context,
    add_user_to_group: AddDelUserToGroup,
    api_url: String,
) -> Promise<Result<(), std::string::String>> {
    debug!("Add user to group: {:?}", &add_user_to_group);

    let ctx = ctx.clone();
    let (sender, promise) = Promise::new();

    let request_payload = match serde_json::to_string(&add_user_to_group) {
        Ok(request_payload) => request_payload,
        Err(e) => {
            sender.send(Err(e.to_string()));
            return promise;
        }
    };

    debug!("request_payload: {}", &request_payload);

    let request = ehttp::Request {
        method: "PATCH".to_owned(),
        url: format!("{}/groups/user", api_url),
        body: request_payload.as_bytes().to_vec(),
        headers: ehttp::Headers::new(&[("Accept", "*/*"), ("Content-Type", "application/json")]),
    };

    ehttp::fetch(request, move |response| {
        let add_user_to_group_result = response.and_then(parse_add_del_user_to_group_response);
        sender.send(add_user_to_group_result);
        ctx.request_repaint(); // wake up UI thread
    });

    promise
}

pub fn create_group(
    ctx: &egui::Context,
    create_group: Group,
    api_url: String,
) -> Promise<Result<(), String>> {
    debug!("Create group: {:?}", &create_group);

    let ctx = ctx.clone();
    let (sender, promise) = Promise::new();

    let request_payload = match serde_json::to_string(&create_group) {
        Ok(request_payload) => request_payload,
        Err(e) => {
            sender.send(Err(e.to_string()));
            return promise;
        }
    };

    let request = ehttp::Request {
        method: "POST".to_owned(),
        url: format!("{}/groups", api_url),
        body: request_payload.as_bytes().to_vec(),
        headers: ehttp::Headers::new(&[("Accept", "*/*"), ("Content-Type", "application/json")]),
    };

    ehttp::fetch(request, move |response| {
        let create_group_result = response.and_then(parse_add_del_user_to_group_response);
        sender.send(create_group_result);
        ctx.request_repaint(); // wake up UI thread
    });

    promise
}

pub fn delete_group(
    ctx: &egui::Context,
    cn: String,
    api_url: String,
) -> Promise<Result<(), String>> {
    debug!("Delete group: {:?}", &cn);

    let request_payload = "".to_string();

    let ctx = ctx.clone();
    let (sender, promise) = Promise::new();

    let request = ehttp::Request {
        method: "DELETE".to_owned(),
        url: format!("{}/groups/{}", api_url, cn),
        body: request_payload.as_bytes().to_vec(),
        headers: ehttp::Headers::new(&[("Accept", "*/*")]),
    };

    ehttp::fetch(request, move |response| {
        let delete_group_result = response.and_then(parse_add_del_user_to_group_response);
        sender.send(delete_group_result);
        ctx.request_repaint(); // wake up UI thread
    });

    promise
}

pub fn get_groups(
    ctx: &egui::Context,
    api_url: String,
) -> Promise<Result<Option<Vec<Group>>, String>> {
    debug!("Get group list.");

    // Begin download.
    // We download the image using `ehttp`, a library that works both in WASM and on native.
    // We use the `poll-promise` library to communicate with the UI thread.
    let ctx = ctx.clone();
    let (sender, promise) = Promise::new();
    let request = ehttp::Request::get(format!("{}/groups", api_url));

    ehttp::fetch(request, move |response| {
        let groups = response.and_then(parse_get_groups_response);
        sender.send(groups);
        ctx.request_repaint(); // wake up UI thread
    });

    promise
}

fn parse_get_groups_response(response: ehttp::Response) -> Result<Option<Vec<Group>>, String> {
    let status = &response.status;
    let status_text = &response.status_text;
    let maybe_text_response = response.text();

    debug!("{:?}", status);
    debug!("{:?}", status_text);
    debug!("{:?}", maybe_text_response);

    match status {
        200 => match maybe_text_response {
            Some(text_response) => match serde_json::from_str(text_response) {
                Ok(json_response) => Ok(json_response),
                Err(e) => Err(e.to_string()),
            },
            None => Ok(None),
        },
        _ => match maybe_text_response {
            Some(text_response) => {
                let common_error: CommonError =
                    match serde_json::from_str::<CommonError>(text_response) {
                        Ok(common_error) => common_error,
                        Err(e) => CommonError::InternalServerError(e.to_string()),
                    };
                Err(common_error.to_string())
            }
            None => Err(status.to_string()),
        },
    }
}

fn parse_add_del_user_to_group_response(response: ehttp::Response) -> Result<(), String> {
    let status = &response.status;
    let status_text = &response.status_text;
    let maybe_text_response = response.text();

    debug!("{:?}", status);
    debug!("{:?}", status_text);
    debug!("{:?}", maybe_text_response);

    match status {
        200 => Ok(()),
        _ => match maybe_text_response {
            Some(text_response) => {
                let common_error: CommonError =
                    match serde_json::from_str::<CommonError>(text_response) {
                        Ok(common_error) => common_error,
                        Err(e) => CommonError::InternalServerError(e.to_string()),
                    };
                Err(common_error.to_string())
            }
            None => Err(status.to_string()),
        },
    }
}
