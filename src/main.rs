#![allow(unused_imports)]
use std::{io::{Read, Write}, net::TcpListener};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment the code below to pass the first stage
    
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {

               let mut buf = String::new();
            // let res=   stream.peek(&mut buf).expect("Byte see");

            let res=stream.read_to_string(&mut buf);

                println!("{:?}  ,res==> {:?}",stream,res);




                stream.write_all(b"+PONG\r\n").unwrap();
                // println!("accepted new connection");
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
