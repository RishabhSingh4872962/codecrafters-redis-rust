use std::{collections::HashMap, io::Write, net::TcpStream, time::Instant};

use crate::{constants::constants::NULL_BULK_STRING, response::response::Response};

pub fn handle_get(
    res: &Vec<&str>,
    key_value_store: &mut HashMap<String, Response<String>>,
    stream: &mut TcpStream,
) {
    let key = res[1];

    if let Some(val) = key_value_store.get(key) {
        if val.expiry.is_some() {
            let expiry = val.expiry.unwrap();

            if expiry < Instant::now() {
                stream.write_all(NULL_BULK_STRING.as_bytes()).unwrap();

                key_value_store.remove(key).unwrap();
                return;
            }
        }

        let s = format!("${}\r\n{}\r\n", val.value.len(), val.value);

        stream.write_all(s.as_bytes()).unwrap();
    } else {
        stream.write_all(NULL_BULK_STRING.as_bytes()).unwrap();
    }
}
