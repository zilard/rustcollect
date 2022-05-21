
// std::prelude 
// is the list of things that Rust automatically imports into 
// every Rust program. It’s kept as small as possible, 
// and is focused on things, particularly traits, 
// which are used in almost every single Rust program.


// srd::io::prelude
// The purpose of this module is to alleviate imports of 
// many common I/O traits by adding a glob import to 
// the top of I/O heavy modules: use std::io::prelude::*;




// std::net Networking primitives for TCP/UDP communication.
// This module provides networking functionality for the 
// Transmission Control and User Datagram Protocols, 
// as well as types for IP and socket addresses.

use std::io::prelude::*;
use std::net;



fn cheapo_request(host: &str, port: u16, path: &str) 
                      -> std::io::Result<String>
{

    // A TCP stream between a local and a remote socket.
    // pub fn connect<A: ToSocketAddrs>(addr: A) -> Result<TcpStream>

    let mut socket = net::TcpStream::connect((host, port))?;


    // A common use for format! is concatenation and interpolation of strings
    // format! panics if a formatting trait implementation returns an error
    let request = format!("GET {} HTTP/1.1\r\nHost: {}\r\n\r\n", 
                          path, host);

    // write_all - Attempts to write an entire buffer into this writer -> socket
    // as_bytes - Converts a string slice to a byte slice. 
    socket.write_all(request.as_bytes())?;


    // pub fn shutdown(&self, how: Shutdown) -> Result<()>
    // Shuts down the read, write, or both halves of this connection."
    // std::net::Shutdown
    //
    //   pub enum Shutdown {
    //       Read,
    //       Write,
    //       Both,
    //   }
    //
    //   Write -The writing portion of the TcpStream should be shut down.
    //   All currently blocked and future writes will return an error.

    socket.shutdown(net::Shutdown::Write)?;


    let mut response = String::new();

    // read_to_string - Read the entire contents of a file into a string
    socket.read_to_string(&mut response)?;

    Ok(response)

}



fn main() -> std::io::Result<()> {


    let response = cheapo_request("example.com", 80, "/")?;

    println!("{}", response);

    Ok(())

}



