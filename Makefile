.PHONY: default
default:
	bash ci/run.sh

.PHONY: lint
lint:
	cargo fmt -- --check
	cargo clippy --all-targets --all-features -- -D warnings
	pytest --pycodestyle -x -v ci
