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
    // put everything into an infinite loop
    loop {
        let (mut socket, _addr) = listener.accept().await.unwrap();

        // if we want to handle multiple clients independently
        // tokio spawn moves all of one client handling under its own independent task
        // async move wrapping up one little piece of code under its own future, tis own unit of async work
        // we could write this as a separate function handling socket, but its not necessary we can use this async block and pass to tokio spawn
        tokio::spawn(async move {
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
                line.clear();
            }  
        });    
    }     
}
