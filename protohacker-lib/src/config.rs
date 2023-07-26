use std::env;

pub struct Config {
    pub addr: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            addr: "[::]:8080".to_string(),
        }
    }
}

impl Config {
    pub fn load() -> Config {
        let addr = env::var("ADDRESS").unwrap_or("[::]:8080".to_string());
        Config { addr }
    }
}
