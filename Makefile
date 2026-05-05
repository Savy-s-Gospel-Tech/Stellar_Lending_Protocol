.PHONY: all build test fmt lint clean

all: build

build:
	cargo build --release --target wasm32-unknown-unknown

test:
	cargo test

fmt:
	cargo fmt --all

lint:
	cargo clippy --all-targets -- -D warnings

clean:
	cargo clean
