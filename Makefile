.PHONY: build

build: target/wasm32-wasi/release/sqlc-gen-node-pg.wasm
	openssl dgst -sha256 target/wasm32-wasi/release/sqlc-gen-node-pg.wasm

target/wasm32-wasi/release/sqlc-gen-node-pg.wasm: build.rs src/codegen.proto src/main.rs
	cargo build --release --target wasm32-wasi
