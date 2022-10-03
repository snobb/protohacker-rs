use std::io::{Read, Write};

pub fn handle(mut stream: impl Read + Write) {
    let mut buf = [0; 2048];

    loop {
        match stream.read(&mut buf) {
            Ok(n) => {
                if n == 0 {
                    break;
                }

                _ = stream.write(&buf[0..n]).unwrap();
            }
            Err(error) => {
                println!("Error: {error}");
                return;
            }
        }
    }
}
