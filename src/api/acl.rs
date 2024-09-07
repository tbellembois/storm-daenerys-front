use eframe::egui;
use log::debug;
use poll_promise::Promise;

use storm_daenerys_common::types::{acl::SetAcl, error::CommonError};

#[cfg(target_arch = "wasm32")]
use ehttp::Mode;

pub fn save_acl(
    ctx: &egui::Context,
    set_acl: SetAcl,
    api_url: String,
) -> Promise<Result<(), std::string::String>> {
    debug!("Save ACL: {:#?}", set_acl);

    let ctx = ctx.clone();
    let (sender, promise) = Promise::new();

    let request_payload = match serde_json::to_string(&set_acl) {
        Ok(request_payload) => request_payload,
        Err(e) => {
            sender.send(Err(e.to_string()));
            return promise;
        }
    };

    let request = ehttp::Request {
        method: "POST".to_owned(),
        url: format!("{}/acls", api_url),
        body: request_payload.as_bytes().to_vec(),
        headers: ehttp::Headers::new(&[("Accept", "*/*"), ("Content-Type", "application/json")]),
        #[cfg(target_arch = "wasm32")]
        mode: Mode::default(),
    };

    ehttp::fetch(request, move |response| {
        let save_directory_acl_result = response.and_then(parse_save_acl_response);
        sender.send(save_directory_acl_result);
        ctx.request_repaint(); // wake up UI thread
    });

    promise
}

fn parse_save_acl_response(response: ehttp::Response) -> Result<(), String> {
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
