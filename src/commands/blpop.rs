use std::{
    collections::{HashMap, VecDeque},
    io::Write,
    net::TcpStream,
    ops::Mul,
    sync::{Arc, Mutex},
    thread,
    time::{Duration, Instant},
};

use crate::{
    constants::constants::NULL_ARRAY_STRING,
    response::response::{DATATYPE, Response},
    utils::utils::create_bulk_string_response,
};

pub fn handle_blpop(
    res: &Vec<&str>,
    stream: &mut TcpStream,
    list_store: Arc<Mutex<HashMap<String, DATATYPE>>>,
) {
    let key = res[1];

    let timeout_sec: Option<f64> = res.get(2).and_then(|sec| sec.parse().ok());

    let mut res = String::from("*2\r\n");

    res.push_str(&create_bulk_string_response(key));

    match timeout_sec {
        Some(0.0) | None => loop {
            let mut list_store = list_store.lock().unwrap();

            if let Some(val) = list_store.get_mut(key) {
                if let DATATYPE::List(val) = val {
                    if let Some(ele) = val.value.pop_front() {
                        res.push_str(&create_bulk_string_response(&ele));
                        stream.write_all(res.as_bytes()).unwrap();
                        break;
                    }
                }
            } else {
                // println!("list stroe {:?}", list_store);
                // thread::sleep(Duration::from_millis(100));
            }
        },
        Some(secs) => {
            let waiting_time = Instant::now() + Duration::from_secs_f64(secs);

            loop {
                let mut list_store = list_store.lock().unwrap();

                let now = Instant::now();

                if now > waiting_time {
                    stream.write_all(NULL_ARRAY_STRING.as_bytes()).unwrap();
                    break;
                }

                if let Some(val) = list_store.get_mut(key) {
                    if let DATATYPE::List(val) = val {
                        if let Some(ele) = val.value.pop_front() {
                            res.push_str(&create_bulk_string_response(&ele));

                            stream.write_all(res.as_bytes()).unwrap();
                            break;
                        }
                    }
                }
            }
        }
    }
}
