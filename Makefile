check:
	cargo check --target wasm32-unknown-emscripten

build:
	cargo build --target wasm32-unknown-emscripten --release
	cp index.html target/wasm32-unknown-emscripten/release/index.html
	cp tileset.jpg target/wasm32-unknown-emscripten/release/tileset.jpg

run: build
	firefox target/wasm32-unknown-emscripten/release/index.html
