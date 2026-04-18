use std::{collections::{HashMap, VecDeque}, io::Write, net::TcpStream};

use crate::response::response::Response;

pub fn handle_llen(
    res: &Vec<&str>,
    stream: &mut TcpStream,

    list_store: &HashMap<String, Response<VecDeque<String>>>,
) {
    let key = res[1];

    if let Some(val) = list_store.get(key) {
        let len = val.value.len();

        let s = format!(":{}\r\n", len);

        stream.write_all(s.as_bytes()).unwrap();
    } else {
        stream.write_all(String::from(":0\r\n").as_bytes()).unwrap();
    }
}
