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

.PHONY: lint
lint:
	cargo clippy

.PHONY: coverage
coverage:
	RUSTFLAGS='-Cinstrument-coverage' LLVM_PROFILE_FILE='test.profraw' cargo test
	grcov . --binary-path ./target/debug/deps/ -s . -t lcov --branch --ignore-not-existing --ignore '../*' --ignore "/*" -o target/coverage/tests.lcov
