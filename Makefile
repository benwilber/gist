debug:
	cargo build

release:
	cargo build --release

format:
	cargo fmt

lint:
	cargo clippy

test:
	cargo test
