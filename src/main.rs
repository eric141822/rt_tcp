use std::{env, sync::mpsc, thread, io};
mod client;
mod common;
mod server;

fn main() {
    let args: Vec<String> = env::args().map(|x| x.to_string()).collect();
    if args.len() != 2 {
        println!("Usage: {} <port>", args[0]);
        return;
    }

    let base_addr: String = format!("{}:{}", common::HOST, args[1]);
    let addr = base_addr.clone();

    // Create a channel
    let (tx, rx) = mpsc::channel();

    // thread a server;
    thread::spawn(move || {
        server::run_server(&addr, tx);
    });

    rx.recv().unwrap();

    // shadow addr.
    let addr = base_addr.clone();
    
    client::run_client_hello(&addr);

    // get input message from stdin.
    let mut msg = String::new();
    println!("Enter a message: ");
    io::stdin().read_line(&mut msg).expect("Failed to parse message.");

    client::run_client(&addr, &msg);
}
