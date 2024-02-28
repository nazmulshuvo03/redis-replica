// Uncomment this block to pass the first stage
use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread,
};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    // TcpListener == "server"; TcpListener::bind -> create server
    // TcpStream == "client"; TcpStream::connect -> connect to a server
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("accepted new connection");
                thread::spawn(|| {
                    handle_response(stream);
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_response(mut stream: TcpStream) {
    let mut buff = [0; 512];

    loop {
        let bytes_read = stream.read(&mut buff).expect("Failed to read stream");

        if bytes_read == 0 {
            return;
        }
        match stream.write_all(b"+PONG\r\n") {
            Ok(_) => println!("Write successful"),
            Err(e) => println!("Error writing stream: {}", e),
        }
    }
}
