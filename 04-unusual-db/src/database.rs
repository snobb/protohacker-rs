use std::collections::HashMap;

const VERSION: &str = "wierd database v1.0";

#[derive(Default)]
pub struct Store {
    store: HashMap<String, String>,
}

pub enum DBResult {
    Insert,
    Query(Option<String>),
}

impl Store {
    pub fn new() -> Self {
        Store::default()
    }

    pub fn handle(&mut self, msg: &str) -> DBResult {
        if msg == "version" {
            return DBResult::Query(Some(format!("version={}", VERSION)));
        }

        match msg.split_once('=') {
            // insert
            Some((pfx, sfx)) => {
                if pfx != "version" {
                    println!("insert: {} => {}", pfx, sfx);
                    self.store.insert(pfx.to_string(), sfx.to_string());
                }
                DBResult::Insert
            }

            // query
            None => {
                if let Some(res) = self.store.get(msg) {
                    println!("query: {} => {}", msg, res);
                    DBResult::Query(Some(format!("{}={}", msg, res)))
                } else {
                    println!("query: {} => not found", msg);
                    DBResult::Query(None)
                }
            }
        }
    }
}
