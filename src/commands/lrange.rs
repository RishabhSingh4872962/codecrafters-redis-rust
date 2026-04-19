use std::{
    collections::{HashMap, VecDeque},
    io::Write,
    net::TcpStream,
    sync::{Arc, Mutex},
};

use crate::{
    constants::constants::EMPTY_ARRAY_STRING,
    response::response::Response,
    utils::utils::{create_array_response, handle_negative_index},
};

pub fn handle_lrange(
    res: &Vec<&str>,
    stream: &mut TcpStream,
    list_store: Arc<Mutex<HashMap<String, Response<VecDeque<String>>>>>,
) {
    let list_key = res[1];

    let list_store = list_store.lock().unwrap();

    if let Some(val) = list_store.get(list_key) {
        let start: isize = res[2].parse().unwrap();

        let end: isize = res[3].parse().unwrap();

        let len = val.value.len() as isize;

        let (start_index, end_index) = handle_negative_index(start, end, len);

        println!("old index start=>{}, end=>{}", start, end);
        println!("new index start=>{}, end=>{}", start_index, end_index);

        if start_index < end_index && start_index < val.value.len() {
            let get_v = val.value.range(start_index..=end_index);

            // println!("get v========> {:?}", get_v);

            let result = create_array_response(get_v);

            println!("result =============> {}", result);

            stream.write_all(result.as_bytes()).unwrap();
        } else {
            stream.write_all(EMPTY_ARRAY_STRING.as_bytes()).unwrap();
        }
    } else {
        stream.write_all(EMPTY_ARRAY_STRING.as_bytes()).unwrap();
    }
}
