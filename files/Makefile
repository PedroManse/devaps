.PHONY: build ci

build:
	cargo build

ci: build
	cargo fmt
	cargo clippy --all-targets
	cargo test
