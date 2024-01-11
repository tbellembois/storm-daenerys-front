use eframe::egui;
use log::debug;
use poll_promise::Promise;
use storm_daenerys_common::types::{config::Config, directory::Quota, error::CommonError};

pub fn get_config(ctx: &egui::Context, api_url: String) -> Promise<Result<Config, String>> {
    dbg!("Get config.");

    // Begin download.
    // We download the image using `ehttp`, a library that works both in WASM and on native.
    // We use the `poll-promise` library to communicate with the UI thread.
    let ctx = ctx.clone();
    let (sender, promise) = Promise::new();
    let request = ehttp::Request::get(format!("{}/config", api_url));

    ehttp::fetch(request, move |response| {
        let config = response.and_then(parse_get_config_response);
        sender.send(config);
        ctx.request_repaint(); // wake up UI thread
    });

    promise
}

pub fn get_du(
    ctx: &egui::Context,
    api_url: String,
    width: u32,
) -> Promise<Result<Option<String>, String>> {
    dbg!("Get du.");

    // Begin download.
    // We download the image using `ehttp`, a library that works both in WASM and on native.
    // We use the `poll-promise` library to communicate with the UI thread.
    let ctx = ctx.clone();
    let (sender, promise) = Promise::new();
    let request = ehttp::Request::get(format!("{}/du?width={}", api_url, width));

    ehttp::fetch(request, move |response| {
        let du = response.and_then(parse_get_du_response);
        sender.send(du);
        ctx.request_repaint(); // wake up UI thread
    });

    promise
}

fn parse_get_config_response(response: ehttp::Response) -> Result<Config, String> {
    let status = &response.status;
    let status_text = &response.status_text;
    let maybe_text_response = response.text();

    debug!("{:?}", status);
    debug!("{:?}", status_text);
    debug!("{:?}", maybe_text_response);

    // TODO: check Config

    match status {
        200 => match maybe_text_response {
            Some(text_response) => match serde_json::from_str(text_response) {
                Ok(json_response) => Ok(json_response),
                Err(e) => Err(e.to_string()),
            },
            None => Err(CommonError::UnexpectedResponse("empty config".to_string()).to_string()),
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

fn parse_get_du_response(response: ehttp::Response) -> Result<Option<String>, String> {
    let status = &response.status;
    let status_text = &response.status_text;
    let maybe_text_response = response.text();

    debug!("{:?}", status);
    debug!("{:?}", status_text);
    debug!("{:?}", maybe_text_response);

    match status {
        200 => match maybe_text_response {
            Some(text_response) => Ok(Some(text_response.to_string())),
            None => Err(CommonError::UnexpectedResponse("empty du".to_string()).to_string()),
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
