use std::collections::HashMap;

mod resp;

#[derive(Debug)]
pub enum ResponseCode {
    // Okeay!!
    Ok200,

    // Errors :(
    BadRequest400,
    Forbidden403,
    NotFound404,
    InternalServerError500,
    NotImplemented501,
    HttpVersionNotSupported505,
}

#[derive(Debug)]
pub struct Response {
    status_code: ResponseCode,
    headers: HashMap<&'static str, String>,
    body: Vec<u8> // This should be bytes
}
