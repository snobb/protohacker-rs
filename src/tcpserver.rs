use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

// Read + Write hack explained at the StackOverflow article below:
// https://stackoverflow.com/questions/68316251/rust-use-common-trait-type-for-different-streams-tcp-stream-tls-stream
pub trait ReadWriter: Read + Write {}
impl ReadWriter for TcpStream {}

pub struct TcpServer {
    connstr: String,
}

impl TcpServer {
    pub fn new(addr: String, port: i32) -> Self {
        TcpServer {
            connstr: format!("{addr}:{port}"),
        }
    }

    pub fn listen(&self, handler: fn(Box<dyn ReadWriter>)) -> io::Result<()> {
        let listener = TcpListener::bind(&self.connstr)?;
        println!("listening on: {}", self.connstr);

        loop {
            let (stream, addr) = listener.accept()?;
            println!("{addr} :: Connect");

            thread::spawn(move || {
                handler(Box::new(stream));
                println!("{addr} :: Disconnect");
            });
        }
    }
}
