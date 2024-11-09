use anyhow::Result;
use std::net::SocketAddr;
use tokio::{
    io::AsyncWriteExt,
    net::{TcpListener, TcpStream},
};
use tracing::{info, warn};

const BUFF_SIZE: usize = 1024;
#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let addr = SocketAddr::from(([127, 0, 0, 1], 6379));
    let listiner = TcpListener::bind(&addr).await?;
    info!("Listening on: {}", addr);
    loop {
        let (socket, _) = listiner.accept().await?;
        info!("Accepted connection from: {}", socket.peer_addr()?);
        tokio::spawn(async move {
            if let Err(e) = process_redis_conn(socket).await {
                warn!("Error processing connection: {:?}", e);
            }
        });
    }
    #[allow(unreachable_code)]
    Ok(())
}

async fn process_redis_conn(mut stream: TcpStream) -> Result<()> {
    loop {
        stream.readable().await?;
        let mut buf = Vec::with_capacity(BUFF_SIZE);
        match stream.try_read_buf(&mut buf) {
            Ok(0) => break,
            Ok(n) => {
                info!("Received {} bytes", n);
                info!("Received content: {:?}", &buf[..]);
                info!("Received content: {:?}", String::from_utf8_lossy(&buf[..n]));
                stream.write_all(b"+OK\r\n").await?;
            }
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => return Err(e.into()),
        }
    }
    warn!("Connection {:?} closed", stream.peer_addr()?);
    Ok(())
}
