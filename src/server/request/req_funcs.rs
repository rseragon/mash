use std::collections::HashMap;

use super::{Request, RequestType, HttpVersion};

impl Request {

    // TODO: parse content reqeust headers
    pub fn parse(buf: &[u8]) -> Result<Request, String> {
        let buf_str = match std::str::from_utf8(buf) {
            Ok(res) => res,
            Err(_) => { return Err(format!("Failed to parse request")); },
        };

        let mut iter = buf_str.split_ascii_whitespace();

        // EG:
        // GET  /  HTTP/1.1
        // (1) (2)  (3)

        // (1) Method
        let method_str = match iter.next() {
            None => return Err(format!("Illegal reqeust format: request method not found")),
            Some(met) => met,
        };
        let method: RequestType;

        if method_str == "GET" {
            method = RequestType::GET;
        }
        else {
            return Err(format!("Unkown request type: {}", method_str));
        }

        // (2) request path
        let req_path = match iter.next() {
            None => return Err(format!("Illegal path format: request doesn't contain path")),
            Some(p) => p,
        };

        // (3) HTTP version
        let version_str = match iter.next() {
            None => return Err(format!("Illegal version format: request http version not found")),
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
            return Err(format!("Unknown http version: {}", version_str));
        }

        Ok(Request {
            req_type: method,
            path: String::from(req_path),
            http_version: version,
            content_headers: HashMap::new()
        })
    }
}
