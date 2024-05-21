fn main() {
    uniffi::generate_scaffolding("./src/database_sdk_poc.udl").unwrap();
}