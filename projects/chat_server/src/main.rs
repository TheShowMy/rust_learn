use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
    signal,
    sync::broadcast,
};
use tokio_util::sync::CancellationToken;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("localhost:8080").await.unwrap();
    let (tx, _) = broadcast::channel(10);
    let token = CancellationToken::new();
    let cancel_token = token.clone();
    tokio::spawn(async move {
        match signal::ctrl_c().await {
            Ok(_) => {
                println!("关闭服务器...");
                cancel_token.cancel();
            }
            Err(e) => {
                eprintln!("Failed to listen for Ctrl+C: {}", e);
            }
        }
    });
    loop {
        let token = token.clone();
        let tx = tx.clone();
        let mut rx = tx.subscribe();
        let (mut socket, addr) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            let (stream_reader, mut stream_writer) = socket.split();
            let mut message = String::new();
            let mut reader = BufReader::new(stream_reader);
            loop {
                tokio::select! {
                    result = reader.read_line(&mut message) => {
                        if result.unwrap() == 0 {
                            break;
                        }
                        tx.send((message.clone(), addr)).unwrap();
                        message.clear();
                    }
                    result = rx.recv() => {
                        let (msg, sender_addr) = result.unwrap();
                        if sender_addr != addr {
                            stream_writer.write_all(msg.as_bytes()).await.unwrap();
                        }
                    }
                    _ = token.cancelled() => {
                        println!("Connection with {} closed.", addr);
                        return;
                    }
                }
            }
        });
    }
}
