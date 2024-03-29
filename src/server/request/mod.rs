use std::collections::HashMap;
pub mod req_funcs;

#[derive(Debug, PartialEq)]
pub enum RequestType {
    GET,
    POST
}

#[derive(Debug)]
pub enum HttpVersion {
    Http1_0, // HTTP/1.0
    Http1_1, // HTTP/1.1
}

#[derive(Debug)]
pub struct Request {
    pub req_type: RequestType,
    pub path: String,
    pub http_version: HttpVersion,
    pub content_headers: HashMap<String, String>,

    pub arguments: HashMap<String, String>,
    pub extra_data: String,
}
