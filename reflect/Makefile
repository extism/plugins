
build:
	cargo build --target=wasm32-unknown-unknown
	cargo build --target=wasm32-unknown-unknown --release
	cp target/wasm32-unknown-unknown/release/reflect.wasm ../plugins/reflect.wasm
	cp target/wasm32-unknown-unknown/debug/reflect.wasm ../plugins/reflect.debug.wasm
