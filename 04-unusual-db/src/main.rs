mod database;

use database::{DBResult, Store};
use std::net::UdpSocket;

const PORT: i16 = 5000;
const BUF_SIZE: usize = 16384;

// Task04 - Unusual Database Program - https://protohackers.com/problem/4
fn main() -> std::io::Result<()> {
    let sock = UdpSocket::bind(format!("0.0.0.0:{}", PORT))?;
    let mut db = Store::new();

    println!("listening on: 0.0.0.0:{}(udp)", PORT);

    let mut buf = [0; BUF_SIZE];

    loop {
        let (n, src) = sock.recv_from(&mut buf)?;

        let msg = String::from_utf8((buf[..n]).to_vec()).unwrap();
        if msg.len() > 1000 {
            println!("The request is too big: {}", msg.len());
            continue;
        }

        match db.handle(&msg) {
            DBResult::Query(Some(result)) => {
                if result.len() > 1000 {
                    println!("The response is too big: {}", result.len());
                    continue;
                }
                sock.send_to(result.as_bytes(), src)?;
            }

            DBResult::Query(None) => continue,
            DBResult::Insert => continue,
        }
    }
}
