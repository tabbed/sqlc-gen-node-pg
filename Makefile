build: build.rs src/codegen.proto src/main.rs
	cargo build --release --target wasm32-wasi
