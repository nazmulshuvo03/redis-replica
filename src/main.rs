use std::{
    collections::HashMap,
    env::{self},
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread,
};

mod admin;
mod assets;
pub mod utils;

use assets::Assets;

use crate::admin::{Admin, Role};

fn handle_response(mut admin: Admin, mut stream: TcpStream, mut storage: HashMap<String, Assets>) {
    let mut buff = [0; 512];
    let separator = "\r\n";
    let line_break = "\n";

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
                let line1 = format!("role:{}", admin.get_replica_role());
                let line2 = format!("master_replid:{}", admin.get_replica_id());
                let line3 = format!("master_repl_offset:{}", admin.get_replica_offset());
                let line = format!("{}{}{}{}{}", line1, line_break, line2, line_break, line3);
                let res = format!("${}{}{}{}", line.len(), separator, line, separator);
                println!("info command response: {:?}", res);
                stream
                    .write_all(res.as_bytes())
                    .expect("Failed to write message");
            }
            "replconf" => {
                let res = format!("{}{}", "+OK", separator);
                println!("ping command response: {:?}", res);
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

fn handle_handshake(master_host: String, master_port: u16) {
    let mut read_buf: [u8; 256] = [0; 256];
    let mut stream =
        TcpStream::connect(format!("{}:{}", master_host.as_str(), master_port)).unwrap();
    let buf = "*1\r\n$4\r\nping\r\n";
    stream.write_all(buf.as_bytes()).unwrap();

    let replconf_1 = "*3\r\n$8\r\nREPLCONF\r\n$14\r\nlistening-port\r\n$4\r\n6380\r\n";
    stream.write_all(replconf_1.as_bytes()).unwrap();

    let replconf_2 = "*3\r\n$8\r\nREPLCONF\r\n$4\r\ncapa\r\n$6\r\npsync2\r\n";
    stream.write_all(replconf_2.as_bytes()).unwrap();

    let result = stream.read(&mut read_buf);
    println!("Handshake result: {:?}", result)
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
        let master_host = args[4].to_string();
        let master_port = args[5].parse::<u16>().unwrap();
        let role = Role::Slave;
        admin.set_replica(master_host, master_port, role);
    }

    // Uncomment this block to pass the first stage
    let listener = TcpListener::bind(format!("{}:{}", admin.get_host(), admin.get_port())).unwrap();

    if admin.get_replica_role() == Role::Slave {
        handle_handshake(admin.get_replica_host(), admin.get_replica_port())
    }

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
