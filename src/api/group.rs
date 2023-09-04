use poll_promise::Promise;

use storm_daenerys_common::types::{
    error::CommonError,
    group::{AddUserToGroup, Group},
};

pub fn add_user_to_group(
    ctx: &egui::Context,
    add_user_to_group: AddUserToGroup,
) -> Promise<Result<(), std::string::String>> {
    dbg!("Add user to group.");

    // TODO: handle error here.
    let request_payload = serde_json::to_string(&add_user_to_group).unwrap();

    let ctx = ctx.clone();
    let (sender, promise) = Promise::new();

    let request = ehttp::Request {
        method: "PATCH".to_owned(),
        url: "http://localhost:3000/groups/user".to_string(),
        body: request_payload.as_bytes().to_vec(),
        headers: ehttp::headers(&[("Accept", "*/*"), ("Content-Type", "application/json")]),
    };

    ehttp::fetch(request, move |response| {
        let save_directory_acl_result = response.and_then(parse_add_user_to_group_response);
        sender.send(save_directory_acl_result);
        ctx.request_repaint(); // wake up UI thread
    });

    promise
}

pub fn get_groups(ctx: &egui::Context) -> Promise<Result<Option<Vec<Group>>, String>> {
    dbg!("Get group list.");

    // Begin download.
    // We download the image using `ehttp`, a library that works both in WASM and on native.
    // We use the `poll-promise` library to communicate with the UI thread.
    let ctx = ctx.clone();
    let (sender, promise) = Promise::new();
    let request = ehttp::Request::get("http://localhost:3000/groups");

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

    tracing::debug!("{:?}", status);
    tracing::debug!("{:?}", status_text);
    tracing::debug!("{:?}", maybe_text_response);

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
            None => Ok(None),
        },
    }
}

fn parse_add_user_to_group_response(response: ehttp::Response) -> Result<(), String> {
    let status = &response.status;
    let status_text = &response.status_text;
    let maybe_text_response = response.text();

    tracing::debug!("{:?}", status);
    tracing::debug!("{:?}", status_text);
    tracing::debug!("{:?}", maybe_text_response);

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
            None => Ok(()),
        },
    }
}
