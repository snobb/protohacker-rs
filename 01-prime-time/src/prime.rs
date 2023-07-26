use serde::{Deserialize, Serialize};
use serde_json::Error;
use std::io::{self, BufRead, Write};
use std::net::TcpStream;

#[derive(Debug, Deserialize)]
#[serde(tag = "method", rename_all = "camelCase")]
enum Request {
    IsPrime { number: f64 },
    Error { msg: String },
}

#[derive(Debug, Serialize)]
#[serde(tag = "method", rename_all = "camelCase")]
enum Response {
    IsPrime { prime: bool },
    Error { msg: String },
}

pub fn handle(addr: &str, stream: Box<TcpStream>) {
    let mut writer = stream.try_clone().expect("Cannot clone stream");
    let reader = io::BufReader::new(stream);

    for line in reader.lines() {
        match line {
            Ok(line) => {
                process_msg(addr, &mut writer, serde_json::from_str::<Request>(&line));
            }
            Err(err) => {
                print!("Error: {err}")
            }
        }
    }
}

fn process_msg(addr: &str, writer: &mut dyn Write, req: Result<Request, Error>) {
    match req {
        Ok(Request::IsPrime { number }) => {
            println!("{addr}: Request: number:{number}");
            write_response(
                writer,
                Response::IsPrime {
                    prime: is_prime(number),
                },
            )
        }

        Ok(Request::Error { msg }) => {
            println!("{addr} Request-error: {msg}");
            write_response(writer, Response::Error { msg })
        }

        Err(err) => {
            println!("{addr} Error: {:?}", err);
            write_response(
                writer,
                Response::Error {
                    msg: err.to_string(),
                },
            )
        }
    }
}

fn is_prime(num: f64) -> bool {
    if num <= 1.0 {
        return false;
    }

    if num.floor() != num {
        return false; // non-integer
    }

    let sq = num.sqrt();
    println!("{sq}");
    let mut i = 2.0;

    while i <= sq {
        if num % i == 0.0 {
            return false;
        }

        i += 1.0;
    }

    true
}

#[test]
fn test_is_prime() {
    assert_eq!(is_prime(1.0), false);
    assert_eq!(is_prime(2.0), true);
    assert_eq!(is_prime(3.0), true);
    assert_eq!(is_prime(4.0), false);
    assert_eq!(is_prime(5.0), true);
    assert_eq!(is_prime(6.0), false);
    assert_eq!(is_prime(7.0), true);
    assert_eq!(is_prime(79.0), true);
    assert_eq!(is_prime(80.0), false);
}

fn write_response(writer: &mut dyn Write, res: Response) {
    let _ = match serde_json::to_string::<Response>(&res) {
        Ok(json) => writer.write(format!("{json}\n").as_bytes()),
        Err(e) => writer.write(format!("{{\"method\":\"error\",\"msg\":\"{e}\"}}\n").as_bytes()),
    };
}
