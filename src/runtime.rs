use std::collections::HashMap;
use serde;

pub struct Runtime {
    version: u32,
    map: HashMap<String, String>
}

impl Runtime {
    pub fn new() -> Runtime {
        Runtime {
            version: 420,
            map: HashMap::new(),
        }
    }
}
