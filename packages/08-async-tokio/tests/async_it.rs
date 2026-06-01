//! Integration tests for async functionality.

use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;

use rustlab08::channels::run_worker_pool;
use rustlab08::echo_server::{fetch_parallel, first_to_finish, run_echo_server};

#[tokio::test]
async fn echo_server_roundtrip() {
    let (shutdown_tx, shutdown_rx) = tokio::sync::oneshot::channel();
    let addr = run_echo_server("127.0.0.1:0", shutdown_rx).await.unwrap();

    let mut stream = TcpStream::connect(addr).await.unwrap();
    let (reader, mut writer) = stream.split();
    let mut buf_reader = BufReader::new(reader);

    writer.write_all(b"hello\n").await.unwrap();
    let mut line = String::new();
    buf_reader.read_line(&mut line).await.unwrap();
    assert_eq!(line, "hello\n");

    line.clear();
    writer.write_all(b"world\n").await.unwrap();
    buf_reader.read_line(&mut line).await.unwrap();
    assert_eq!(line, "world\n");

    shutdown_tx.send(()).ok();
}

#[tokio::test]
async fn join_parallel_execution() {
    let (a, b) = fetch_parallel().await;
    assert_eq!(a, "result_a");
    assert_eq!(b, "result_b");
}

#[tokio::test]
async fn select_first_wins() {
    let winner = first_to_finish().await;
    assert_eq!(winner, "fast");
}

#[tokio::test]
async fn worker_pool_processes_all() {
    let items = vec!["hello".to_string(), "world".to_string(), "rust".to_string()];

    let mut results = run_worker_pool(items, 2).await;
    results.sort_by(|a, b| a.input.cmp(&b.input));

    assert_eq!(results.len(), 3);
    assert_eq!(results[0].output, "processed: HELLO");
    assert_eq!(results[1].output, "processed: RUST");
    assert_eq!(results[2].output, "processed: WORLD");
}
