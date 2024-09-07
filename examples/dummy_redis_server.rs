use anyhow::Result;
use std::io;
use tokio::net::{TcpListener, TcpStream};
use tracing::{error, info};

const BUF_SIZE: usize = 1024 * 4;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    // 构建 一个 tcp listener
    let addr = "0.0.0.0:6379";

    let listener = TcpListener::bind(addr).await?;
    info!("dummy redis listening on: {}", addr);

    // 不断的从 socket 中拿数据
    loop {
        let (stream, addr) = listener.accept().await?;

        info!("Accepted connection from {}", addr);
        tokio::spawn(async move {
            if let Err(e) = process_redis_connection(stream).await {
                error!("Process Redis connection error: {}", e);
            }
        });
    }
}

async fn process_redis_connection(stream: TcpStream) -> Result<()> {
    loop {
        // 如果不是 readable 就会一直等在这里，直到可以读
        stream.readable().await?;

        let mut buf = Vec::with_capacity(BUF_SIZE);

        match stream.try_read_buf(&mut buf) {
            Ok(0) => break,
            Ok(n) => {
                info!("read {} bytes", n);
                let line = String::from_utf8_lossy(&buf);
                info!("read line from connection: {:?}", line);
            }

            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => {
                return Err(e.into());
            }
        }
    }

    Ok(())
}
