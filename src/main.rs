// Uncomment this block to pass the first stage
use std::{
    collections::HashMap,
    env::{self},
    io::{Read, Write},
    net::{SocketAddr, TcpListener, TcpStream},
    thread,
};

mod admin;
mod assets;
pub mod utils;

use assets::Assets;

use crate::admin::Admin;

fn handle_response(mut admin: Admin, mut stream: TcpStream, mut storage: HashMap<String, Assets>) {
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
            "set" => {
                let mut asset = Assets::new(raw_input_vec[6].to_string());
                if raw_input_vec.len() > 8 {
                    if raw_input_vec[8] == "px" {
                        asset.update_expiry(raw_input_vec[10]);
                    } else {
                        println!("set is provided with any other parameter than px")
                    }
                } else {
                    println!("No expiry provide");
                }
                storage.insert(raw_input_vec[4].to_string(), asset);
                let res = format!("{}{}", "+OK", separator,);
                println!("set command response: {:?}", res);
                stream
                    .write_all(res.as_bytes())
                    .expect("Failed to write respnse");
            }
            "get" => {
                if let Some(asset) = storage.get(raw_input_vec[4]) {
                    println!("Found asset: {:?}", asset);
                    let mut new_asset = asset.clone();
                    if !new_asset.is_value_expired() {
                        let res = format!(
                            "${}{}{}{}",
                            new_asset.get_value_len(),
                            separator,
                            new_asset.get_value(),
                            separator
                        );
                        println!("get command response object found: {:?}", res);
                        stream
                            .write_all(res.as_bytes())
                            .expect("Failed to write response");
                    } else {
                        let res = format!("${}{}", "-1", separator);
                        println!("get command response object not found: {:?}", res);
                        stream
                            .write_all(res.as_bytes())
                            .expect("Failed to write response");
                    }
                } else {
                    let res = format!("${}{}", "-1", separator);
                    println!("get command response object not found: {:?}", res);
                    stream
                        .write_all(res.as_bytes())
                        .expect("Failed to write response");
                }
            }
            "info" => {
                // let line1 = "#Replication";
                let line2 = format!("role:{}", admin.get_role().as_str());
                let res = format!(
                    "${}{}{}{}",
                    // "${}{}{}${}{}{}{}",
                    // line1.len(),
                    // separator,
                    // line1,
                    line2.len(),
                    separator,
                    line2,
                    separator
                );
                println!("info command response: {:?}", res);
                stream
                    .write_all(res.as_bytes())
                    .expect("Failed to write message");
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

    let storage: HashMap<String, Assets> = HashMap::new();
    println!("Storage: {:?}", storage);

    let mut admin = Admin::new();
    println!("Admin: {:?}", admin);

    let args: Vec<String> = env::args().collect();
    println!("Env Args: {:?}", args);

    if args.len() > 1 && args[1] == "--port" {
        let new_port: u16 = args[2].parse::<u16>().unwrap();
        admin.set_port(new_port);
    }

    if args.len() > 3 && args[3] == "--replicaof" {
        println!("replica of: {}:{}: ", args[4], args[5]);
        admin.set_role(String::from("slave"))
    }

    // Uncomment this block to pass the first stage
    let listener = TcpListener::bind(SocketAddr::from(([127, 0, 0, 1], admin.get_port()))).unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("accepted new connection");
                let storage_clone = storage.clone();
                let admin_clone = admin.clone();
                thread::spawn(move || {
                    handle_response(admin_clone, stream, storage_clone);
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
