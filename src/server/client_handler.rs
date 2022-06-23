use std::path::Path;

use crate::cliparser::Config;
use crate::server::request::Request;
use crate::server::request::RequestType;
use crate::server::response::Response;
use crate::server::response::ResponseCode;
use crate::utils::path_utils::server_cwd_path;

pub async fn process_request(req: Request, config: &Config) -> Response {

    let path_str = match server_cwd_path(&req.path, config) {
        Ok(p) => p,
        Err(e) => {
            return Response::new(e.err, e.expl);
        }
    };

    let resp_code: ResponseCode;
    let resp_str: String;

    match read_file(path_str).await {
        Err(e) => {
            // TODO: add better html for error
            resp_code = ResponseCode::NOT_FOUND_404;
            resp_str = e;
        }
        Ok(s) => {
            resp_code = ResponseCode::OK_200;
            resp_str = s;
        }
    };

    Response::new(resp_code, resp_str)
}

async fn read_file(path: String) -> Result<String, String> {
    match std::fs::read_to_string(path) {
        Ok(s) => return Ok(s),
        Err(err) => return Err(String::from("Failed to read file")),
    }
}
