extern crate rt_tcp;

use std::io::prelude::*;
use std::net::TcpListener;
use std::env;

use rt_tcp::common;

fn run_server(addr: &str) {
    println!("Listening on {}", addr);

    if let Ok(listener) = TcpListener::bind(addr) {
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

fn main() {
    let args: Vec<String> = env::args().map(|x| x.to_string()).collect();
    if args.len() != 2 {
        println!("Usage: {} <port>", args[0]);
        return;
    }

    let addr: String = format!("{}:{}", common::HOST, args[1]);

    run_server(&addr);
}