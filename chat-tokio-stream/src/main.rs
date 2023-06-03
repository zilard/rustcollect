use tokio::{
    net::TcpListener, 
    io::{AsyncWriteExt, BufReader, AsyncBufReadExt}
};

#[tokio::main]
async fn main() {
    // first thing we need a tcp listener
    // listening on incoming requests
    let listener = TcpListener::bind("localhost:8080").await.unwrap();

    // so we have a tcp listener and we are ready to start awaiting for connections
    let (mut socket, _addr) = listener.accept().await.unwrap();

    // tokio has a method that allows to separate the read part and the write part of the socket
    let (reader, mut writer) = socket.split();

    let mut reader = BufReader::new(reader);
    let mut line = String::new();

    loop {
        let bytes_read = reader.read_line(&mut line).await.unwrap();
        if bytes_read == 0 {
            break;
        }

        writer.write_all(line.as_bytes()).await.unwrap();
    }    
}
