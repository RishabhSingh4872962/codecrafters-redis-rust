#![allow(unused_imports)]
use std::{
    collections::HashMap,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread,
    time::{Duration, Instant, SystemTime},
};

const NULL_BULK_STRING: &str = "$-1\r\n";

const OK_SIMPLE_STRING: &str = "+OK\r\n";



pub struct Value {
    value: String,
    expiry: Option<Instant>,
}

impl Value {
    fn new(val: String, exp: Option<Instant>) -> Self {
        Self {
            value: val,
            expiry: exp,
        }
    }
}

fn handle_stream(mut stream: TcpStream, mut store: HashMap<String, Value>) {
    let mut buf = [0; 1024];

    loop {
        match stream.read(&mut buf) {
            Ok(_res) => {
                let str = String::from_utf8_lossy(&buf[..]);

                let str: String = str.split("\r\n").collect();

                println!("st====> {}",str);

                // let uppper_str = str.to_uppercase();

                let res = handle_stream_parser(&str);

                println!("ress===> {:?}", res);

                match res[0] {
                    "PING" => {
                        stream.write_all(b"+PONG\r\n").unwrap();
                    }
                    "ECHO" => {
                        let s = format!("${}\r\n{}\r\n", res[1].len(), res[1]);

                        stream.write_all(s.as_bytes()).unwrap();
                    }
                    "SET" => {
                        let key = res[1].to_string();

                        let value = res[2].to_string();

                        let exp_format = res.get(3);

                        match exp_format {
                            Some(exp) => {
                                let duration: u64 = res[4].parse().unwrap();

                                let expiry_time = handle_expiry(*exp, duration);

                                store.insert(key, Value::new(value, Some(expiry_time)));
                            }
                            None => {
                                store.insert(key, Value::new(value, None));
                            }
                        }

                        stream.write_all(OK_SIMPLE_STRING.as_bytes()).unwrap();
                    }
                    "GET" => {
                        let key = res[1];

                        if let Some(val) = store.get(key) {
                            if val.expiry.is_some() {
                                let expiry = val.expiry.unwrap();

                                if expiry < Instant::now() {
                                    stream.write_all(NULL_BULK_STRING.as_bytes()).unwrap();

                                    store.remove(key).unwrap();
                                    return;
                                }
                            }

                            let s = format!("${}\r\n{}\r\n", val.value.len(), val.value);

                            stream.write_all(s.as_bytes()).unwrap();
                        } else {
                            stream.write_all(NULL_BULK_STRING.as_bytes()).unwrap();
                        }
                    }
                    _ => {}
                }
            }
            Err(e) => {
                // println!("{e}");
                stream.write_all(b"+EEEror\r\n").unwrap();
                break;
            }
        }
    }
}

fn handle_expiry(expiry_type: &str, time: u64) -> Instant {
    match expiry_type {
        "PX" => Instant::now() + Duration::from_millis(time),

        "EX" => Instant::now() + Duration::from_secs(time),
        _ => Instant::now(),
    }
}

fn handle_stream_parser<'a>(str: &'a str) -> Vec<&'a str> {
    let first_char = &str[..1];

    let mut v: Vec<&str> = Vec::new();

    match first_char {
        "*" => {
            let arr_length: u8 = str[1..2].parse().unwrap();

            handle_ele_parse(&str[2..], &mut v, arr_length);
        }
        _ => {}
    }

    v
}

fn handle_ele_parse<'a>(str: &'a str, v: &mut Vec<&'a str>, n: u8) {
    let first_char = &str[..1];

    match first_char {
        "$" => {
            let str_length: usize = str[1..2].parse().unwrap();

            let start: usize = 2;

            let end: usize = 2 + str_length;

            let s: &str = &str[start..end];

            v.push(s);

            if n == 1 {
                return;
            }

            handle_ele_parse(&str[end..], v, n - 1)
        }
        _ => {}
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
                let store: HashMap<String, Value> = HashMap::new();

                || handle_stream(stream, store)
            });
        }
    }

    // let req = b"*1\r\n$4\r\nPING\r\n";
}
