.PHONY: run
run:
	RUST_LOG=debug cargo run balance example.bean

.PHONY: cov
cov:
	RUSTFLAGS='-Cinstrument-coverage' LLVM_PROFILE_FILE='target/prof/cargo-test-%p-%m.profraw' cargo test
	grcov . -s . --binary-path ./target/debug/ -t lcov,html --branch -o target/coverage/
	open target/coverage/html/index.html

.PHONY: py
py:
	rye run maturin develop --pip-path ~/.rye/self/bin/pip
