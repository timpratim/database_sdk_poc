uniffi::include_scaffolding!("database_sdk_poc");
extern crate uniffi;
use std::sync::RwLock;

#[derive(Clone)]
pub struct Person {
    pub name: String,
}

pub struct Surrealdb{
    persons: RwLock<Vec<String>>
}

impl Surrealdb {
pub fn new() -> Self {
    Surrealdb {
        persons: RwLock::new(Vec::new())
       
    }
}
pub fn create(&self,input: String) {
    self.persons.write().unwrap().push(input);
}

fn select(&self) -> Vec<String> {
    self.persons.read().unwrap().clone()
}

}

// First we need to remove the hardcoded person