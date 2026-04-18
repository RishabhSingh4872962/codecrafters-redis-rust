use std::{collections::HashMap, io::Write, net::TcpStream};

use crate::response::response::Response;

pub fn handle_rpush(
    res: &Vec<&str>,
    stream: &mut TcpStream,
    list_store: &mut HashMap<String, Response<Vec<String>>>,
) {
    let key = res[1];

    let elements = &res[2..];

    let mut v = Vec::new();

    for ele in elements {
        v.push(ele.to_string());
    }

    let response: String;

    if let Some(val) = list_store.get_mut(key) {
        val.value.append(&mut v);

        response = format!(":{}\r\n", val.value.len());
    } else {
        response = format!(":{}\r\n", v.len());

        list_store.insert(key.to_string(), Response::new(v, None));
    }

    stream.write_all(response.as_bytes()).unwrap();
}
