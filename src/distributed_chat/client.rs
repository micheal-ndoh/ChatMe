use std::io::{self, BufRead, BufReader, Write};
use std::net::TcpStream;
use std::thread;

pub fn start_client() {
    let mut stream = TcpStream::connect("127.0.0.1:7878").expect("Failed to connect to server");

    println!("Enter your username: ");
    let mut username = String::new();
    io::stdin()
        .read_line(&mut username)
        .expect("Failed to read username");
    username = username.trim().to_string();

    // Send username to server
    writeln!(stream, "{}", username).expect("Failed to send username");
    stream.flush().expect("Failed to flush username");

    let mut stream_clone = stream.try_clone().expect("Failed to clone stream");

    // Spawn thread to listen for messages from the server
    thread::spawn(move || {
        let reader = BufReader::new(&mut stream_clone);
        for line in reader.lines() {
            match line {
                Ok(msg) => println!("{}", msg),
                Err(_) => {
                    println!("Disconnected from server.");
                    break;
                }
            }
        }
    });

    // Read input from user and send messages
    loop {
        let mut message = String::new();
        io::stdin()
            .read_line(&mut message)
            .expect("Failed to read input");
        let message = message.trim().to_string();

        if message == "/exit" {
            println!("Disconnecting...");
            break;
        }

        writeln!(stream, "{}", message).expect("Failed to send message");
        stream.flush().expect("Failed to flush message");
    }
}
