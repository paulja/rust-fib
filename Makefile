.PHONY: all build-wasm build-wasm-release fib-wasm fib-http clean

# Build the WASM module (debug) and copy it into fib-wasm/ for embedding.
build-wasm:
	cargo build --target wasm32-unknown-unknown -p fib-core
	cp target/wasm32-unknown-unknown/debug/fib_core.wasm fib-wasm/fib_core.wasm

# Build the WASM module (release) and copy it into fib-wasm/ for embedding.
build-wasm-release:
	cargo build --release --target wasm32-unknown-unknown -p fib-core
	cp target/wasm32-unknown-unknown/release/fib_core.wasm fib-wasm/fib_core.wasm

# Build the wasm-bindgen package for the browser comparison page in fib-http.
build-wasm-browser:
	wasm-pack build fib-core --target web -- --features wasm

# Build the fib-wasm Go binary (requires build-wasm or build-wasm-release first).
fib-wasm: build-wasm
	go -C fib-wasm build -o ../bin/fib-wasm .

# Build the fib-http Rust binary (requires build-wasm-browser first).
fib-http: build-wasm-browser
	cargo build --release -p fib-http

clean:
	cargo clean
	rm -f fib-wasm/fib_core.wasm bin/fib-wasm
