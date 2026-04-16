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

const EMPTY_ARRAY_STRING: &str = "*0\r\n";

pub struct Value<T> {
    value: T,
    expiry: Option<Instant>,
}

impl<T> Value<T> {
    fn new(val: T, exp: Option<Instant>) -> Self {
        Self {
            value: val,
            expiry: exp,
        }
    }
}

fn handle_stream(
    mut stream: TcpStream,
    key_value_store: &mut HashMap<String, Value<String>>,
    list_store: &mut HashMap<String, Value<Vec<String>>>,
) {
    let mut buf = [0; 1024];

    loop {
        match stream.read(&mut buf) {
            Ok(_res) => {
                let str = String::from_utf8_lossy(&buf[..]);

                // let str: String = "*5$3SET$10strawberry$5grape$2PX$3100".to_string();

                println!("str====> {:?}", str);

                // let uppper_str = str.to_uppercase();

                let res = parser(&str);

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

                                key_value_store.insert(key, Value::new(value, Some(expiry_time)));
                            }
                            None => {
                                key_value_store.insert(key, Value::new(value, None));
                            }
                        }

                        stream.write_all(OK_SIMPLE_STRING.as_bytes()).unwrap();
                    }
                    "GET" => {
                        let key = res[1];

                        if let Some(val) = key_value_store.get(key) {
                            if val.expiry.is_some() {
                                let expiry = val.expiry.unwrap();

                                if expiry < Instant::now() {
                                    stream.write_all(NULL_BULK_STRING.as_bytes()).unwrap();

                                    key_value_store.remove(key).unwrap();
                                    return;
                                }
                            }

                            let s = format!("${}\r\n{}\r\n", val.value.len(), val.value);

                            stream.write_all(s.as_bytes()).unwrap();
                        } else {
                            stream.write_all(NULL_BULK_STRING.as_bytes()).unwrap();
                        }
                    }
                    "RPUSH" => {
                        let key = res[1];

                        let elements = &res[2..];

                        let mut v = Vec::new();

                        for ele in elements {
                            v.push(ele.to_string());
                        }

                        let response: String;

                        if let Some(val) = list_store.get_mut(key) {
                            val.value.append(&mut v);

                            response = format!(":{}\r\n", val.value.len());
                        } else {
                            response = format!(":{}\r\n", v.len());

                            list_store.insert(key.to_string(), Value::new(v, None));
                        }

                        stream.write_all(response.as_bytes()).unwrap();

                        buf=[0;1024];
                    }

                    "LRANGE" => {


                        println!("lrange =========> str======> {}",str);

                        let list_key = res[1];

                        if let Some(val) = list_store.get(list_key) {
                            let start_index: usize = res[2].parse().unwrap();

                            let end_index: usize = res[3].parse().unwrap();

                            if start_index < end_index && start_index < val.value.len() {
                                let get_v;

                                if end_index+1 >= val.value.len() {
                                    get_v = val.value.get(start_index ..);
                                } else {
                                    get_v = val.value.get(start_index..end_index+1);
                                }

                                if let Some(res) = get_v {
                                    let result = create_array_response(res);

                                    stream.write_all(result.as_bytes()).unwrap();
                                    return;
                                }
                            }
                        }

                        stream.write_all(EMPTY_ARRAY_STRING.as_bytes()).unwrap();
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

fn handle_expiry(expiry_type: &str, time: u64) -> Instant {
    match expiry_type {
        "PX" => Instant::now() + Duration::from_millis(time),

        "EX" => Instant::now() + Duration::from_secs(time),
        _ => Instant::now(),
    }
}

pub fn create_array_response(v: &[String]) -> String {
    let mut res = format!("*{}\r\n", v.len());

    for ele in v {
        let s = create_string_response(ele);

        res.push_str(&s);
    }

    res
}

pub fn create_string_response(str: &str) -> String {
    format!("${}\r\n{}\r\n", str.len(), str)
}
fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment the code below to pass the first stage

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            thread::spawn({
                let mut store: HashMap<String, Value<String>> = HashMap::new();

                let mut list_map: HashMap<String, Value<Vec<String>>> = HashMap::new();

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

fn parser(str: &str) -> Vec<&str> {
    let first_ch: &str = &str[..1];

    let mut v: Vec<&str> = Vec::new();


    // println!("str parser===>{}",str);
    match first_ch {
        "*" => {
            let next = str.find("\r\n");

            if let Some(index) = next {
                // let mut arr_len: isize = str[1..index].parse().unwrap();

                let rest_str = &str[index + 2..];

                // println!("arr_len==> {},res_str ===>{:?}", 0, rest_str);

                let mut p = 0;

                for s in rest_str.lines() {
                    handle_string(s, &mut p, &mut v);
                }
            }

            return v;
        }
        _ => return v,
    }
}

fn handle_string<'a>(str: &'a str, prev: &mut usize, v: &mut Vec<&'a str>) {
    let first_ch = &str[..1];

    match first_ch {
        "$" => {
            *prev = str[1..].parse().unwrap();
        }
        _ if str.len() > 0 => {
            if str.len() == *prev {
                v.push(str);
            }
        }
        _ => {}
    }
}
