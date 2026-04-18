use std::{io::Write, net::TcpStream};

pub fn handle_echo(stream: &mut TcpStream, res: Option<&&str>) {
    match res {
        Some(str) => {
            let len = str.len();

            let s = format!("${}\r\n{}\r\n", len, str);

            stream.write_all(s.as_bytes()).unwrap();
        }
        None => todo!(),
    }
}
