use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, TcpStream};

use crate::cliparser::Config;
use crate::utils::stream_utils;
use crate::server::request as request;
use crate::server::response::Response;
use crate::utils::html_builder;

use super::client_handler::process_request;

pub async fn serve(config: &Config) {
    paris::info!("Started serving [{}:{}]: {}", config.host, config.port, config.path);

    let addr: &str = &format!("{}:{}", config.host, config.port);
    
    let socket = TcpListener::bind(addr).await.unwrap_or_else(|err| {
        paris::error!("Binding to port {} failed!: {}", config.port, err);
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

    let mut resp = handle_request(buf, &config).await;

    match resp.send_resp(&mut sock).await {
        Ok(_) => {},
        Err(e) => {
            let peer_addr = match sock.peer_addr() {
                Ok(saddr) => saddr.to_string(),
                Err(_) => "Failed to get peer's socket addr".to_string(),
            };
            paris::error!("Failed to respond to request {}: {}", peer_addr, e);
        }
    }

    sock.flush().await.unwrap_or_else(|err| {
        paris::error!("Failed to flush socket: {}", err);
    });
}

async fn handle_request(buf: Vec<u8>, config: &Config) -> Response {

    let req_res = request::Request::parse(&buf, config);

    // TODO: logging of request
    
    let resp: Response;

    // Check if request is okay
    match req_res {
        Err(ee) => { 
            // TODO: add better html for error
            // resp = Response::new(ee.err, ee.expl.as_bytes().to_vec());
            let body = html_builder::error_page_builder(&ee.err, &ee.expl).as_bytes().to_vec();
            return Response::new(ee.err, body);
        },
        Ok(req) => {
            paris::log!("{:?} {}", req.req_type, req.path);
            resp = process_request(req, config).await;
        }
    }

    resp
}
