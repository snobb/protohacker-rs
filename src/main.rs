mod config;
mod task00;
mod tcpserver;

use config::Config;
use std::io;
use tcpserver::TcpServer;

fn task_00(config: Config) -> io::Result<()> {
    TcpServer::new(config.addr, config.tcp_port).listen(task00::handle)
}

fn main() -> io::Result<()> {
    task_00(Config::load())
}
