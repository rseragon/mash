use tokio::{net::{TcpListener, TcpStream}, io::{AsyncReadExt, AsyncWriteExt}};
use std::collections::HashMap;

use crate::cliparser::Config;
use crate::utils::stream_utils;
use crate::server::request as request;
use crate::server::response::{Response, ResponseCode};

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

        tokio::spawn(async {
            process(sock).await;
        });
    }

}

async fn process(mut sock: TcpStream) {

    let mut buf = stream_utils::read_to_bytes(&mut sock).await;

    let req = match request::Request::parse(& buf) {
        Err(e) => { paris::error!("{e}"); return (); },
        Ok(r) => r,
    };

    // TODO: Loggins of request

    let resp = Response::new(ResponseCode::OK_200, HashMap::new(), String::from("200 Okiew"));

    match stream_utils::write_bytes(&mut sock, resp.build().as_bytes()).await {
        Err(s) => {
            paris::error!("{}", s);
        },
        Ok(_) => {}, // TODO
    };

}
