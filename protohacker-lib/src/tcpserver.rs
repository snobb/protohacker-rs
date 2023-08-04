use std::io;
use std::marker::Send;
use std::net::{TcpListener, TcpStream};
use std::thread;

pub struct TcpServer {
    connstr: String,
}

impl TcpServer {
    pub fn new(connstr: String) -> Self {
        TcpServer { connstr }
    }

    pub fn listen<F>(&self, handler: F) -> io::Result<()>
    where
        F: Fn(&str, Box<TcpStream>),
        F: Clone,
        F: Send + 'static,
    {
        let listener = TcpListener::bind(&self.connstr)?;

        println!("listening on: {}", self.connstr);

        loop {
            let (stream, addr) = listener.accept()?;
            println!("{addr} :: Connect");
            let handler_fn = handler.clone();

            thread::spawn(move || {
                handler_fn(&addr.to_string(), Box::new(stream));
                println!("{addr} :: Disconnect");
            });
        }
    }
}
