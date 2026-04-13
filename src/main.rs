#![allow(unused_imports)]
use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

fn handle_stream(mut stream: TcpStream) {
    let mut buf = [0; 1024];

    loop {
        match stream.read(&mut buf) {
            Ok(_res) => {
                let str = String::from_utf8_lossy(&buf[..]);

                let str: String = str.split("\r\n").collect();

                let res = handle_stream_parser(&str);

                println!("ress===> {:?}",res);

                match res[0] {
                    "PING" => {
                        stream.write_all(b"+PONG\r\n").unwrap();
                    }
                    "ECHO" => {
                        let s = format!("${}\r\n{}\r\n", res[1].len(), res[1]);

                        stream.write_all(s.as_bytes()).unwrap();
                    }
                    _ => {}
                }
            }
            Err(e) => {
                println!("{e}");
                stream.write_all(b"+EEEror\r\n").unwrap();
                break;
            }
        }
        // buf.flush().unwrap();
    }

    // println!("accepted new connection");
}

fn handle_stream_parser(str: &str) -> Vec<&str> {
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
            thread::spawn(|| handle_stream(stream));
        }
    }

    // let req = b"*1\r\n$4\r\nPING\r\n";
}
