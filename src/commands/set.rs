use std::{
    collections::HashMap,
    io::Write,
    net::TcpStream,
    sync::{Arc, Mutex},
    time::Instant,
};

use crate::{
    constants::constants::OK_SIMPLE_STRING,
    response::response::{DATATYPE, Response},
    utils::utils::{handle_expiry, insert_key},
};

pub fn handle_set(
    res: &Vec<&str>,
    redis_main_store: Arc<Mutex<HashMap<String, DATATYPE>>>,
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

            insert_key(
                redis_main_store,
                key,
                DATATYPE::String(Response::new(value, Some(expiry_time))),
            );
        }
        None => {
            insert_key(
                redis_main_store,
                key,
                DATATYPE::String(Response::new(value, None)),
            );
        }
    }

    stream.write_all(OK_SIMPLE_STRING.as_bytes()).unwrap();
}
