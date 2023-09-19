
build:
	cargo build --target=wasm32-unknown-unknown
	cargo build --target=wasm32-unknown-unknown --release
	cp target/wasm32-unknown-unknown/release/count_vowels.wasm ../plugins/count_vowels.wasm
	cp target/wasm32-unknown-unknown/debug/count_vowels.wasm ../plugins/count_vowels.debug.wasm
