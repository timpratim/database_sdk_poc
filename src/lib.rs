use uniffi_macros::uniffi;

#[derive(Debug)]
pub struct RecordId {
    pub tb: String,
    pub id: String,
}

#[derive(Debug)]
pub struct Person {
    pub id: RecordId,
    pub firstname: String,
    pub lastname: String,
    pub age: Option<u32>,
}

#[uniffi::export]
pub fn create(id: RecordId, firstname: String, lastname: String) -> Person {
    Person {
        id,
        firstname,
        lastname,
        age: None,
    }
}

uniffi::include_scaffolding!("database_sdk_poc");
