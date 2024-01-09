use eframe::egui;
use log::debug;
use poll_promise::Promise;

use storm_daenerys_common::types::{
    directory::{CreateDirectory, Directory},
    error::CommonError,
};

pub fn create_directory(
    ctx: &egui::Context,
    create_directory: CreateDirectory,
    api_url: String,
) -> Promise<Result<(), String>> {
    dbg!("Create directory: {:?}", &create_directory);

    let ctx = ctx.clone();
    let (sender, promise) = Promise::new();

    let request_payload = match serde_json::to_string(&create_directory) {
        Ok(request_payload) => request_payload,
        Err(e) => {
            sender.send(Err(e.to_string()));
            return promise;
        }
    };

    let request = ehttp::Request {
        method: "POST".to_owned(),
        url: format!("{}/directories", api_url),
        body: request_payload.as_bytes().to_vec(),
        headers: ehttp::headers(&[("Accept", "*/*"), ("Content-Type", "application/json")]),
    };

    ehttp::fetch(request, move |response| {
        let create_directory_result = response.and_then(parse_create_directory_response);
        sender.send(create_directory_result);
        ctx.request_repaint(); // wake up UI thread
    });

    promise
}

pub fn get_root_directories(
    ctx: &egui::Context,
    api_url: String,
) -> Promise<Result<Option<Vec<Directory>>, String>> {
    dbg!("Get directory list.");

    // Begin download.
    // We download the image using `ehttp`, a library that works both in WASM and on native.
    // We use the `poll-promise` library to communicate with the UI thread.
    let ctx = ctx.clone();
    let (sender, promise) = Promise::new();
    let request = ehttp::Request::get(format!("{}/directories", api_url));

    ehttp::fetch(request, move |response| {
        let folders = response.and_then(parse_get_directories_response);
        sender.send(folders);
        ctx.request_repaint(); // wake up UI thread
    });

    promise
}

fn parse_get_directories_response(
    response: ehttp::Response,
) -> Result<Option<Vec<Directory>>, String> {
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

fn parse_create_directory_response(response: ehttp::Response) -> Result<(), String> {
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
