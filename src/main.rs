#![allow(unused_imports)]
use std::{
    collections::{HashMap, VecDeque},
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread,
    time::{Duration, Instant, SystemTime},
};

mod constants;
use constants::constants::*;

mod commands;

mod utils;

use utils::utils::{
    create_array_response, create_bulk_string_response, handle_expiry, handle_negative_index,
};

use utils::parser::parser;

use crate::{
    commands::{
        blpop::handle_blpop, datatype::handle_datatype, echo::handle_echo, get::handle_get,
        llen::handle_llen, lpop::handle_lpop, lpush::handle_lpush, lrange::handle_lrange,
        ping::handle_ping, rpush::handle_rpush, set::handle_set,
    },
    response::response::{DATATYPE, RedisObject},
};

mod response;

use response::response::Response;

fn handle_stream(mut stream: TcpStream, redis_main_store: Arc<Mutex<HashMap<String, DATATYPE>>>) {
    let mut buf = [0; 1024];

    loop {
        let redis_main_store = redis_main_store.clone();
        match stream.read(&mut buf) {
            Ok(_res) => {
                let str = String::from_utf8_lossy(&buf[..]);

                // let str: String = "*5$3SET$10strawberry$5grape$2PX$3100".to_string();

                // println!("str====> {:?}", str);

                // let uppper_str = str.to_uppercase();

                let res = parser(&str);

                println!("ress===> {:?}", res);

                match res[0] {
                    "PING" => handle_ping(&mut stream),
                    "ECHO" => {
                        handle_echo(&mut stream, res.get(1));
                    }
                    "SET" => {
                        handle_set(&res, redis_main_store, &mut stream);

                        buf = [0; 1024];
                    }
                    "GET" => {
                        handle_get(&res, redis_main_store, &mut stream);
                        buf = [0; 1024];
                    }
                    "RPUSH" => {
                        handle_rpush(&res, &mut stream, redis_main_store);
                        buf = [0; 1024];
                    }

                    "LRANGE" => {
                        // println!("lrange =========> str======> {:?}", str);

                        handle_lrange(&res, &mut stream, redis_main_store);
                        buf = [0; 1024];
                    }
                    "LPUSH" => {
                        handle_lpush(&res, &mut stream, redis_main_store);
                        buf = [0; 1024];
                    }
                    "LLEN" => {
                        handle_llen(&res, &mut stream, redis_main_store);

                        buf = [0; 1024];
                    }
                    "LPOP" => {
                        handle_lpop(&res, &mut stream, redis_main_store);

                        buf = [0; 1024];
                    }
                    "BLPOP" => {
                        handle_blpop(&res, &mut stream, redis_main_store);

                        buf = [0; 1024];
                    }
                    "TYPE" => {
                        handle_datatype(&res, &mut stream, redis_main_store);
                        buf = [0; 1024];
                    }

                    _ => {}
                }
            }
            Err(_e) => {
                // println!("{e}");
                stream.write_all(b"+EEEror\r\n").unwrap();
                break;
            }
        }
    }
}

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment the code below to pass the first stage

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    let redis_main_object: Arc<Mutex<HashMap<String, DATATYPE>>> =
        Arc::new(Mutex::new(HashMap::new()));

    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            thread::spawn({
                let main_store = redis_main_object.clone();

                move || handle_stream(stream, main_store)
            });
        }
    }

    // let req = b"*5$3SET$10strawberry$5grape$2PX$3100";

    // let str: String =
    //     "*5\r\n$3\r\nSET\r\n$10\r\nstrawberry\r\n$5\r\ngrape\r\n$2\r\nPX\r\n$3\r\n100\r\n"
    //         .to_string();

    // let s = "*5\r\n$3\r\nSET\r\n$4\r\npear\r\n$6\r\nbanana\r\n$2\r\nPX\r\n$3\r\n100\r\n\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0";

    // let res = parser(&s);

    // println!("{:?}", res);
}
