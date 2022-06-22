use std::collections::HashMap;

use crate::{server::response::ResponseCode, cliparser::Config};
use crate::utils::err_and_expl::ErrAndExpl;
use crate::utils::path_utils;

use super::{Request, RequestType, HttpVersion};


impl Request {

    // TODO: parse content reqeust headers
    // TODO: print info about the request parsing
    /// Parses and verifies the request
    pub fn parse(buf: &[u8], config: &Config) -> Result<Request, ErrAndExpl<ResponseCode>> {
        let buf_str = match std::str::from_utf8(buf) {
            Ok(res) => res,
            Err(_) => { 
                return Err(ErrAndExpl::new(ResponseCode::BAD_REQUEST_400, 
                                           String::from("Invalid reqeust encoding: Has to be UTF-8")));
            },
        };

        let mut iter = buf_str.split_ascii_whitespace();

        // EG:
        // GET  /  HTTP/1.1
        // (1) (2)  (3)

        // (1) Method
        let method_str = match iter.next() {
            None => {
                return Err(ErrAndExpl::new(ResponseCode::BAD_REQUEST_400,
                                           String::from("Illegal request format")));
            },
            Some(met) => met,
        };
        let method: RequestType;

        if method_str == "GET" {
            method = RequestType::GET;
        }
        else {
            return Err(ErrAndExpl::new(ResponseCode::NOT_IMPLEMENTED_501,
                                       format!("Unkown request type: {method_str}")));
        }

        // (2) request path
        let req_path = match iter.next() {
            None => {
                return Err(ErrAndExpl::new(ResponseCode::BAD_REQUEST_400,
                                           String::from("Illegal path format: request doesn't contain path")));
            },
            Some(p) => p,
        };
        // Verifies the path
        match path_utils::verify_server_relative_path(req_path, config) {
            false => {
                return Err(ErrAndExpl::new(ResponseCode::NOT_FOUND_404,
                                       format!("Invalid path: {req_path}")));
            },
            true => {}
        }


        // (3) HTTP version
        let version_str = match iter.next() {
            None => {
                return Err(ErrAndExpl::new(ResponseCode::BAD_REQUEST_400,
                                           String::from("Illegal version format: request HTTP version not found")));
            },
            Some(v) => v,
        };
        let version: HttpVersion;

        if version_str == "HTTP/1.1" {
            version = HttpVersion::HTTP_1_1;
        }
        else if version_str == "HTTP/1.0" {
            version = HttpVersion::HTTP_1_0;
        }
        else {
            return Err(ErrAndExpl::new(ResponseCode::HTTP_VERSION_NOT_SUPPORTED_505,
                                       format!("HTTP version '{version_str}' not supported")));
        }

        Ok(Request {
            req_type: method,
            path: String::from(req_path),
            http_version: version,
            content_headers: HashMap::new()
        })
    }
}
