use std::{
    collections::{HashMap, VecDeque},
    io::Write,
    net::TcpStream,
    sync::{Arc, Mutex},
};

use crate::response::response::{DATATYPE, Response};

pub fn handle_lpush(
    res: &Vec<&str>,
    stream: &mut TcpStream,
    list_store: Arc<Mutex<HashMap<String, DATATYPE>>>,
) {
    let key = res[1];

    let elements = &res[2..];

    let response: String;

    let mut list_store = list_store.lock().unwrap();
    
    if let Some(val) = list_store.get_mut(key) {
        match val {
            DATATYPE::List(val) => {
                for ele in elements {
                    val.value.push_front(ele.to_string());
                }

                response = format!(":{}\r\n", val.value.len());
            }
            _ => {
                response = "".to_string();
            }
        }
    } else {
        let mut queue = VecDeque::new();

        for ele in elements {
            queue.push_front(ele.to_string());
        }

        let len = queue.len();
        list_store.insert(key.to_string(), DATATYPE::List(Response::new(queue, None)));

        response = format!(":{}\r\n", len);
    }

    stream.write_all(response.as_bytes()).unwrap();
}
