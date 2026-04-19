use std::{
    collections::{HashMap, VecDeque},
    io::Write,
    net::TcpStream,
    sync::{Arc, Mutex},
};

use crate::{
    response::response::{DATATYPE, Response},
    utils::utils::create_simple_string,
};

pub fn handle_datatype(
    res: &Vec<&str>,
    stream: &mut TcpStream,
    list_store: Arc<Mutex<HashMap<String, DATATYPE>>>,
) {
    let key = res[1];

    let response: String;

    if let Some(val) = list_store.lock().unwrap().get(key) {
        match val {
            DATATYPE::String(_) => {
                response = create_simple_string("string");
            }
            DATATYPE::List(_) => {
                response = create_simple_string("list");
            }
        }
    } else {
        response = create_simple_string("none");
    }

    stream.write_all(response.as_bytes()).unwrap();
}
