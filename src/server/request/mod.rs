use std::collections::HashMap;
pub mod req_funcs;

#[derive(Debug)]
pub enum RequestType {
    GET
}

#[derive(Debug)]
pub enum HttpVersion {
    HTTP_1_0, // HTTP/1.0
    HTTP_1_1, // HTTP/1.1
}

#[derive(Debug)]
pub struct Request {
    req_type: RequestType,
    path: String,
    http_version: HttpVersion,
    content_headers: HashMap<String, String>,
}
