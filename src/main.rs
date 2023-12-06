use std::{env, sync::mpsc, thread};
mod client;
mod common;
mod server;

fn main() {
    let args: Vec<String> = env::args().map(|x| x.to_string()).collect();
    if args.len() != 2 {
        println!("Usage: {} <port>", args[0]);
        return;
    }

    let addr: String = format!("{}:{}", common::HOST, args[1]);
    let addr2 = addr.clone();

    // Create a channel
    let (tx, rx) = mpsc::channel();

    // thread a server;
    thread::spawn(move || {
        server::run_server(&addr[..], tx);
    });

    rx.recv().unwrap();

    client::run_client(&addr2[..]);
}
