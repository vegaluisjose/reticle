.PHONY: default
default:
	cargo fmt -- --check
	cargo clippy
