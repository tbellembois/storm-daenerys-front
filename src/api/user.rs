use eframe::egui;
use log::debug;
use poll_promise::Promise;

use storm_daenerys_common::types::{error::CommonError, user::User};

pub fn get_user_display(
    ctx: &egui::Context,
    q: String,
    api_url: String,
) -> Promise<Result<Option<String>, String>> {
    debug!("Get user display.");

    // Begin download.
    // We download the image using `ehttp`, a library that works both in WASM and on native.
    // We use the `poll-promise` library to communicate with the UI thread.
    let ctx = ctx.clone();
    let (sender, promise) = Promise::new();
    let request = ehttp::Request::get(format!("{}/userdisplay?q={}", api_url, q));

    ehttp::fetch(request, move |response| {
        let userdisplay = response.and_then(parse_get_user_display_response);
        sender.send(userdisplay);
        ctx.request_repaint(); // wake up UI thread
    });

    promise
}

pub fn get_users(
    ctx: &egui::Context,
    q: String,
    api_url: String,
) -> Promise<Result<Option<Vec<User>>, String>> {
    debug!("Get user list.");

    // Begin download.
    // We download the image using `ehttp`, a library that works both in WASM and on native.
    // We use the `poll-promise` library to communicate with the UI thread.
    let ctx = ctx.clone();
    let (sender, promise) = Promise::new();
    let request = ehttp::Request::get(format!("{}/users?q={}", api_url, q));

    ehttp::fetch(request, move |response| {
        let users = response.and_then(parse_get_users_response);
        sender.send(users);
        ctx.request_repaint(); // wake up UI thread
    });

    promise
}

fn parse_get_user_display_response(response: ehttp::Response) -> Result<Option<String>, String> {
    let status = &response.status;
    let status_text = &response.status_text;
    let maybe_text_response = response.text();

    debug!("{:?}", status);
    debug!("{:?}", status_text);
    debug!("{:?}", maybe_text_response);

    // TODO: check Config

    match status {
        200 => match maybe_text_response {
            Some(text_response) => {
                let maybe_json_response: Option<String> =
                    serde_json::from_str(text_response).unwrap();
                match maybe_json_response {
                    Some(json_response) => {
                        debug!("a");
                        Ok(Some(json_response))
                    }
                    None => {
                        debug!("z");
                        Ok(None)
                    }
                }
            }

            None => Ok(Some(String::from("invalid user"))),
        },
        _ => match maybe_text_response {
            Some(text_response) => {
                let common_error: CommonError =
                    match serde_json::from_str::<CommonError>(text_response) {
                        Ok(common_error) => {
                            debug!("c");
                            common_error
                        }
                        Err(e) => {
                            debug!("d");
                            CommonError::InternalServerError(e.to_string())
                        }
                    };
                debug!("b");
                Err(common_error.to_string())
            }
            None => {
                debug!("y");
                Err(status.to_string())
            }
        },
    }
}

fn parse_get_users_response(response: ehttp::Response) -> Result<Option<Vec<User>>, String> {
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
