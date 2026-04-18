use std::{
    collections::{HashMap, VecDeque},
    io::Write,
    net::TcpStream,
};

use crate::response::response::Response;

pub fn handle_lpush(
    res: &Vec<&str>,
    stream: &mut TcpStream,
    lpush_store: &mut HashMap<String, Response<VecDeque<String>>>,
) {
    let key = res[1];

    let elements = &res[2..];

    let response: String;

    if let Some(val) = lpush_store.get_mut(key) {
        for ele in elements {
            val.value.push_front(ele.to_string());
        }

        response = format!(":{}\r\n", val.value.len());
    } else {
        let mut queue = VecDeque::new();

        for ele in elements {
            queue.push_front(ele.to_string());
        }

        let len = queue.len();
        lpush_store
            .insert(key.to_string(), Response::new(queue, None));
          

        response = format!(":{}\r\n", len);
    }

    stream.write_all(response.as_bytes()).unwrap();
}
