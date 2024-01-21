extern crate rt_tcp;

use rt_tcp::common;
use std::env;
use std::io::prelude::*;
use std::net::TcpStream;

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

    let mut msg = String::new();
    loop {
        println!("Enter a message, or EXIT to quit: ");
        if let Err(e) = std::io::stdin().read_line(&mut msg) {
            eprintln!("Error: {}", e);
            continue;
        }

        let trimmed_msg = msg.trim();

        // equal ignore case.
        if trimmed_msg.eq_ignore_ascii_case("EXIT") {
            break;
        }

        run_client(&addr, &trimmed_msg);
        msg.clear();
    }
}
