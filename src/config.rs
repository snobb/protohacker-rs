use std::env;

pub struct Config {
    pub addr: String,
    pub tcp_port: i32,
    pub udp_port: i32,
}

impl Config {
    pub fn load() -> Config {
        let addr = env::var("ADDRESS").unwrap_or(String::from("0.0.0.0"));
        let tcp_port = env::var("TCP_PORT")
            .unwrap_or(String::from("8080"))
            .parse::<i32>()
            .unwrap_or(8080);
        let udp_port = env::var("UDP_PORT")
            .unwrap_or(String::from("5000"))
            .parse::<i32>()
            .unwrap_or(5000);

        Config {
            addr,
            tcp_port,
            udp_port,
        }
    }
}
