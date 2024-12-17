// Importing necessary modules from the Rust libraries
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) {
    // 1024-byte sized buffer to read data from client
    let mut buffer = [0; 1024];

    // reads data from the stream and stores it in buffer
    stream
        .read(&mut buffer)
        .expect("Failed to read from client!");

    // convers the data in buffer into UTF-8 encoded string
    let request = String::from_utf8_lossy(&buffer[..]);
    println!("Received request: {request}");

    let response = "Hello, Client!".as_bytes();
    stream.write(response).expect("Failed to write response!");
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind to address");

    println!("Server listening on 127.0.0.1");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                std::thread::spawn(|| handle_client(stream));
            }
            Err(e) => {
                // stderr - standard error stream
                eprintln!("Failed to establish connection: {e}");
            }
        }
    }
}
