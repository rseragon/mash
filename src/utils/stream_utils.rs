use std::os::unix::prelude::OsStrExt;

use tokio::{net::TcpStream, io::{AsyncReadExt, AsyncWriteExt}};

pub async fn read_to_str(sock: &mut TcpStream) -> String {
    
    let mut read = String::new();

    sock.read_to_string(&mut read).await;

    read
}

pub async fn read_to_bytes(sock: &mut TcpStream) -> Vec<u8> {
    let mut buf = Vec::new();

    loop {
        let mut curr_buf = [0; 4096];
        match sock.read(&mut curr_buf[..]).await {
            Ok(0) => break,
            Ok(n) => {
                if n == 0 {
                    break;
                }
                buf.extend_from_slice(&mut curr_buf[0..n]);
            },
            Err(_) => break, // TODO: error handling
        }

        break; // TODO: This is bad
    }

    buf
}

pub async fn write_bytes(sock: &mut TcpStream, buf: &[u8]) -> Result<(), String> {

    match sock.write(&buf[..]).await {
        Err(err) => return Err(format!("Failed to write: {}", err)),
        Ok(_) => {},
    };

    match sock.flush().await {
        Err(err) => return Err(format!("Failed to flush socket: {}", err)),
        Ok(_) => {},
    }

    Ok(())

}

pub async fn write_string(sock: &mut TcpStream, buf: &str) -> Result<(), String> {

    match sock.write(&buf.as_bytes()[..]).await {
        Err(err) => return Err(format!("Failed to write: {}", err)),
        Ok(_) => {},
    };

    match sock.flush().await {
        Err(err) => return Err(format!("Failed to flush socket: {}", err)),
        Ok(_) => {},
    }

    Ok(())

}
