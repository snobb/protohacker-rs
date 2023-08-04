mod session;

use session::{GlobalStore, Session};
use std::io;
use std::sync::{Arc, Mutex};

use protohacker_lib::config::Config;
use protohacker_lib::tcpserver::TcpServer;

fn main() -> io::Result<()> {
    let config = Config::load();
    let broker = Arc::new(Mutex::new(GlobalStore::default()));

    TcpServer::new(config.addr).listen(move |addr, stream| {
        Session::new(addr, broker.clone()).handle(stream);
    })
}
