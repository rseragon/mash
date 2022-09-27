use tokio::{net::TcpStream, io::{AsyncReadExt, AsyncWriteExt}};

// TODO: This might not work everytime, as `read` is only meant to read 
// once from the stream
pub async fn read_to_bytes(sock: &mut TcpStream) -> Vec<u8> {
    let mut buf = Vec::new();

    let mut curr_buf = [0; 4096];
    match sock.read(&mut curr_buf).await {
        Ok(0) => return buf,
        Ok(n) => {
            buf.extend_from_slice(&curr_buf[0..n]);
        },
        Err(_) => {}, // TODO: error handling
    }

    buf
}

pub async fn write_bytes(sock: &mut TcpStream, buf: &[u8]) -> Result<(), String> {

    if let Err(err) = sock.write_all(buf).await {
        return Err(format!("Failed to write: {}", err));
    }

    if let Err(err) = sock.flush().await {
        return Err(format!("Failed to flush socket: {}", err));
    }
    // match sock.flush().await {
    //     Err(err) => return Err(format!("Failed to flush socket: {}", err)),
    //     Ok(_) => {},
    // }

    Ok(())

}
