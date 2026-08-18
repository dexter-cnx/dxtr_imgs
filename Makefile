.PHONY: setup fmt check test clippy run release macos-bundle

setup:
	cargo fetch

fmt:
	cargo fmt --all -- --check

check:
	cargo check --workspace

test:
	cargo test --workspace

clippy:
	cargo clippy --workspace --all-targets -- -D warnings

run:
	cargo run -p dxtr-imgs-ui-gpui

release:
	cargo build --workspace --release

macos-bundle:
	@echo "M0 placeholder: .app bundling is implemented in the macOS productization milestone."
	@echo "Use 'make release' for the current release binary."
