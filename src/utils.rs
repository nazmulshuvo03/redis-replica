use std::{
    io::{Read, Write},
    net::TcpStream,
    time::{SystemTime, UNIX_EPOCH},
};

pub fn current_time_millis() -> u64 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    since_the_epoch.as_secs() * 1000 + u64::from(since_the_epoch.subsec_millis())
}

pub fn write_steam(stream: &mut TcpStream, content: String) {
    stream
        .write_all(content.as_bytes())
        .expect("Failed to write conent to stream")
}

pub fn get_input_vector_from_stream(stream: &mut TcpStream, mut buff: [u8; 512]) -> Vec<String> {
    let separator = "\r\n";
    let bytes_read = stream.read(&mut buff).expect("Failed to read stream");
    println!("bytes read: {:?}", bytes_read);

    if bytes_read == 0 {
        eprintln!("0 Bytes read from stream");
        return Vec::new();
    }

    let buff_vec = buff[..bytes_read].to_vec();
    println!("Buffer vector: {:?}", buff_vec);

    let raw_input = String::from_utf8(buff_vec).expect("String read failed");

    raw_input.split(separator).map(|s| s.to_string()).collect()
}
