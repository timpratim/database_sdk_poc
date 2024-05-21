uniffi::include_scaffolding!("database_sdk_poc");
extern crate uniffi;
use std::sync::RwLock;

#[derive(Clone)]
pub struct Person {
    pub name: String,
}

pub struct Surrealdb{
    persons: RwLock<Vec<Person>>
}

impl Surrealdb {
pub fn new() -> Self {
    Surrealdb {
        persons: RwLock::new(Vec::new())
       
    }
}
pub fn create(&self,person: Person) {
    self.persons.write().unwrap().push(person);
}

fn select(&self) -> Vec<Person> {
    self.persons.read().unwrap().clone()
}


}

