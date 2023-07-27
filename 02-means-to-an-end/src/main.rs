mod msg;
mod price;

use std::io;
use std::net::TcpStream;

use protohacker_lib::config::Config;
use protohacker_lib::tcpserver::TcpServer;

fn main() -> io::Result<()> {
    let config = Config::load();
    TcpServer::new(config.addr).listen(handle)
}

fn handle(addr: &str, stream: Box<TcpStream>) {
    price::Store::new(addr).handle(stream);
}
