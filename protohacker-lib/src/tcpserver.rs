use std::io;
use std::net::{TcpListener, TcpStream};
use std::thread;

pub struct TcpServer {
    connstr: String,
}

impl TcpServer {
    pub fn new(connstr: String) -> Self {
        TcpServer { connstr }
    }

    pub fn listen(&self, handler: fn(addr: &str, Box<TcpStream>)) -> io::Result<()> {
        let listener = TcpListener::bind(&self.connstr)?;
        println!("listening on: {}", self.connstr);

        loop {
            let (stream, addr) = listener.accept()?;
            println!("{addr} :: Connect");

            thread::spawn(move || {
                handler(&addr.to_string(), Box::new(stream));
                println!("{addr} :: Disconnect");
            });
        }
    }
}
