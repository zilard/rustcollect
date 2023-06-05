use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net;
use std::str;

#[tokio::main]
async fn main() {
    let host = "localhost:8080";

    // Start up out TCP server
    let srv = net::TcpListener::bind(host).await.unwrap();

    loop {
        // Accept a new connection
        let (mut sock, _) = srv.accept().await.unwrap();

        // Spawn a new taks to handle the connection]
        tokio::spawn(async move {
            let mut buf = [0; 1024];
            // Read data
            let n = sock.read(&mut buf).await.unwrap();
            // Write data
            sock.write_all(&buf[0..n]).await.unwrap();

            let data = str::from_utf8(&buf[0..n]).unwrap();
            println!("Echoed: {:?}", data);
            // Close the socket
            sock.shutdown().await.unwrap();
        });
    }
}
