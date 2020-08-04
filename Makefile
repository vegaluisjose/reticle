.PHONY: default
default:
	cargo fmt -- --check
	cargo clippy
	cargo test
