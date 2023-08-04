use regex::Regex;
use std::collections::HashMap;
use std::io::{self, BufRead, Write};
use std::net::TcpStream;
use std::sync::{Arc, Mutex};

#[derive(Default)]
pub struct GlobalStore {
    clients: HashMap<String, TcpStream>,
}

pub struct Session {
    addr: String,
    state: Arc<Mutex<GlobalStore>>,
}

impl Session {
    pub fn new(addr: &str, state: Arc<Mutex<GlobalStore>>) -> Self {
        Self {
            addr: addr.to_string(),
            state,
        }
    }

    pub fn handle(&mut self, stream: Box<TcpStream>) {
        let mut writer = stream.try_clone().expect("Cannot clone stream");
        let reader = io::BufReader::new(stream);

        let _ = writer.write("Welcome to budgetchat! What shall I call you?\n".as_bytes());
        let mut iter = reader.lines();
        let id = match iter.next() {
            Some(Ok(id)) => id,
            Some(Err(e)) => {
                println!("Error getting an id: {}", e);
                return;
            }
            None => return, // Empty response - just break
        };

        if !self.validate(&id) {
            println!("{}: invalid id", self.addr);
            let _ = writer.write(format!("Error: invalid name {}", id).as_bytes());
            return;
        }

        self.register(&id, writer);

        for line in iter {
            let line = match line {
                Ok(line) => line,
                Err(err) => {
                    println!("{}: Error: {}", self.addr, err);
                    break;
                }
            };

            self.send(&id, &line);
        }

        self.unregister(&id);
    }

    fn validate(&self, name: &String) -> bool {
        if name.is_empty() {
            return false;
        }

        let re = Regex::new(r"^[a-zA-Z0-9]*$").unwrap();
        re.is_match(name)
    }

    pub fn register(&mut self, id: &str, conn: TcpStream) {
        let mut state = self.state.lock().unwrap();

        let mut keys: Vec<&str> = vec![];
        for key in state.clients.keys() {
            keys.push(key);
        }

        let mut writer = &conn;
        let _ = writer.write(format!("* the room contains: {}\n", keys.join(", ")).as_bytes());

        for mut sock in state.clients.values() {
            let _ = sock.write(format!("* {} has entered the room\n", id).as_bytes());
        }

        state.clients.insert(id.to_string(), conn);
    }

    pub fn unregister(&mut self, id: &str) {
        let mut state = self.state.lock().unwrap();
        state.clients.remove(id);

        for mut sock in state.clients.values() {
            let _ = sock.write(format!("* {} has left the room\n", id).as_bytes());
        }
    }

    pub fn send(&mut self, id: &str, msg: &str) {
        let mut state = self.state.lock().unwrap();

        for (client_id, conn) in state.clients.iter_mut() {
            if id == client_id {
                continue;
            }

            let _ = conn.write(format!("[{}] {}\n", id, msg).as_bytes());
        }
    }
}
