use std::{
    collections::{HashMap, VecDeque},
    io::Write,
    net::TcpStream,
    sync::{Arc, Mutex},
};

use crate::response::response::Response;

pub fn handle_rpush(
    res: &Vec<&str>,
    stream: &mut TcpStream,
    list_store: Arc<Mutex<HashMap<String, Response<VecDeque<String>>>>>,
) {
    let key = res[1];

    let elements = &res[2..];

    let mut v = VecDeque::new();

    for ele in elements {
        v.push_back(ele.to_string());
    }

    let response: String;

    let store = list_store.lock();

    match store {
        Ok(mut store) => {
            if let Some(val) = store.get_mut(key) {
                val.value.append(&mut v);

                response = format!(":{}\r\n", val.value.len());
            } else {
                response = format!(":{}\r\n", v.len());

                store.insert(key.to_string(), Response::new(v, None));
            }
            stream.write_all(response.as_bytes()).unwrap();
        }
        Err(_) => {}
    }
}
