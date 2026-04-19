use std::{
    collections::HashMap,
    io::Write,
    net::TcpStream,
    sync::{Arc, Mutex},
    time::Instant,
};

use crate::{
    constants::constants::OK_SIMPLE_STRING, response::response::Response,
    utils::utils::handle_expiry,
};

pub fn handle_set(
    res: &Vec<&str>,
    key_value_store: Arc<Mutex<HashMap<String, Response<String>>>>,
    stream: &mut TcpStream,
) {
    let key = res[1].to_string();

    let value = res[2].to_string();

    let exp_format = res.get(3);

    let duration = res.get(4);

    match exp_format {
        Some(exp) => {
            let duration: u64 = duration.unwrap().parse().unwrap();

            let expiry_time = handle_expiry(*exp, duration);

            {
                let mut store = key_value_store.lock().unwrap();

                store.insert(key, Response::new(value, Some(expiry_time)));
            }
        }
        None => {
            let mut store = key_value_store.lock().unwrap();

            store.insert(key, Response::new(value, None));
        }
    }

    stream.write_all(OK_SIMPLE_STRING.as_bytes()).unwrap();
}
