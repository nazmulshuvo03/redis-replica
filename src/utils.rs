use std::{
    error::Error,
    time::{SystemTime, UNIX_EPOCH},
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

pub fn current_time_millis() -> u64 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    since_the_epoch.as_secs() * 1000 + u64::from(since_the_epoch.subsec_millis())
}

pub async fn write_steam(stream: &mut TcpStream, content: String) -> Result<(), Box<dyn Error>> {
    stream.write_all(content.as_bytes()).await?;
    Ok(())
}

pub async fn write_vector_steam(
    stream: &mut TcpStream,
    content: Vec<u8>,
) -> Result<(), Box<dyn Error>> {
    stream.write_all(&content).await?;
    Ok(())
}

pub async fn get_input_vector_from_stream(
    stream: &mut TcpStream,
    mut buff: [u8; 512],
) -> Result<Vec<String>, Box<dyn Error>> {
    let separator = "\r\n";
    let bytes_read = stream.read(&mut buff).await?;
    println!("bytes read: {:?}", bytes_read);

    if bytes_read == 0 {
        eprintln!("0 Bytes read from stream");
        return Ok(Vec::new());
    }

    let buff_vec = buff[..bytes_read].to_vec();
    println!("Buffer vector: {:?}", buff_vec);

    let raw_input = String::from_utf8(buff_vec).expect("String read failed");

    Ok(raw_input.split(separator).map(|s| s.to_string()).collect())
}

// pub fn hex_to_binary_string(hex_string: &str) -> String {
//     let mut binary_string = String::new();
//     for chunk in hex_string.as_bytes().chunks(2) {
//         let chunk_str = std::str::from_utf8(chunk).ok();
//         let hex_value = match u8::from_str_radix(chunk_str.unwrap(), 16) {
//             Ok(val) => val,
//             Err(err) => {
//                 println!("Invalid hexadecimal string: {:?}", err);
//                 return String::new();
//             }
//         };
//         let converted = format!("{:b}", hex_value);
//         binary_string.push_str(converted.as_str());
//     }
//
//     binary_string
// }

pub fn hex_to_binary_vector(hex_string: &str) -> Option<Vec<u8>> {
    let mut binary_vec: Vec<u8> = Vec::new();
    for chunk in hex_string.as_bytes().chunks(2) {
        let chunk_str = std::str::from_utf8(chunk).ok();
        let hex_value = match u8::from_str_radix(chunk_str.unwrap(), 16) {
            Ok(val) => val,
            Err(err) => {
                println!("Invalid hexadecimal string: {:?}", err);
                return None;
            }
        };
        binary_vec.push(hex_value);
    }

    Some(binary_vec)
}
