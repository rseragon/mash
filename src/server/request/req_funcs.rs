use std::collections::HashMap;
use crate::{server::response::ResponseCode, cliparser::Config};
use crate::utils::err_and_expl::ErrAndExpl;

use super::{Request, RequestType, HttpVersion};


impl Request {

    // TODO: parse content reqeust headers
    // TODO: print info about the request parsing
    /// Parses and verifies the request
    pub fn parse(buf: &[u8], _config: &Config) -> Result<Request, ErrAndExpl<ResponseCode>> {
        let buf_str = match std::str::from_utf8(buf) {
            Ok(res) => res,
            Err(_) => { 
                return Err(ErrAndExpl::new(ResponseCode::BadRequest400, 
                                           String::from("Invalid reqeust encoding: Has to be UTF-8")));
            },
        };

        let mut iter = buf_str.clone().split_ascii_whitespace();

        // EG:
        // GET  /  HTTP/1.1
        // (1) (2)  (3)
        // Content-Headers
        // (4)
        //
        // Extra data
        // (5)

        // (1) Method
        let method_str = match iter.next() {
            None => {
                return Err(ErrAndExpl::new(ResponseCode::BadRequest400,
                                           String::from("Illegal request format")));
            },
            Some(met) => met,
        };
        let method: RequestType;

        if method_str == "GET" {
            method = RequestType::GET;
        }
        else if method_str == "POST" {
            method = RequestType::POST;
        }
        else {
            return Err(ErrAndExpl::new(ResponseCode::NotImplemented501,
                                       format!("Unkown request type: {method_str}")));
        }

        // (2) request path
        let mut req_path = match iter.next() {
            None => {
                return Err(ErrAndExpl::new(ResponseCode::BadRequest400,
                                           String::from("Illegal path format: request doesn't contain path")));
            },
            Some(p) => p,
        };
        paris::info!("Path requested: {}", req_path);
        // Verifies the path
        /* will be done by client_handler
         * since the parser has only has to worry about semantics
         * and and not the correctness of the request
        match path_utils::verify_server_relative_path(req_path, config) {
            false => {
                return Err(ErrAndExpl::new(ResponseCode::NOT_FOUND_404,
                                       format!("Invalid path: {req_path}")));
            },
            true => {}
        }
        */
        // Check for GET request arguments
        let mut arguments = HashMap::new(); // TODO: Init only once
        if method == RequestType::GET && req_path.contains("?") {

            let (real_path, args) = match req_path.split_once("?") {
                Some((r, a)) => (r,a),
                None => (req_path, ""),
            };

            if !args.is_empty() {
                arguments = parse_path_args(args);
            }
            req_path = real_path;
        }

        // (3) HTTP version
        let version_str = match iter.next() {
            None => {
                return Err(ErrAndExpl::new(ResponseCode::BadRequest400,
                                           String::from("Illegal version format: request HTTP version not found")));
            },
            Some(v) => v,
        };
        let version: HttpVersion;

        if version_str == "HTTP/1.1" {
            version = HttpVersion::Http1_1;
        }
        else if version_str == "HTTP/1.0" {
            version = HttpVersion::Http1_0;
        }
        else {
            return Err(ErrAndExpl::new(ResponseCode::HttpVersionNotSupported505,
                                       format!("HTTP version '{version_str}' not supported")));
        }

        /* NEW iterator to collect other data */
        // Collect the rest of data 
        // content headers, arguments, extra data

        let mut line_iter = buf_str.split("\n");
        // Skip the first line ;)
        match line_iter.next() {
            Some(_) => {},
            None => {}
        };

        let mut content_headers = HashMap::new();
        let mut extra_data = String::new();
        loop {
            
            let text = match line_iter.next() {
                Some(x) => x,
                None => break
            };

            let (k, v) = match text.split_once(":") {
                Some((k,v)) => (k,v),
                None => {
                    extra_data.push_str(text);
                    continue;
                }
            };
            content_headers.insert(k.trim().to_string(), v.trim().to_string());
        };

        // Get POST arguments
        if method == RequestType::POST {

            let args = extra_data.split("&");

            for arg in args {
                let (k, v) = match arg.split_once("=") {
                    Some((k,v)) => (k,v),
                    None => continue,
                };
                arguments.insert(k.trim().to_string(), v.trim().to_string());
            }
        }

        Ok(Request {
            req_type: method,
            path: String::from(req_path),
            http_version: version,

            content_headers: content_headers,

            arguments: arguments,
            extra_data: extra_data,
        })
    }
}

fn parse_path_args(args: &str) -> HashMap<String, String> {
    let mut argument_map = HashMap::new();

    for arg in args.split("&") {

        let (key, val) = match arg.split_once("=") {
            Some((k,v)) => (k,v),
            None => continue
        };

        argument_map.insert(key.to_string(), val.to_string());

    }

    argument_map
}
