use std::{
    collections::HashMap,
    env::{self},
    io::Read,
    net::{TcpListener, TcpStream},
    thread,
};

mod admin;
mod assets;
mod response;
mod utils;

use assets::Assets;
use utils::get_input_vector_from_stream;

use crate::{
    admin::{Admin, Role},
    response::generate_response,
    utils::{write_steam, write_vector_steam},
};

fn handle_response(mut admin: Admin, mut stream: TcpStream, mut storage: HashMap<String, Assets>) {
    let buff = [0; 512];

    loop {
        let raw_input_vec = get_input_vector_from_stream(&mut stream, buff);
        println!("Raw input vector: {:?}", raw_input_vec);

        let (response_content, second_response) =
            generate_response(raw_input_vec, &mut storage, &mut admin);
        write_steam(&mut stream, response_content);
        if let Some(content) = second_response {
            write_vector_steam(&mut stream, content);
        }
    }
}

fn handle_handshake(master_host: String, master_port: u16) {
    let mut read_buf: [u8; 256] = [0; 256];
    let mut stream =
        TcpStream::connect(format!("{}:{}", master_host.as_str(), master_port)).unwrap();
    let buf = "*1\r\n$4\r\nping\r\n";
    write_steam(&mut stream, buf.to_string());

    let replconf_1 = "*3\r\n$8\r\nREPLCONF\r\n$14\r\nlistening-port\r\n$4\r\n6380\r\n";
    write_steam(&mut stream, replconf_1.to_string());

    let replconf_2 = "*3\r\n$8\r\nREPLCONF\r\n$4\r\ncapa\r\n$6\r\npsync2\r\n";
    write_steam(&mut stream, replconf_2.to_string());

    let psync = "*3\r\n$5\r\nPSYNC\r\n$1\r\n?\r\n$2\r\n-1\r\n";
    write_steam(&mut stream, psync.to_string());

    let result = stream.read(&mut read_buf);
    println!("Handshake result: {:?}", result)
}

fn handle_env(admin: &mut Admin) {
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
}

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let storage: HashMap<String, Assets> = HashMap::new();
    println!("Storage: {:?}", storage);

    let mut admin = Admin::new();
    println!("Admin: {:?}", admin);

    handle_env(&mut admin);

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
