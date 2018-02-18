wasm:
	rm -rf target tmp

	cargo +nightly build --example wasm --target wasm32-unknown-unknown --release
	mv target/wasm32-unknown-unknown/release/examples/wasm.wasm target/wasm32-unknown-unknown/release/examples/wasm.orig.wasm
	wasm-gc target/wasm32-unknown-unknown/release/examples/wasm.orig.wasm target/wasm32-unknown-unknown/release/examples/wasm.wasm

	mkdir -p tmp
	cp target/wasm32-unknown-unknown/release/examples/wasm.wasm tmp
