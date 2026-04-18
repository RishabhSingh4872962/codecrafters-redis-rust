use std::{
    collections::{HashMap, VecDeque},
    io::Write,
    net::TcpStream,
};

use crate::{
    constants::constants::NULL_BULK_STRING, response::response::Response,
    utils::utils::create_string_response,
};

pub fn handle_lpop(
    res: &Vec<&str>,
    stream: &mut TcpStream,
    list_store: &mut HashMap<String, Response<VecDeque<String>>>,
) {
    let key = res[1];

    if let Some(val) = list_store.get_mut(key) {
        let first_ele = val.value.pop_front();

        if let Some(ele) = first_ele {
            let s = create_string_response(&ele);

            stream.write_all(s.as_bytes()).unwrap();
            
        } else {
            stream.write_all(NULL_BULK_STRING.as_bytes()).unwrap();
        }
    } else {
        stream.write_all(NULL_BULK_STRING.as_bytes()).unwrap();
    }
}
