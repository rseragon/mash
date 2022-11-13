use std::arch::x86_64::_MM_FROUND_CUR_DIRECTION;
use std::path::Path;

use tokio::io::AsyncReadExt;
use std::collections::HashMap;

use crate::cliparser::Config;
use crate::server::request::Request;
use crate::server::response::Response;
use crate::server::response::ResponseCode;
use crate::utils::html_builder::dir_list_html;
use crate::utils::html_builder::text_page;
use crate::utils::path_utils::server_cwd_path;
use crate::utils::html_builder;

pub async fn process_request(req: Request, config: &Config) -> Response {
    use crate::server::response::ResponseCode::*;

    // Verficiation and access rights is done here
    let path_str = match server_cwd_path(&req.path, config) {
        Ok(p) => p,
        Err(ee) => {
            // return Response::new(ee.err, ee.expl.as_bytes().to_vec());
            // let body = html_builder::error_page_builder(&ee.err, &ee.expl).as_bytes().to_vec();
            let body = html_builder::error_page_builder(&ee.err, &ee.expl).as_bytes().to_vec();
            return Response::new(ee.err, body);
        }
    };

    let resp_code: ResponseCode;
    let mut resp_header: HashMap<&'static str, String> = HashMap::new();
    let body: Vec<u8>;

    // given path is a directory
    let curr_path = Path::new(&path_str);
    if curr_path.is_dir() {
        resp_code = Ok200;

        // If the dir has a `index.html` already, then load it
        // else give directory listing
        let index_html_path = curr_path.join("index.html");

        if config.detect_index_html && index_html_path.exists() {

            body = match read_file(&index_html_path.display().to_string()).await {
                Err(_) => dir_listing(&path_str, &config).as_bytes().to_vec(), // If failed, just
                                                                               // give dir listing
                Ok(b) => b
            };
            resp_header.insert("Content-Type", "text/html".to_string());
        }

        else {
            body = dir_listing(&path_str, &config).as_bytes().to_vec();
        }

    } else { // is a file
        match read_file(&path_str).await {
            Err(e) => {
                resp_code = NotFound404;
                // resp_str = e.as_bytes().to_vec();
                body = html_builder::error_page_builder(&resp_code, &e).as_bytes().to_vec();
                resp_header.insert("Content-Type", "text/html".to_string()); // returned reponse is
                                                                             // an error page
            }
            Ok(s) => {
                resp_code = Ok200;

                // MimeType checking only for a file which exists
                if let Ok(mime_type) = get_mime_type(&path_str)  {
                    paris::success!("{}: {}", path_str, mime_type);

                    if mime_type == "text/markdown" {
                        body = text_page(&path_str).as_bytes().to_vec(); // TODO: This seems dubious
                                                                         // since we've to convert
                                                                         // strings into vector
                        resp_header.insert("Content-Type", "text/html".to_string());
                    }
                    else {
                        body = s; // If mime gets okay but file is not md file
                        resp_header.insert("Content-Type", mime_type);
                    }
                    // Set the file size for known mime types
                    // so that browser can read it

                    // TODO: Need to including this after setting body (There are changes we need somthing else
                    // like mark down will get it's own page, mp4 will get it's own player)
                    // let size = std::fs::metadata(&path_str).unwrap().size();
                    // resp_header.insert("Content-Length", size.to_string());

                }
                else {
                    body = s; // If mime files to acertain the file type
                }
            }
        };
    }

    // TODO: Set other headers
    // Make sure the connection is not cut off
    // resp_header.insert("Keep-Alive", "timeout=5, max=1000".to_string()); // TODO: IDK this
    resp_header.insert("Connection", "Keep-Alive".to_string());

    let mut resp = Response::new(resp_code, body);
    resp.set_headers(resp_header);

    resp
}

async fn read_file(path: &String) -> Result<Vec<u8>, String> {
    let mut file = match tokio::fs::File::open(path).await {
        Ok(f) => f,
        Err(err) => return Err(format!("Failed to open file: {}", err)),
    };

    let mut buf = Vec::new();

    match file.read_to_end(&mut buf).await {
        Ok(_) => {},
        Err(err) => return Err(format!("Failed to read file: {}", err)),
    }

    Ok(buf)
}

fn dir_listing(path_str: &String, config: &Config) -> String {

    dir_list_html(path_str, config)
}

fn get_mime_type(path_str: &String) -> Result<String, ()> {

    // directories don't have mime types 
    // https://stackoverflow.com/questions/18869772/mime-type-for-a-directory
    if Path::new(path_str).is_dir() {
        return Err(());
    }

    let guess = mime_guess::from_path(path_str);

    // Didn't find mime type
    if guess.is_empty() {
        paris::error!("Unabled to determine MimeType: {}", path_str);
        return Err(());
    }

    // TODO: is okay for now, but should consider others
    let mime_type = guess.first_raw().unwrap();

    Ok(String::from(mime_type))

}
