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
    body: Vec<u8>, // This should be bytes
}

impl ToString for ResponseCode {
    fn to_string(&self) -> String {

        let status = match self {
            Ok200 => "200 OK",
            BadRequest400 => "400 Bad Request",
            Forbidden403 => "403 Forbidden",
            NotFound404 => "404 Not Found",
            InternalServerError500 => "500 Internal Server Error",
            NotImplemented501 => "501 Not Implemented",
            HttpVersionNotSupported505 => "505 HTTP Version Not Supported",
        }.to_string();

        return status;
    }
}
