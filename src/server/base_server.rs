use tokio::{net::{TcpListener, TcpStream}, io::{AsyncReadExt, AsyncWriteExt}};
use std::path::Path;
use std::collections::HashMap;

use crate::cliparser::Config;
use crate::utils::stream_utils;
use crate::server::request as request;
use crate::server::response::{Response, ResponseCode};

pub async fn serve(config: &Config) {
    println!("Started serving [{}:{}]: {}", config.host, config.port, config.path);

    let addr: &str = &format!("{}:{}", config.host, config.port);
    
    let socket = TcpListener::bind(addr).await.unwrap_or_else(|err| {
        eprintln!("Binding to port {} failed!", config.port);
        std::process::exit(-1);
    });

    loop {
        let (sock, sock_addr) = match socket.accept().await {
            Ok((s, sa)) => (s, sa),
            Err(_) => continue // TODO: print failed statement
        };

        println!("New connection: {:?}", sock_addr);

        tokio::spawn(async {
            process(sock).await;
        });
    }

}

async fn process(mut sock: TcpStream) {

    let mut buf = stream_utils::read_to_bytes(&mut sock).await;

    let req = match request::Request::parse(& buf) {
        Err(e) => { eprintln!("{e}"); return (); },
        Ok(r) => r,
    };

    println!("{:?}", req);

    let resp = Response::new(ResponseCode::OK_200, HashMap::new(), String::from("200 Okiew"));
    eprintln!("{:?}", resp);
    eprintln!("{}", resp.build());

    match stream_utils::write_bytes(&mut sock, resp.build().as_bytes()).await {
        Err(s) => {
            eprintln!("{}", s);
        },
        Ok(_) => {}, // TODO
    };

    /* let mut headers = [httparse::EMPTY_HEADER; 1024];
    let mut req = httparse::Request::new(&mut headers);

    let res = req.parse(&buf).unwrap_or_else(|err| {
        eprintln!("Failed: {:?}", err);
        httparse::Status::Partial
    });
    println!("Parsed");

    let resp = b"HTTP/1.0 404 File not found\r\n";

    if res.is_complete() {
        match req.path {
            Some(ref path) => {
                println!("one");
                if Path::new(path).exists() {
                    // sock.write(b"HTTP/1.0 200 OK\r\n").await.unwrap();
                    sock.write(b"HTTP/1.0 200 OK\r\n\n200 OK").await.unwrap_or(0);
                }
                else {
                    let mut response = http::Response::builder()
                        .status(http::StatusCode::NOT_FOUND)
                        .body("Not found :(")
                        .unwrap();


                    println!("{:?}", response);
                    sock.write(response.body().as_bytes()).await.unwrap_or(0);
                }
            }
            _ => {
                println!("two");
                sock.write(resp).await.unwrap_or(0);
                // sock.write(b"HTTP/1.0 404 Not Found\r\n").await.unwrap();
            }
        };
    }
    else {
        println!("three");
        sock.write(resp).await.unwrap();
        // sock.write(b"HTTP/1.0 404 Not Found\r\n").await.unwrap();
    }
    sock.flush().await.unwrap_or_else(|err| {
        eprintln!("Failed flush: {:?}", err);
    }); */
}
