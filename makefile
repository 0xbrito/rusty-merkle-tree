.PHONY: build run test fmt lint clean

build:
	cargo build

run:
	cargo run

test:
	cargo test

fmt:
	cargo fmt

lint:
	cargo clippy -- -D warnings

clean:
	cargo clean