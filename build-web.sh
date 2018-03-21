#!/bin/bash
export EMMAKEN_CFLAGS="-s USE_SDL=2"
cargo build --release --target wasm32-unknown-emscripten
cp target/wasm32-unknown-emscripten/release/raycaster.js html
cp target/wasm32-unknown-emscripten/release/raycaster.wasm html
