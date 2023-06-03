use tokio::{
    net::TcpListener, 
    io::{AsyncReadExt, AsyncWriteExt}
};

#[tokio::main]
async fn main() {
    // first thing we need a tcp listener
    // listening on incoming requests
    let listener = TcpListener::bind("localhost:8080").await.unwrap();

    // so we have a tcp listener and we are ready to start awaiting for connections
    let (mut socket, _addr) = listener.accept().await.unwrap();

    loop {
        let mut buffer = [0u8; 1024];

        let bytes_read = socket.read(&mut buffer).await.unwrap();

        socket.write_all(&buffer[..bytes_read]).await.unwrap();
    }    
}
