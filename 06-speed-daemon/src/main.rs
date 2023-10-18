mod speed;

use speed::{Session, Store};
use std::io;
use std::sync::{Arc, Mutex};

use protohacker_lib::config::Config;
use protohacker_lib::tcpserver::TcpServer;

fn main() -> io::Result<()> {
    let config = Config::load();
    let store = Arc::new(Mutex::new(Store::default()));

    TcpServer::new(config.addr).listen(move |addr, stream| {
        Session::new(addr, store.clone()).handle(stream);
    })
}
