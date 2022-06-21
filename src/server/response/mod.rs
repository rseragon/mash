use std::collections::HashMap;

mod resp;

#[derive(Debug)]
pub enum ResponseCode {
    OK_200,
    NOT_FOUND_404,
    INTERNAL_SERVER_ERROR_500,
    NOT_IMPLEMENTED_501
}

#[derive(Debug)]
pub struct Response {
    status_code: ResponseCode,
    headers: HashMap<String, String>,
    body: String
}
