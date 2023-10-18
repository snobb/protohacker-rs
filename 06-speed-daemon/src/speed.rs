use std::{
    net::TcpStream,
    sync::{Arc, Mutex},
};

#[derive(Default)]
pub struct Store {}

pub struct Session {
    addr: String,
    store: Arc<Mutex<Store>>,
}

impl Session {
    pub fn new(addr: &str, store: Arc<Mutex<Store>>) -> Self {
        Self {
            addr: addr.to_string(),
            store,
        }
    }

    pub fn handle(&mut self, stream: Box<TcpStream>) {}
}
