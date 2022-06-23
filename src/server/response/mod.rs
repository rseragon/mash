use std::collections::HashMap;

mod resp;

#[derive(Debug)]
pub enum ResponseCode {
    // Okeay!!
    OK_200,

    // Errors :(
    BAD_REQUEST_400,
    FORBIDDEN_403,
    NOT_FOUND_404,
    INTERNAL_SERVER_ERROR_500,
    NOT_IMPLEMENTED_501,
    HTTP_VERSION_NOT_SUPPORTED_505,
}

#[derive(Debug)]
pub struct Response {
    status_code: ResponseCode,
    headers: HashMap<String, String>,
    body: String // This should be bytes
}
