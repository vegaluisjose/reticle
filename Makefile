.PHONY:
test: test-vast
	cargo fmt -- --check
	cargo clean --doc
	cargo doc --no-deps
	cargo deadlinks

.PHONY: test-vast
test-vast:
	cargo build
	cargo test --release
	cargo clippy --tests
