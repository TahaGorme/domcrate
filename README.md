# domcrate

A tiny Rust WebAssembly project that drives the DOM with wasm-bindgen.

I wanted to see what it takes to touch the browser's DOM from Rust. The crate declares the browser APIs it needs with `extern "C"` and calls them like plain functions.

Two functions are exported to JavaScript:

- `run_alert(item)` shows a browser alert.
- `create_stuff()` adds a `div` and a `p` to the page body.

## Running it

```sh
cargo build --target wasm32-unknown-unknown --release
wasm-bindgen --target web --out-dir . target/wasm32-unknown-unknown/release/domcrate.wasm

npm install
npm run serve
```

Open the page. It prints "Hello from WASM" and shows an alert.

I built this to learn wasm-bindgen.
