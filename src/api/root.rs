use log::debug;
use poll_promise::Promise;
use storm_daenerys_common::types::error::CommonError;

pub fn get_admin(ctx: &egui::Context, api_url: String) -> Promise<Result<Option<String>, String>> {
    dbg!("Get admin.");

    // Begin download.
    // We download the image using `ehttp`, a library that works both in WASM and on native.
    // We use the `poll-promise` library to communicate with the UI thread.
    let ctx = ctx.clone();
    let (sender, promise) = Promise::new();
    let request = ehttp::Request::get(format!("{}/admin", api_url));

    ehttp::fetch(request, move |response| {
        let admin = response.and_then(parse_get_admin_response);
        sender.send(admin);
        ctx.request_repaint(); // wake up UI thread
    });

    promise
}

fn parse_get_admin_response(response: ehttp::Response) -> Result<Option<String>, String> {
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
            None => Err(CommonError::UnexpectedResponse("empty admin".to_string()).to_string()),
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
