prepare:
	rustup target add wasm32-unknown-unknown

build-contract:
	cd contract && cargo build --release --target wasm32-unknown-unknown
	wasm-strip contract/target/wasm32-unknown-unknown/release/counter.wasm 2>/dev/null | true
	wasm-strip contract/target/wasm32-unknown-unknown/release/logger.wasm 2>/dev/null | true

test: build-contract
	mkdir -p tests/wasm
	cp contract/target/wasm32-unknown-unknown/release/counter.wasm tests/wasm
	cp contract/target/wasm32-unknown-unknown/release/logger.wasm tests/wasm
	cd tests && cargo test -- --nocapture

clippy:
	cd contract && cargo clippy --all-targets -- -D warnings
	cd tests && cargo clippy --all-targets -- -D warnings

check-lint: clippy
	cd contract && cargo fmt -- --check
	cd tests && cargo fmt -- --check

lint: clippy
	cd contract && cargo fmt
	cd tests && cargo fmt

clean:
	cd contract && cargo clean
	cd tests && cargo clean
	rm -rf tests/wasm
