use std::{
    collections::{HashMap, VecDeque},
    io::Write,
    net::TcpStream,
    sync::{Arc, Mutex},
};

use crate::{
    constants::constants::NULL_BULK_STRING, response::response::Response,
    utils::utils::create_string_response,
};

pub fn handle_lpop(
    res: &Vec<&str>,
    stream: &mut TcpStream,
    list_store: Arc<Mutex<HashMap<String, Response<VecDeque<String>>>>>,
) {
    let key = res[1];

    let removed_ele_count = res.get(2).and_then(|e| e.parse::<usize>().ok());

    let mut list_store = list_store.lock().unwrap();

    if let Some(val) = list_store.get_mut(key) {
        if let Some(count) = removed_ele_count {
            let response = handle_pop_to_string(&mut val.value, count);

            stream.write_all(response.as_bytes()).unwrap();
        } else {
            let first_ele = val.value.pop_front();

            if let Some(ele) = first_ele {
                let s = create_string_response(&ele);

                stream.write_all(s.as_bytes()).unwrap();
            } else {
                stream.write_all(NULL_BULK_STRING.as_bytes()).unwrap();
            }
        }
    } else {
        stream.write_all(NULL_BULK_STRING.as_bytes()).unwrap();
    }
}

fn handle_pop_to_string(val: &mut VecDeque<String>, mut n: usize) -> String {
    n = if n > val.len() { val.len() } else { n };

    let mut res = format!("*{}\r\n", n);

    loop {
        if n == 0 {
            break;
        }
        if let Some(poped_ele) = val.pop_front() {
            let s = create_string_response(&poped_ele);

            res.push_str(&s);
        } else {
            break;
        }
        n -= 1;
    }

    res
}
