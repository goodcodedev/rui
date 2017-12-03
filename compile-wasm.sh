rustc +nightly --target wasm32-unknown-unknown -O --crate-type=cdylib src/main.rs -o main.big.wasm
wasm-gc main.big.wasm main.wasm