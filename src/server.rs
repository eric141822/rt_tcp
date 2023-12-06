use std::io::prelude::*;
use crate::common;
use std::net::TcpListener;
use std::sync::mpsc;
pub fn run_server(addr: &str, tx: mpsc::Sender<bool>) {
    println!("Listening on {}", addr);

    if let Ok(listener) = TcpListener::bind(addr) {
        tx.send(true).unwrap();
        println!("Bind to {}", addr);
        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    let mut buf = [0; common::BUF_SIZE];
                    let nbytes = stream.read(&mut buf).unwrap();
                    println!("Received {} bytes", nbytes);
                    println!("{}", String::from_utf8_lossy(&buf[..nbytes]));
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }
    } else {
        println!("Could not bind to address {}", addr);
    }
}