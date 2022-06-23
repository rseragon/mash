use tokio::{net::{TcpListener, TcpStream}, io::{AsyncReadExt, AsyncWriteExt}};
use std::collections::HashMap;

use crate::cliparser::Config;
use crate::utils::stream_utils;
use crate::server::request as request;
use crate::server::response::{Response, ResponseCode};

use super::client_handler::process_request;

pub async fn serve(config: &Config) {
    paris::info!("Started serving [{}:{}]: {}", config.host, config.port, config.path);

    let addr: &str = &format!("{}:{}", config.host, config.port);
    
    let socket = TcpListener::bind(addr).await.unwrap_or_else(|err| {
        paris::error!("Binding to port {} failed!", config.port);
        std::process::exit(-1);
    });

    loop {
        let (sock, sock_addr) = match socket.accept().await {
            Ok((s, sa)) => (s, sa),
            Err(_) => continue // TODO: print failed statement
        };

        paris::info!("New connection: {:?}", sock_addr);

        let config_clone = config.clone();
        tokio::spawn(async {
            // TODO: cloning bad
            process(config_clone, sock).await;
        });
    }

}

async fn process(config: Config, mut sock: TcpStream) {

    let buf = stream_utils::read_to_bytes(&mut sock).await;

    let resp = handle_request(buf, &config).await;

    match stream_utils::write_bytes(&mut sock, resp.build().as_bytes()).await {
        Err(s) => {
            paris::error!("{}", s);
        },
        Ok(_) => {}, // TODO
    };

}

async fn handle_request(buf: Vec<u8>, config: &Config) -> Response {

    let req_res = request::Request::parse(&buf, config);

    // TODO: logging of request
    
    let resp: Response;

    // Check if request is okay
    match req_res {
        Err(ee) => { 
            // TODO: add better html for error
            resp = Response::new(ee.err, ee.expl);
        },
        Ok(req) => {
            resp = process_request(req, config).await;
        }
    }

    resp
}
