clean:
	rm -rf target/

build:
	cargo build

build-prod:
	cargo build --release

build-wasm:
	cargo build --target wasm32-unknown-unknown

build-wasm-prod:
	cargo build --release --target wasm32-unknown-unknown
