use poll_promise::Promise;

use storm_daenerys_common::types::{acl::SetAcl, error::CommonError};

pub fn save_acl(ctx: &egui::Context, set_acl: SetAcl) -> Promise<Result<(), std::string::String>> {
    dbg!("Save ACL.");

    // TODO: handle error here.
    let request_payload = serde_json::to_string(&set_acl).unwrap();

    let ctx = ctx.clone();
    let (sender, promise) = Promise::new();

    let request = ehttp::Request {
        method: "POST".to_owned(),
        url: "http://localhost:3000/acls".to_string(),
        body: request_payload.as_bytes().to_vec(),
        headers: ehttp::headers(&[("Accept", "*/*"), ("Content-Type", "application/json")]),
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
