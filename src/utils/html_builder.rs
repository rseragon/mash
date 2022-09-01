use crate::server::response::ResponseCode;

pub fn error_page_builder(code: &ResponseCode, msg: &String) -> String {

    format!("Error occured: Status code [{}]\n\n<HR>\n\nReason: {}", code.to_string(), msg)

}
