#![allow(unused_imports)]
use std::{
    collections::{HashMap, VecDeque},
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread,
    time::{Duration, Instant, SystemTime},
};

mod constants;
use constants::constants::*;

mod commands;

mod utils;

use utils::utils::{
    create_array_response, create_string_response, handle_expiry, handle_negative_index,
};

use utils::parser::parser;

use crate::commands::{
    echo::handle_echo, get::handle_get, lpush::handle_lpush, lrange::handle_lrange,
    ping::handle_ping, rpush::handle_rpush, set::handle_set,
};

mod response;

use response::response::Response;

fn handle_stream(
    mut stream: TcpStream,
    key_value_store: &mut HashMap<String, Response<String>>,
    list_store: &mut HashMap<String, Response<VecDeque<String>>>,
) {
    let mut buf = [0; 1024];

    loop {
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
                        handle_set(&res, key_value_store, &mut stream);

                        buf = [0; 1024];
                    }
                    "GET" => {
                        handle_get(&res, key_value_store, &mut stream);
                        buf = [0; 1024];
                    }
                    "RPUSH" => {
                        handle_rpush(&res, &mut stream, list_store);
                        buf = [0; 1024];
                    }

                    "LRANGE" => {
                        // println!("lrange =========> str======> {:?}", str);

                        handle_lrange(&res, &mut stream, list_store);
                        buf = [0; 1024];

                    }
                    "LPUSH" => {
                        handle_lpush(&res,&mut stream,list_store);
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

    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            thread::spawn({
                let mut store: HashMap<String, Response<String>> = HashMap::new();

                let mut list_map: HashMap<String, Response<VecDeque<String>>> = HashMap::new();


                move || handle_stream(stream, &mut store, &mut list_map)
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
