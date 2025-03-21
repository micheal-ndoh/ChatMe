use std::collections::HashMap;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

type Clients = Arc<Mutex<HashMap<String, TcpStream>>>;

pub fn start_server() {
    let listener = TcpListener::bind("127.0.0.1:7878").expect("Failed to bind to port 7878");
    let clients: Clients = Arc::new(Mutex::new(HashMap::new()));

    println!("Server running on 127.0.0.1:7878...");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let clients_clone = Arc::clone(&clients);
                thread::spawn(move || handle_client(stream, clients_clone));
            }
            Err(e) => println!("Connection failed: {}", e),
        }
    }
}

fn handle_client(stream: TcpStream, clients: Clients) {
    let mut reader = BufReader::new(stream.try_clone().expect("Failed to clone stream"));
    let mut username = String::new();

    // Read the username
    if reader.read_line(&mut username).is_err() {
        println!("Failed to read username");
        return;
    }
    username = username.trim().to_string();
    println!("New user connected: {}", username);

    // Store the client connection
    {
        let mut clients_lock = clients.lock().unwrap();
        clients_lock.insert(username.clone(), stream.try_clone().unwrap());
    }

    let mut buffer = String::new();
    while reader.read_line(&mut buffer).is_ok() {
        let message = buffer.trim().to_string();
        if message.is_empty() {
            continue;
        }

        println!("Received message from {}: {}", username, message);

        if message.starts_with('@') {
            // Private message
            let parts: Vec<&str> = message.splitn(2, ' ').collect();
            if parts.len() < 2 {
                continue;
            }
            let target_username = parts[0].trim_start_matches('@');
            let message_content = parts[1];

            let clients_lock = clients.lock().unwrap();
            if let Some(target_stream) = clients_lock.get(target_username) {
                let mut target_stream = target_stream.try_clone().unwrap();
                if let Err(e) = writeln!(
                    target_stream,
                    "[Private from {}]: {}",
                    username, message_content
                ) {
                    println!("Failed to send private message: {}", e);
                }
                let _ = target_stream.flush();
            } else {
                println!("User {} not found for private message", target_username);
            }
        } else {
            // Broadcast message
            let clients_lock = clients.lock().unwrap();
            for (user, client_stream) in clients_lock.iter() {
                if *user != username {
                    let mut client_stream = client_stream.try_clone().unwrap();
                    if let Err(e) = writeln!(client_stream, "[{}]: {}", username, message) {
                        println!("Failed to send message to {}: {}", user, e);
                    }
                    let _ = client_stream.flush();
                }
            }
        }

        buffer.clear();
    }

    // Remove user on disconnect
    {
        let mut clients_lock = clients.lock().unwrap();
        clients_lock.remove(&username);
    }
    println!("{} disconnected", username);
}
