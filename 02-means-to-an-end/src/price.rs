use std::collections::BTreeMap;
use std::io::{Read, Write};
use std::net::TcpStream;

use crate::msg::Request;

pub struct Store {
    addr: String,
    data: BTreeMap<i32, i32>,
}

impl Store {
    pub fn new(addr: &str) -> Self {
        Store {
            addr: addr.to_string(),
            data: BTreeMap::new(),
        }
    }

    pub fn handle(&mut self, mut stream: Box<TcpStream>) {
        let mut writer = stream.try_clone().expect("Cannot clone stream");
        let mut buf = [0u8; Request::SIZE];

        loop {
            match stream.read_exact(&mut buf) {
                Ok(()) => self.process_msg(&buf, &mut writer),
                Err(e) => {
                    let addr = &self.addr;
                    println!("{addr} !! error: {e}");
                }
            }
        }
    }

    fn process_msg(&mut self, buf: &[u8; Request::SIZE], stream: &mut TcpStream) {
        let req = Request::try_from(*buf);

        let addr = &self.addr;

        match req {
            Ok(Request::Insert { time, price }) => {
                println!("{addr} :: Insert time:{time} price:{price}");
                self.data.insert(time, price);
            }
            Ok(Request::Query { mintime, maxtime }) => {
                println!("{addr} :: Query mintime:{mintime} maxtime:{maxtime}");
                self.send_response(stream, self.get_average(mintime, maxtime));
            }
            Err(e) => {
                let addr = &self.addr;
                println!("{addr} !! error: {e:?}");
            }
        }
    }

    fn get_average(&self, mintime: i32, maxtime: i32) -> i32 {
        let addr = &self.addr;

        if self.data.is_empty() {
            println!("{addr} !! Warn empty data");
            return 0;
        }

        if mintime > maxtime {
            println!("{addr} !! Error: Invalid time range {mintime}-{maxtime}");
            return 0;
        }

        let mut avg: f64 = 0.0;
        let mut t: f64 = 1.0;

        for (_, &val) in self.data.range(mintime..=maxtime) {
            avg += (val as f64 - avg) / t;
            t += 1.0;
        }

        avg as i32
    }

    fn send_response(&self, stream: &mut TcpStream, price: i32) {
        let addr = &self.addr;
        println!("{addr} :: Result price:{price}");
        let _ = stream.write(&price.to_be_bytes());
    }
}
