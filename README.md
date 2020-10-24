# WASM Rain: Rain Simulation in Rust

A simple web app written in Rust compiled into WASM, thus running in browsers.

Demo: https://storage.googleapis.com/strohel-pub/wasmrain/index.html

## Build

1. `cargo build --target wasm32-unknown-unknown [--release]`
2. `wasm-bindgen --target web --no-typescript --out-dir target target/wasm32-unknown-unknown/debug/wasmrain.wasm`
   - Replace `debug`by `release` if you are using the release build.

Note that building requires the solver from https://github.com/strohel/snapwater
which is not currently public. Feel free to ping me.

Deploy (for me):
1. `gsutil -h "Cache-Control:no-cache,max-age=0" cp index.html gs://strohel-pub/wasmrain/`
2. `gsutil -h "Cache-Control:no-cache,max-age=0" cp target/wasmrain.js target/wasmrain_bg.wasm gs://strohel-pub/wasmrain/target/`

## Resources

- https://dev.to/dandyvica/wasm-in-rust-without-nodejs-2e0c
- https://rustwasm.github.io/docs/wasm-bindgen/examples/without-a-bundler.html
- https://rustwasm.github.io/docs/book/introduction.html
