use std::collections::HashMap;
use std::error::Error;

pub struct Context {
    pub ctx: HashMap<String, String>
}

impl Context {
    pub fn new() -> Self {
        Self {
            ctx: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: String, value: String) {
        self.ctx.insert(key, value);
    }

    pub fn get(&mut self, key: String) -> Option<String>{
        if let Some(value) = self.ctx.get(&key) {
            Some(value.to_string())
        } else {
            None
        }
    }
}
