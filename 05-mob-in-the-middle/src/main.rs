mod proxy;

use proxy::Proxy;
use std::io;

use protohacker_lib::config::Config;
use protohacker_lib::tcpserver::TcpServer;

fn main() -> io::Result<()> {
    let config = Config::load();

    TcpServer::new(config.addr).listen(move |addr, stream| {
        if let Err(e) = Proxy::new(addr).handle(stream) {
            println!("Error: {}", e);
        }
    })
}
