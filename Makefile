all: target/aarch64-apple-darwin/debug/libdatabase_sdk_poc.dylib
	CGO_LDFLAGS='-ldatabase_sdk_poc -L/Users/pratimbhosale/Desktop/surrealdb_projects/gosdkpoc/database_sdk_poc/target/aarch64-apple-darwin/debug -lm -ldl' CGO_ENABLED=1 go run main.go

target/aarch64-apple-darwin/debug/libdatabase_sdk_poc.dylib: src/lib.rs Cargo.toml
	cargo build --target aarch64-apple-darwin

clean:
	rm -rf target