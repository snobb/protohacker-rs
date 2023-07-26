use std::io::{Read, Write};
use std::net::TcpStream;

pub fn handle(addr: &str, mut stream: Box<TcpStream>) {
    let mut buf = [0; 2048];

    println!("Handling request from {addr}");

    loop {
        match stream.read(&mut buf) {
            Ok(n) => {
                if n == 0 {
                    break;
                }

                _ = stream.write(&buf[0..n]).unwrap();
            }
            Err(error) => {
                println!("Error: {error}");
                return;
            }
        }
    }
}
