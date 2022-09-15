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

    // pub fn set_status_code(&mut self, code: ResponseCode) {
    //     self.status_code = code;
    // }

    pub fn set_headers(&mut self, headers: HashMap<&'static str, String>) {
        self.headers = headers;
    }
    pub fn set_body_array(&mut self, body: &[u8]) {
        let body = body.to_vec();
        self.body = body;
    }

    // pub fn set_body_vec(&mut self, body: Vec<u8>) {
    //     self.body = body;
    // }
    //
    pub fn modify_header(&mut self, key: &'static str, value: String) {
        self.headers.insert(key, value);
    }

    pub fn build_header(&mut self) -> String {

        // EG: (1) 200 OK 
        let code_str = self.status_code.to_string();


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


    pub async fn send_resp(&mut self, sock: &mut TcpStream) -> Result<(), String> {

        // send headers
        if let Err(e) = stream_utils::write_bytes(sock, self.build_header().as_bytes()).await {
            paris::error!("Failed to send headers: {}", e);
        }

        // send body
        if let Err(e) = stream_utils::write_bytes(sock, &self.body[..]).await {
            paris::error!("Failed to send body: {}", e);
        }

        Ok(())
    }
}

