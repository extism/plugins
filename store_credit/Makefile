
build:
	cargo build --target=wasm32-unknown-unknown
	cargo build --target=wasm32-unknown-unknown --release
	cp target/wasm32-unknown-unknown/release/store_credit.wasm ../plugins/store_credit.wasm
	cp target/wasm32-unknown-unknown/debug/store_credit.wasm ../plugins/store_credit.debug.wasm
