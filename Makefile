.PHONY:
test: test-reticle
	cargo fmt -- --check
	cargo clean --doc
	cargo doc --no-deps
	cargo deadlinks

.PHONY: test-reticle
test-reticle:
	cargo build
	cargo test --release
	cargo clippy --tests
