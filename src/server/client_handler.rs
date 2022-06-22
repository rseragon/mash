use std::path::Path;

use crate::cliparser::Config;
use crate::server::request::Request;
use crate::server::request::RequestType;
use crate::server::response::Response;
use crate::server::response::ResponseCode;

pub async fn process_request(req: Request, config: &Config) -> Response {

    // TODO: This is repeated in path_utils
    let mut path = std::path::PathBuf::new();
    path.push(&config.path);
    let mut path_str = req.path.to_string();
    path_str.remove(0); // Removes the extra '/' at the front
                        // eg: /index.html -> index.html
    path.push(path_str);

    // TODO: Find better way
    let path_str = path.display().to_string();

    let resp_code: ResponseCode;
    let resp_str: String;

    match read_file(path_str).await {
        Err(e) => {
            // TODO: add better html for error
            resp_code = ResponseCode::NOT_FOUND_404;
            resp_str = e;
        },
        Ok(s) => {
            resp_code = ResponseCode::OK_200;
            resp_str = s;
        },
    };

    Response::new(resp_code, resp_str)
}

async fn read_file(path: String) -> Result<String, String> {

    match std::fs::read_to_string(path) {
        Ok(s) => return Ok(s),
        Err(err) => return Err(String::from("Failed to read file")),
    }

}
