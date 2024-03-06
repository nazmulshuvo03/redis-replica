use std::collections::HashMap;

use crate::{
    admin::Admin,
    assets::Assets,
    utils::{hex_to_binary_string, hex_to_binary_vector},
};

pub fn generate_response(
    raw_input_vec: Vec<String>,
    storage: &mut HashMap<String, Assets>,
    admin: &mut Admin,
) -> (String, Option<Vec<u8>>) {
    let separator = "\r\n";
    let line_break = "\n";

    let input_command = raw_input_vec[2].as_str();

    match input_command.to_lowercase().as_str() {
        "ping" => {
            let res = format!("{}{}", "+PONG", separator);
            println!("ping command response: {:?}", res);
            (res, None)
        }
        "echo" => {
            let res = format!(
                "{}{}{}{}",
                raw_input_vec[3], separator, raw_input_vec[4], separator
            );
            println!("echo command respnse: {:?}", res);
            (res, None)
        }
        "set" => {
            let mut asset = Assets::new(raw_input_vec[6].to_string());
            if raw_input_vec.len() > 8 {
                if raw_input_vec[8] == "px" {
                    asset.update_expiry(raw_input_vec[10].as_str());
                } else {
                    println!("set is provided with any other parameter than px")
                }
            } else {
                println!("No expiry provide");
            }
            storage.insert(raw_input_vec[4].to_string(), asset);
            let res = format!("{}{}", "+OK", separator,);
            println!("set command response: {:?}", res);
            (res, None)
        }
        "get" => {
            if let Some(asset) = storage.get(raw_input_vec[4].as_str()) {
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
                    (res, None)
                } else {
                    let res = format!("${}{}", "-1", separator);
                    println!("get command response object not found: {:?}", res);
                    (res, None)
                }
            } else {
                let res = format!("${}{}", "-1", separator);
                println!("get command response object not found: {:?}", res);
                (res, None)
            }
        }
        "info" => {
            let line1 = format!("role:{}", admin.get_replica_role());
            let line2 = format!("master_replid:{}", admin.get_replica_id());
            let line3 = format!("master_repl_offset:{}", admin.get_replica_offset());
            let line = format!("{}{}{}{}{}", line1, line_break, line2, line_break, line3);
            let res = format!("${}{}{}{}", line.len(), separator, line, separator);
            println!("info command response: {:?}", res);
            (res, None)
        }
        "replconf" => {
            let res = format!("{}{}", "+OK", separator);
            println!("replconf command response: {:?}", res);
            (res, None)
        }
        "psync" => {
            let res = format!(
                "{} {} {}{}",
                "+FULLRESYNC",
                admin.get_replica_id(),
                "0",
                separator
            );
            println!("psync command response: {:?}", res);

            let empty_rdb_hex = "524544495330303131fa0972656469732d76657205372e322e30fa0a72656469732d62697473c040fa056374696d65c26d08bc65fa08757365642d6d656dc2b0c41000fa08616f662d62617365c000fff06e3bfec0ff5aa2";

            let binary_string = hex_to_binary_string(empty_rdb_hex);
            let formated_res = format!("${}{}{}", binary_string.len(), separator, binary_string);
            println!("Formated binary string from RDB hex: {:?}", formated_res);

            let binary_vec = hex_to_binary_vector(empty_rdb_hex).unwrap();
            let mut vector_res = Vec::from(format!("${}{}", binary_vec.len(), separator));
            vector_res.extend(&binary_vec);
            println!("Formated binary vector from RDB hex: {:?}", vector_res);

            (res, Some(vector_res))
        }
        _ => {
            println!("Undefined command {:?}", input_command);
            (String::from(""), None)
        }
    }
}
