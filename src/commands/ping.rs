use std::{io::Write, net::TcpStream};


pub fn handle_ping(stream:&mut TcpStream){


       stream.write_all(b"+PONG\r\n").unwrap();


}