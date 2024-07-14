build:
	cargo build
	cargo build --release

	cargo build --target wasm32-wasi
	cargo build --target wasm32-wasi --release

	@find . -type f -name Makefile | grep -v "^./Makefile" | while read dir; do \
		echo "Executing in $$dir"; \
		(cd `dirname $$dir` && $(MAKE)); \
	done

# 	wasm32-unknown-unknown
	@ls target/wasm32-unknown-unknown/release/*.wasm | while read name; do \
		cp $$name plugins/; \
	done
	@ls target/wasm32-unknown-unknown/debug/*.wasm | while read name; do \
		export newFilename=$$(echo $$name | sed 's/\.wasm/.debug.wasm/g'); \
		cp $$name plugins/$$(basename $$newFilename); \
	done

# 	wasm32-wasi
	@ls target/wasm32-wasi/release/*.wasm | while read name; do \
		cp $$name plugins/; \
	done
	@ls target/wasm32-wasi/debug/*.wasm | while read name; do \
		export newFilename=$$(echo $$name | sed 's/\.wasm/.debug.wasm/g'); \
		cp $$name plugins/$$(basename $$newFilename); \
	done

	@for i in *.wat; do wasm-tools parse $$i -o plugins/$${i%.wat}.wasm; done
