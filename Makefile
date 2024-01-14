.DEFAULT_GOAL = build

.PHONY: build
build: fmt
	cargo build

.PHONY: release
release: fmt
	cargo build --release

.PHONY: run
run:
	cargo run balance example.bean

.PHONY: fmt
fmt:
	cargo fmt

.PHONY: test
test:
	cargo test
