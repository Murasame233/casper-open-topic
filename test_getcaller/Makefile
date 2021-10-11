prepare:
	rustup target add wasm32-unknown-unknown

build-caller:
	cd caller_contract && cargo build --release --target wasm32-unknown-unknown
	wasm-strip caller_contract/target/wasm32-unknown-unknown/release/caller_recorder.wasm 2>/dev/null | true
	wasm-strip caller_contract/target/wasm32-unknown-unknown/release/contract_caller.wasm 2>/dev/null | true
	wasm-strip caller_contract/target/wasm32-unknown-unknown/release/contract_caller2.wasm 2>/dev/null | true

test-caller: build-caller
	mkdir -p tests/wasm
	cp caller_contract/target/wasm32-unknown-unknown/release/caller_recorder.wasm tests/wasm
	cp caller_contract/target/wasm32-unknown-unknown/release/contract_caller.wasm tests/wasm
	cp caller_contract/target/wasm32-unknown-unknown/release/contract_caller2.wasm tests/wasm
	cd tests && cargo test --bin caller_record_tests
	mkdir -p result
	mv tests/stack* result

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
