#![allow(unused_imports)]
use std::{
    io::{Read, Write},
    net::TcpListener,
};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment the code below to pass the first stage

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                stream.write_all(b"+PONg\r\n").unwrap();
                let mut buf = Vec::new();
                // let res=   stream.peek(&mut buf).expect("Byte see");

                 stream.read_to_end(&mut buf).unwrap();


                  stream.write_all(b"this is pong foin\r\n").unwrap();

                match String::from_utf8(buf) {
                    Ok(str) => {
                        if str.as_str() == "PING" {
                            stream.write_all(b"+PONG\r\n").unwrap();
                        }
                    }
                    Err(_e) => {}
                }

                // println!("{:?}  ,res==> {:?}",stream,String::from_utf8(buf));

                // println!("accepted new connection");
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
