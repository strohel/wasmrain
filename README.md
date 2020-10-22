# WASM Rain: Rain Simulation in Rust

A simple web app written in Rust compiled into WASM, thus running in browsers.

## Build

1. `cargo build --target wasm32-unknown-unknown [--release]`
2. `wasm-bindgen --target web --no-typescript --out-dir target target/wasm32-unknown-unknown/debug/wasmrain.wasm`
   - Replace `debug`by `release` if you are using the release build.

## Resources

- https://dev.to/dandyvica/wasm-in-rust-without-nodejs-2e0c
- https://rustwasm.github.io/docs/wasm-bindgen/examples/without-a-bundler.html
- https://rustwasm.github.io/docs/book/introduction.html
