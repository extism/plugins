build:
	cargo build
	cargo build --release
	@find . -type f -name Makefile | grep -v "^./Makefile" | while read dir; do \
		echo "Executing in $$dir"; \
		(cd `dirname $$dir` && $(MAKE)); \
	done
	@ls target/wasm32-unknown-unknown/release/*.wasm | while read name; do \
		cp $$name plugins/; \
	done
	@ls target/wasm32-unknown-unknown/debug/*.wasm | while read name; do \
		export newFilename=$$(echo $$name | sed 's/\.wasm/.debug.wasm/g'); \
		cp $$name plugins/$$(basename $$newFilename); \
	done
