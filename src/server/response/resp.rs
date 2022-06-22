use super::{Response, ResponseCode};
use std::collections::HashMap;

impl Response {
    pub fn new(status_code: ResponseCode, body: String) -> Response {
        Response {
            status_code,
            headers: HashMap::new(),
            body
        }
    }

    pub fn set_status_code(&mut self, code: ResponseCode) {
        self.status_code = code;
    }

    pub fn set_headers(&mut self, headers: HashMap<String, String>) {
        self.headers = headers;
    }
    pub fn set_body(&mut self, body: String) {
        self.body = body;
    }

    pub fn build(&self) -> String {
        use ResponseCode::*;

        // EG: (1) 200 OK 
        let code_str = match self.status_code {
            OK_200 => "200 OK",
            BAD_REQUEST_400 => "400 Bad Request",
            NOT_FOUND_404 => "404 Not Found",
            INTERNAL_SERVER_ERROR_500 => "500 Internal Server Error",
            NOT_IMPLEMENTED_501 => "501 Not Implemented",
            HTTP_VERSION_NOT_SUPPORTED_505 => "505 HTTP Version Not Supported",
        };


        // EG: (2) Connection: Keep-Alive
        let mut headers_str = String::new();
        for (header, val) in self.headers.iter() {
            headers_str.push_str(&format!("{}: {}\r\n", header, val)[..]);
        }

        // EG: (3) Content-Length
        let body_size = self.body.as_bytes().len();
        let content_len = format!("Content-Length: {}", body_size);

        /* Response format
         * EG:
         * HTTP/1.1 200 OK
         * Connection: Keep-Alive
         * ...
         * Content-Length: (len)
         *
         * (body)
         */

        // {}{} is concatinated as, the headers_str will leave \r\n at the end
        // this will be continued by Content-Length
        // \r\n\r\n is cuz headers and body should have 1 line space
        
        let response = format!("\
        {} {}\r\n\
        {}{}\r\n\r\n\
        {}", "HTTP/1.1", code_str, headers_str, content_len, self.body);

        // TODO: Shoulw I hard code HTTP/1.1 ?

        // Servers compatible with HTTP/1.1 should response with HTTP/1.1
        // to all requests pertaining lower than 1.1
        // https://stackoverflow.com/questions/57334986/why-is-http-version-number-presented-in-both-a-request-line-and-a-status-line


        response
    }
}

