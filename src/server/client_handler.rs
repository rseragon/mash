use std::path::Path;

use crate::cliparser::Config;
use crate::server::request::Request;
use crate::server::response::Response;
use crate::server::response::ResponseCode;
use crate::utils::path_utils::server_cwd_path;

pub async fn process_request(req: Request, config: &Config) -> Response {
    use crate::server::response::ResponseCode::*;

    let path_str = match server_cwd_path(&req.path, config) {
        Ok(p) => p,
        Err(ee) => {
            return Response::new(ee.err, ee.expl);
        }
    };

    let resp_code: ResponseCode;
    let resp_str: String;

    // given path is a directory
    if Path::new(&path_str).is_dir() {
        resp_code = OK_200;

        resp_str = dir_listing(&path_str);
    } else {
        // is a file
        match read_file(path_str).await {
            Err(e) => {
                // TODO: add better html for error
                resp_code = NOT_FOUND_404;
                resp_str = e;
            }
            Ok(s) => {
                resp_code = OK_200;
                resp_str = s;
            }
        };
    }

    Response::new(resp_code, resp_str)
}

async fn read_file(path: String) -> Result<String, String> {
    match std::fs::read_to_string(path) {
        Ok(s) => return Ok(s),
        Err(err) => return Err(String::from("Failed to read file")),
    }
}

fn dir_listing(path_str: &String) -> String {
    let mut dir_list = String::new();

    for p in std::fs::read_dir(&path_str).unwrap() {
        let dir = p.unwrap().path().display().to_string();
        dir_list.push_str(&format!("<li><a href='{}'>{}</a></li>\n", dir, dir));
    }

    let head = format!(
        "<!DOCTYPE HTML PUBLIC>\n\
<html>\n\
<head>\n\
<meta http-equiv='Content-Type' content='text/html; charset=utf-8'>\n\
<title> Directory listing for {} </title>\n\
<h1> Directory listing for {} </h1>\n\
<hr>\n\
<ul>\n\
{}\n\
</ul>\n\
<hr>\n\
</body></html>\n",
        path_str, path_str, dir_list
    );

    head
}
