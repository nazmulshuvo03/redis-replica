// Uncomment this block to pass the first stage
use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread,
};

fn handle_response(mut stream: TcpStream) {
    let mut buff = [0; 512];
    let separator = "\r\n";

    loop {
        let bytes_read = stream.read(&mut buff).expect("Failed to read stream");
        println!("bytes read: {:?}", bytes_read);

        if bytes_read == 0 {
            eprintln!("0 Bytes read from stream");
            return;
        }
        let buff_vec = buff[..bytes_read].to_vec();
        println!("Buffer vector: {:?}", buff_vec);
        let raw_input = String::from_utf8(buff_vec).expect("String read failed");
        let raw_input_str = raw_input.as_str();
        println!("Raw input string: {:?}", raw_input_str);
        raw_input_str.chars().for_each(|c| print!("{} ", c));
        let raw_input_vec: Vec<&str> = raw_input_str.split(separator).collect();
        println!("Raw input vector: {:?}", raw_input_vec);

        // let mut first: u32 = 0;
        // match raw_input_vec.first() {
        //     Some(v) => {
        //         println!("first: {:?}", v);
        //         match v.strip_prefix("*") {
        //             Some(num) => {
        //                 println!("Number from first: {:?}", num);
        //                 let value: u32 = num.parse::<u32>().unwrap();
        //                 first = value;
        //             }
        //             None => println!("Could not found * in first element"),
        //         }
        //     }
        //     None => println!("There is no element in raw input array"),
        // };
        // println!("Value from first element: {}", first);

        let command = raw_input_vec[2];

        match command {
            "ping" => {
                let res = format!("{}{}", "+PONG", separator);
                println!("ping command response: {:?}", res);
                stream
                    .write_all(res.as_bytes())
                    .expect("Failed to write respnse");
            }
            "echo" => {
                let res = format!(
                    "{}{}{}{}",
                    raw_input_vec[3], separator, raw_input_vec[4], separator
                );
                println!("echo command respnse: {:?}", res);
                stream
                    .write_all(res.as_bytes())
                    .expect("Failed to write respnse");
            }
            _ => {
                println!("Undefined command");
            }
        }
    }
}

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
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
