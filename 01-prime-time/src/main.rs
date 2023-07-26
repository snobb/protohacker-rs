mod prime;

use protohacker_lib::config::Config;
use protohacker_lib::tcpserver::TcpServer;
use std::io;

fn main() -> io::Result<()> {
    let config = Config::load();
    TcpServer::new(config.addr).listen(prime::handle)
}
