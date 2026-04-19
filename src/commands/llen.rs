use std::{
    collections::{HashMap, VecDeque},
    io::Write,
    net::TcpStream,
    sync::{Arc, Mutex},
};

use crate::response::response::{DATATYPE, Response};

pub fn handle_llen(
    res: &Vec<&str>,
    stream: &mut TcpStream,

    list_store: Arc<Mutex<HashMap<String, DATATYPE>>>,
) {
    let key = res[1];

    let list_store = list_store.lock().unwrap();

    if let Some(val) = list_store.get(key) {
        if let DATATYPE::List(val) = val {
            let len = val.value.len();

            let s = format!(":{}\r\n", len);

            stream.write_all(s.as_bytes()).unwrap();
        }
    } else {
        stream.write_all(String::from(":0\r\n").as_bytes()).unwrap();
    }
}
