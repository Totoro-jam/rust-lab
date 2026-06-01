//! Async TCP echo server using tokio.

use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};

/// Starts an echo server on the given address. Returns the bound address.
/// The server runs until the provided shutdown signal resolves.
pub async fn run_echo_server(
    addr: &str,
    shutdown: tokio::sync::oneshot::Receiver<()>,
) -> std::io::Result<std::net::SocketAddr> {
    let listener = TcpListener::bind(addr).await?;
    let local_addr = listener.local_addr()?;

    tokio::spawn(async move {
        tokio::select! {
            _ = accept_loop(listener) => {}
            _ = shutdown => {}
        }
    });

    Ok(local_addr)
}

async fn accept_loop(listener: TcpListener) {
    while let Ok((stream, _addr)) = listener.accept().await {
        tokio::spawn(handle_connection(stream));
    }
}

async fn handle_connection(stream: TcpStream) {
    let (reader, mut writer) = stream.into_split();
    let mut buf_reader = BufReader::new(reader);
    let mut line = String::new();

    loop {
        line.clear();
        match buf_reader.read_line(&mut line).await {
            Ok(0) => break, // connection closed
            Ok(_) => {
                if writer.write_all(line.as_bytes()).await.is_err() {
                    break;
                }
            }
            Err(_) => break,
        }
    }
}

/// Demonstrates tokio::join! for concurrent operations.
pub async fn fetch_parallel() -> (String, String) {
    let (a, b) = tokio::join!(
        async {
            tokio::time::sleep(std::time::Duration::from_millis(10)).await;
            String::from("result_a")
        },
        async {
            tokio::time::sleep(std::time::Duration::from_millis(10)).await;
            String::from("result_b")
        }
    );
    (a, b)
}

/// Demonstrates tokio::select! for racing futures.
pub async fn first_to_finish() -> &'static str {
    tokio::select! {
        _ = tokio::time::sleep(std::time::Duration::from_millis(50)) => {
            "slow"
        }
        _ = tokio::time::sleep(std::time::Duration::from_millis(10)) => {
            "fast"
        }
    }
}
