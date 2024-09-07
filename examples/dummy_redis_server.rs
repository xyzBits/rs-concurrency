use anyhow::Result;
use std::io;
use std::net::SocketAddr;
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, TcpStream};
use tracing::{error, info, warn};

const BUF_SIZE: usize = 1024 * 4;

#[tokio::main] // 为程序引入运行时
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    // 构建 一个 tcp listener
    let addr = "0.0.0.0:6379";

    let listener = TcpListener::bind(addr).await?;
    info!("dummy redis listening on: {}", addr);

    // 不断的从 socket 中拿数据
    loop {
        let (stream, client_addr) = listener.accept().await?;

        info!("Accepted connection from {}", client_addr);

        // 不能直接 loop stream，否则主线程就会被卡住，没法处理新的 stream
        // tokio task ，由 tokio 运行时去管理这个 task
        tokio::spawn(async move {
            if let Err(e) = process_redis_connection(stream, client_addr).await {
                error!("Process Redis connection error: {}", e);
            }
        });
    }
}

// addr 是可以 copy 的，因为他是一个 Enum 数据结构非常小
async fn process_redis_connection(mut stream: TcpStream, client_addr: SocketAddr) -> Result<()> {
    loop {
        // 如果不是 readable 就会一直等在这里，直到可以读
        stream.readable().await?;

        let mut buf = Vec::with_capacity(BUF_SIZE);

        match stream.try_read_buf(&mut buf) {
            // 证明数据接收结束了，0 代表 EOF end of file，socket 中没有数据了
            Ok(0) => break,
            Ok(n) => {
                info!("read {} bytes", n);
                let line = String::from_utf8_lossy(&buf);
                info!("read line from connection: {:?}", line);
                stream.write_all(b"OK\r\n").await?;
            }

            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => {
                return Err(e.into());
            }
        }
    }

    warn!("Connection: {} closed", client_addr);
    Ok(())
}
