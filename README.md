# domcrate

A tiny Rust WebAssembly project that drives the DOM with wasm-bindgen.

I wrote this to see how little it takes to touch the browser's DOM from Rust. No framework, no glue library. I just declare the browser APIs I need with `extern "C"` and call them like normal functions.

It exports two functions back to JavaScript:

- `run_alert(item)` - pops a browser alert with your text
- `create_stuff()` - makes a `div` and a `p` and drops them into the page body

## Running it

```sh
cargo build --target wasm32-unknown-unknown --release
wasm-bindgen --target web --out-dir . target/wasm32-unknown-unknown/release/domcrate.wasm

npm install
npm run serve
```

Open the page and it prints "Hello from WASM" and fires an alert.

That is the whole thing. I built it as an excuse to play with wasm-bindgen.
