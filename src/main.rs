#![allow(unused_imports)]
use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

fn handle_stream(mut stream: TcpStream) {
    let mut buf =Vec::new();

    loop {
        match stream.read(&mut buf) {
            Ok(res) => {
            
            println!("==> {:?}",String::from_utf8(buf.clone()));
               
                stream.write_all(b"+PONG\r\n").unwrap();
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
}
