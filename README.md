# ChatMe - A Simple Chat Application in Rust

<table >
    <thead align="center">
        <tr border: 1px;>
            <td><b>🐛 Issues</b></td>
            <td><b>🔔 Open PRs</b></td>
            <td><b>🔕 Closed PRs</b></td>
        </tr>
     </thead>
    <tbody>
         <tr>
            <td><img alt="Issues" src="https://img.shields.io/github/issues/Dericko681/ChatMe?style=flat&logo=github"/></td>
            <td><img alt="Open Pull Requests" src="https://img.shields.io/github/issues-pr/Dericko681/ChatMe?style=flat&logo=github"/></td>
           <td><img alt="Close Pull Requests" src="https://img.shields.io/github/issues-pr-closed/Dericko681/ChatMe?style=flat&color=critical&logo=github"/></td>
        </tr>
    </tbody>
</table>



ChatMe is a simple chat application written in Rust that demonstrates how to use threads and channels for communication between senders and receivers. The application consists of two parts:

Local Chat (Threading version): A multi-threaded, single-process implementation where messages are sent between sender and receiver threads within the same machine.
Distributed Chat (Networking version): A TCP-based version where a client sends messages to a server, and the server receives and displays the messages. This version allows communication between different machines over a network.

## Table of Contents

    Project Overview
    Technologies Used
    Local Chat (Threading)
        Features
        Setup Instructions
        Running the Application
    Distributed Chat (Networking)
        Features
        Setup Instructions
        Running the Server
        Running the Client
    Contributing
    License

## Project Overview

The ChatMe project is designed to demonstrate basic concepts around multi-threading and communication protocols in Rust. It consists of two main versions:

Local Chat (Threading version): Both the sender and receiver are running in separate threads on the same machine and communicate through Rust's mpsc (multi-producer, single-consumer) channels.
Distributed Chat (Networking version): Uses TCP sockets for communication between a client (sender) and a server (receiver), where the client can send messages to the server over a network connection.

This project is intended for learning and experimentation with Rust's concurrency and networking capabilities.
Technologies Used

Rust: The programming language used to implement the entire chat application.
Standard Library: Utilized features from Rust's standard library including std::thread, std::sync::mpsc, and std::net for threading, channels, and networking respectively.

## Local Chat (Threading)

In this version, both the sender and receiver are running as separate threads within the same process. They communicate through an in-memory channel (mpsc::channel), simulating a real-time message exchange.
Features
Multi-threaded design with sender and receiver running on separate threads.
Communication between threads is achieved using Rust channels (mpsc).
Simple message passing using a Message struct that contains a sender's name and message cont

## Setup Instructions

Clone the Repository:

```
git clone https://github.com/Dericko681/ChatMe.git
cd chatme
```

Install Rust (if not already installed): Follow the official instructions: https://www.rust-lang.org/learn/get-started

Build the Project: Inside the project directory, run the following command to build the project:

```
cargo build
```

Run the Application: To run the local chat version, simply run the following command:

```
cargo run
```

This will start the application with multiple sender threads, each sending messages to a receiver thread.

Running the Application

When you run the program, it will simulate two sender threads sending messages to a receiver thread. The output will look like this:

Sent message: Hello from Sender 1
Sent message: Second message from Sender 1
Received message from Sender 1: Hello from Sender 1
Received message from Sender 1: Second message from Sender 1
Sent message: Hello from Sender 2
Sent message: Second message from Sender 2
Received message from Sender 2: Hello from Sender 2
Received message from Sender 2: Second message from Sender 2

## Distributed Chat (Networking)

In this version, a server listens for incoming connections from clients. A client connects to the server and sends messages, which are then displayed by the server.
Features

    Server listens for incoming connections on a specified IP and port.
    Client connects to the server and sends messages.
    Communication between the client and server is done using TCP sockets.

## Setup Instructions


Run the Server: In one terminal window, run the server (this will listen for incoming connections on port 7878):

cargo run --bin server

Run the Client: In a separate terminal window, run the client to send messages to the server. Make sure the server is running first!

    cargo run --bin client

 The client will connect to the server on 127.0.0.1:7878 by default and send a message. The server will print the received message.

Running the Server

    The server listens on port 7878 by default and can accept multiple client connections.
    It will spawn a new thread to handle each incoming connection and print the received messages.

Running the Client

    The client connects to the server using TcpStream::connect().
    It will prompt the user to input a message and send it to the server.
    After sending the message, the client will exit.

## Contributing

We welcome contributions! To contribute to ChatMe, follow these steps:

    Fork the repository.
    Create a new branch for your feature (git checkout -b feature/your-feature).
    Make your changes.
    Commit your changes (git commit -am 'Add new feature').
    Push to your branch (git push origin feature/your-feature).
    Open a pull request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
Additional Notes

    If you're looking to extend this project, consider implementing advanced features like message encryption, user authentication, or saving chat history to a database.
    You can also experiment with adding a GUI using libraries like gtk-rs or crossterm for terminal-based UIs.
