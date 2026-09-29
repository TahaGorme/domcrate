# domcrate

A tiny Rust WebAssembly project that drives the DOM with wasm-bindgen.

## What it does

The crate compiles to WebAssembly and runs in the browser. It declares the browser APIs it needs with `extern "C"` and calls them like plain functions, then exports two functions back to JavaScript:

- `run_alert(item)` shows a browser alert.
- `create_stuff()` adds a `div` and a `p` to the page body.

`index.js` imports the generated module and calls both of them.

## What I learned

- How to declare browser objects in Rust. `HTMLDocument`, `Element`, and the `document` global, wired up with `#[wasm_bindgen(method)]`, plus `getter`, `setter`, and `js_name` so the Rust names line up with the actual DOM.
- Why the crate has to be `cdylib`. That is what produces a `.wasm` the browser can load.
- What `wasm-bindgen --target web` spits out, and how the generated `domcrate.js` and `domcrate_bg.wasm` pair fit together.
- Gluing the whole thing into a page with webpack.

## Running it

```sh
cargo build --target wasm32-unknown-unknown --release
wasm-bindgen --target web --out-dir . target/wasm32-unknown-unknown/release/domcrate.wasm

npm install
npm run serve
```

Open the page. It prints "Hello from WASM" and shows an alert.

I built this to learn wasm-bindgen.
