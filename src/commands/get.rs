use std::{
    collections::HashMap,
    io::Write,
    net::TcpStream,
    sync::{Arc, Mutex},
    time::Instant,
};

use crate::{
    constants::constants::NULL_BULK_STRING,
    response::response::{DATATYPE, Response},
};

pub fn handle_get(
    res: &Vec<&str>,
    redis_main_store: Arc<Mutex<HashMap<String, DATATYPE>>>,
    stream: &mut TcpStream,
) {
    let key = res[1];

    let mut store = redis_main_store.lock().unwrap();

    if let Some(val) = store.get(key) {
        match val {
            DATATYPE::String(val) => {
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
            }
            _ => {}
        }
    } else {
        stream.write_all(NULL_BULK_STRING.as_bytes()).unwrap();
    }
}
