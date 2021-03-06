use tokio::net::TcpStream;
use super::{Response, ResponseCode};
use std::collections::HashMap;
use crate::utils::stream_utils;

impl Response {
    pub fn new(status_code: ResponseCode, body: Vec<u8>) -> Response {
        Response {
            status_code,
            headers: HashMap::new(),
            body
        }
    }

    pub fn set_status_code(&mut self, code: ResponseCode) {
        self.status_code = code;
    }

    pub fn set_headers(&mut self, headers: HashMap<&'static str, String>) {
        self.headers = headers;
    }
    pub fn set_body_array(&mut self, body: &[u8]) {
        let body = body.to_vec();
        self.body = body;
    }

    pub fn set_body_vec(&mut self, body: Vec<u8>) {
        self.body = body;
    }

    pub fn build_header(&mut self) -> String {
        use ResponseCode::*;

        // EG: (1) 200 OK 
        let code_str = match self.status_code {
            Ok200 => "200 OK",
            BadRequest400 => "400 Bad Request",
            Forbidden403 => "403 Forbidden",
            NotFound404 => "404 Not Found",
            InternalServerError500 => "500 Internal Server Error",
            NotImplemented501 => "501 Not Implemented",
            HttpVersionNotSupported505 => "505 HTTP Version Not Supported",
        };


        // EG: (2) Connection: Keep-Alive, Content-Length
        // Calcualte few headers
        
        /* TODO: Setting content len is mearking files not to download
        let body_size = self.body.len().to_string();
        self.headers.insert("Content-Length", body_size);
        */

        let mut headers_str = String::new();
        for (header, val) in self.headers.iter() {
            headers_str.push_str(&format!("{}: {}\r\n", header, val)[..]);
        }

        /* Response format
         * EG:
         * HTTP/1.1 200 OK
         * Connection: Keep-Alive
         * ...
         * Content-Length: (len)
         *
         * (body)
         * // Content-Length and body will be sent later
         */

        // the headers_str will leave \r\n at the end
        // this will be continued by Content-Length
        
        let response = format!("\
        {} {}\r\n\
        {}\r\n", "HTTP/1.1", code_str, headers_str);

        // TODO: Shoulw I hard code HTTP/1.1 ?

        // Servers compatible with HTTP/1.1 should response with HTTP/1.1
        // to all requests pertaining lower than 1.1
        // https://stackoverflow.com/questions/57334986/why-is-http-version-number-presented-in-both-a-request-line-and-a-status-line

        response
    }


    pub async fn send_resp(&mut self, mut sock: &mut TcpStream) -> Result<(), String> {

        // send header
        match stream_utils::write_bytes(&mut sock, self.build_header().as_bytes()).await {
            Err(e) => {
                paris::error!("{}", e);
            },
            Ok(_) => {}, // TODO
        };

        // Sleep for a sec
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;

        // send body
        match stream_utils::write_bytes(&mut sock, &self.body[..]).await {
            Err(e) => {
                paris::error!("{}", e);
            },
            Ok(_) => {}, // TODO
        };

        // TODO: Need to stop here and confirm that the file is sent

        Ok(())
    }
}

