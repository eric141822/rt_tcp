extern crate rt_tcp;

use std::io::prelude::*;
use std::net::TcpStream;
use std::env;

use rt_tcp::common;

fn run_client(addr: &str, msg: &str) {
    println!("Connecting to {}", addr);

    if let Ok(mut stream) = TcpStream::connect(addr) {
        stream.write(msg.as_bytes()).unwrap();
        println!("Sent message: {}", msg);
    } else {
        println!("Could not connect to {}", addr);
    }
}

fn main() {
    let args: Vec<String> = env::args().map(|x| x.to_string()).collect();
    if args.len() != 2 {
        println!("Usage: {} <port>", args[0]);
        return;
    }

    let addr: String = format!("{}:{}", common::HOST, args[1]);


    // get input message from stdin.
    let mut msg = String::new();
    println!("Enter a message: ");
    std::io::stdin().read_line(&mut msg).expect("Failed to parse message.");

    run_client(&addr, &msg.trim());

    msg.clear();
    
    println!("Enter a message: ");
    std::io::stdin().read_line(&mut msg).expect("Failed to parse message.");

    run_client(&addr, &msg);
}