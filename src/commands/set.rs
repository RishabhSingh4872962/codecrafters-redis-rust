use std::{collections::HashMap, io::Write, net::TcpStream, time::Instant};

use crate::{
    constants::constants::OK_SIMPLE_STRING, response::response::Response,
    utils::utils::handle_expiry,
};

pub fn handle_set(
    res: &Vec<&str>,
    key_value_store: &mut HashMap<String, Response<String>>,
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

            key_value_store.insert(key, Response::new(value, Some(expiry_time)));
        }
        None => {
            key_value_store.insert(key, Response::new(value, None));
        }
    }

    stream.write_all(OK_SIMPLE_STRING.as_bytes()).unwrap();
}
