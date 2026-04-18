use std::{
    collections::{HashMap, VecDeque},
    io::Write,
    net::TcpStream,
    time::{Duration, Instant},
};

use crate::{constants::constants::NULL_ARRAY_STRING, response::response::Response};

pub fn handle_blpop(
    res: &Vec<&str>,
    stream: &mut TcpStream,
    list_store: &mut HashMap<String, Response<VecDeque<String>>>,
) {
    let key = res[1];

    let timeout_sec: Option<u64> = res.get(2).and_then(|sec| sec.parse().ok());

    let mut res = String::from("*2\r\n");

    res.push_str(key);

    match timeout_sec {
        Some(0) | None => loop {
            if let Some(val) = list_store.get_mut(key) {

                println!("value ===> {:?}",val.value);

                if let Some(ele) = val.value.pop_front() {
                    res.push_str(&ele);
                    stream.write_all(res.as_bytes()).unwrap();
                    break;
                }
            }
        },
        Some(secs) => {
            let waiting_time = Instant::now() + Duration::from_secs(secs);

            loop {
                let now = Instant::now();

                if now > waiting_time {
                    stream.write_all(NULL_ARRAY_STRING.as_bytes()).unwrap();
                    break;
                }

                if let Some(val) = list_store.get_mut(key) {
                    if let Some(ele) = val.value.pop_front() {
                        res.push_str(&ele);

                        stream.write_all(res.as_bytes()).unwrap();
                        break;
                    }
                }
            }
        }
    }

}
