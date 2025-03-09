mod distributed_chat;
mod local_chat;
mod utils;

use distributed_chat::client::start_client;
use distributed_chat::server::start_server;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: cargo run -- (server | client)");
        return;
    }

    let mode = args[1].as_str();
    match mode {
        "server" => start_server(),
        "client" => start_client(),
        _ => eprintln!("Invalid argument! Use 'server' or 'client'"),
    }
}
