use std::{
    collections::HashMap,
    io::Write,
    net::TcpStream,
    sync::{Arc, Mutex},
    time::Instant,
};

use crate::{constants::constants::NULL_BULK_STRING, response::response::Response};

pub fn handle_get(
    res: &Vec<&str>,
    key_value_store: Arc<Mutex<HashMap<String, Response<String>>>>,
    stream: &mut TcpStream,
) {
    let key = res[1];

    let mut store = key_value_store.lock().unwrap();

    if let Some(val) = store.get(key) {
        if val.expiry.is_some() {
            let expiry: Instant = val.expiry.unwrap();

            if expiry < Instant::now() {
                stream.write_all(NULL_BULK_STRING.as_bytes()).unwrap();

                store.remove(key).unwrap();
                return;
            }
        }

        let s = format!("${}\r\n{}\r\n", val.value.len(), val.value);

        stream.write_all(s.as_bytes()).unwrap();
    } else {
        stream.write_all(NULL_BULK_STRING.as_bytes()).unwrap();
    }
}
