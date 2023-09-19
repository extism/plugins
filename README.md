# Plugins

Some up to date Extism plugins for use in demos and tests.

## Development

Run make on the top level folder, it will run through each folder and run `make build` in the individual folder.

```
make build
```

If you add a plug-in, make sure it has a unique name and it has a Makefile that conforms to the build schema which builds a debug and a release and copies the files to `plugins/`.

```
build:
	cargo build --target=wasm32-unknown-unknown
	cargo build --target=wasm32-unknown-unknown --release
	cp target/wasm32-unknown-unknown/release/count_vowels.wasm ../plugins/count_vowels.wasm
	cp target/wasm32-unknown-unknown/debug/count_vowels.wasm ../plugins/count_vowels.debug.wasm
```
