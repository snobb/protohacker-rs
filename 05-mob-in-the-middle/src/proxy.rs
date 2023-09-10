use std::env;
use std::io::{self, BufRead, Write};
use std::net::TcpStream;
use std::sync::{mpsc, mpsc::Receiver, mpsc::Sender};
use std::thread;

const EVIL_ADDRESS: &str = "7YWHMfk9JZe0LM0g1ZauHuiSxhI";
const DEFAULT_BACKEND: &str = "chat.protohackers.com:16963";

pub struct Proxy {
    addr: String,
    backend: String,
}

impl Proxy {
    pub fn new(addr: &str) -> Self {
        let backend = env::var("BE_ADDRESS").unwrap_or(DEFAULT_BACKEND.to_string());

        Proxy {
            addr: addr.to_string(),
            backend,
        }
    }

    pub fn handle(&self, stream: Box<TcpStream>) -> Result<(), io::Error> {
        let fe_writer = stream.try_clone()?;
        let fe_reader = io::BufReader::new(stream);

        let be = match TcpStream::connect(&self.backend) {
            Ok(stream) => stream,
            Err(e) => {
                println!("error: {:?}", e);
                return Err(e);
            }
        };

        let be_writer = be.try_clone().expect("Cannot clone be stream");
        let be_reader = io::BufReader::new(Box::new(be));

        let (fe_tx, fe_rx): (Sender<bool>, Receiver<bool>) = mpsc::channel();
        let (be_tx, be_rx): (Sender<bool>, Receiver<bool>) = mpsc::channel();

        let addr = self.addr.clone();
        let handler = thread::spawn(|| {
            Proxy::proxy(addr, be_rx, fe_tx, fe_reader, be_writer);
        });
        Proxy::proxy(self.addr.to_string(), fe_rx, be_tx, be_reader, fe_writer);
        handler.join().unwrap();

        Ok(())
    }

    fn proxy(
        addr: String,
        rx: Receiver<bool>,
        tx: Sender<bool>,
        reader: io::BufReader<Box<TcpStream>>,
        mut writer: TcpStream,
    ) {
        for line in reader.lines() {
            if let Ok(is_close) = rx.try_recv() {
                if is_close {
                    break;
                }
            }

            let line = match line {
                Ok(line) => line,
                Err(err) => {
                    println!("{}: Error: {}", addr, err);
                    break;
                }
            };
            println!("{}", line);

            let mut tokens: Vec<String> = Vec::new();
            for token in line.split_whitespace() {
                if Proxy::is_valid_address(token) {
                    tokens.push(EVIL_ADDRESS.to_string());
                } else {
                    tokens.push(token.to_string());
                }
            }

            _ = writer.write(&tokens.join(" ").into_bytes());
            _ = writer.write("\n".as_bytes());
        }

        _ = tx.send(true);
    }

    fn is_valid_address(addr: &str) -> bool {
        let mut chars = addr.chars();
        if let Some(c) = chars.next() {
            if c != '7' {
                return false;
            }
        }

        if addr.len() < 26 || addr.len() > 35 {
            return false;
        }

        for c in chars {
            if c.is_alphanumeric() {
                continue;
            }

            return false;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_address() {
        assert_eq!(
            Proxy::is_valid_address("7adNeSwJkMakpEcln9HEtthSRtxdmEHOT8T"),
            true
        );
        assert_eq!(Proxy::is_valid_address("7F1u3wSD5RbOHQmupo9nx4TnhQ"), true);
        assert_eq!(
            Proxy::is_valid_address("7iKDZEwPZSqIvDnHvVN2r0hUWXD5rHX"),
            true
        );
        assert_eq!(
            Proxy::is_valid_address("7LOrwbDlS8NujgjddyogWgIM93MV5N2VR"),
            true
        );
        assert_eq!(
            // too long
            Proxy::is_valid_address("7LOrwbDlS8NujgjddyogWgIM93MV5N2VR000000000000000000"),
            false
        );
        assert_eq!(
            // too short
            Proxy::is_valid_address("7LOrwbDlS8Nujg"),
            false
        );
    }
}
