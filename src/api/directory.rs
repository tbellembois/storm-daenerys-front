use poll_promise::Promise;

use storm_daenerys_common::types::directory::Directory;

pub fn get_root_directories(
    ctx: &egui::Context,
) -> Promise<Result<Option<Vec<Directory>>, String>> {
    dbg!("Get directory list.");

    // Begin download.
    // We download the image using `ehttp`, a library that works both in WASM and on native.
    // We use the `poll-promise` library to communicate with the UI thread.
    let ctx = ctx.clone();
    let (sender, promise) = Promise::new();
    let request = ehttp::Request::get("http://localhost:3000/directories");

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
    let maybe_text_response = response.text();

    tracing::debug!("{:?}", maybe_text_response);

    match maybe_text_response {
        Some(text_response) => match serde_json::from_str(text_response) {
            Ok(json_response) => Ok(json_response),
            Err(e) => Err(e.to_string()),
        },
        None => Ok(None),
    }
}
