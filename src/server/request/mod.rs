use std::collections::HashMap;
pub mod req_funcs;

#[derive(Debug)]
pub enum RequestType {
    GET
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
}

