use std::io::prelude::*;
use std::net::TcpStream;

pub fn run_client_hello(addr: &str) {
    println!("Connecting to {}", addr);

    if let Ok(mut stream) = TcpStream::connect(addr) {
        let msg = "Hello, world!";
        stream.write(msg.as_bytes()).unwrap();
        println!("Sent message: {}", msg);
    } else {
        println!("Could not connect to {}", addr);
    }
}

pub fn run_client(addr: &str, msg: &str) {
    println!("Connecting to {}", addr);

    if let Ok(mut stream) = TcpStream::connect(addr) {
        stream.write(msg.as_bytes()).unwrap();
        println!("Sent message: {}", msg);
    } else {
        println!("Could not connect to {}", addr);
    }
}