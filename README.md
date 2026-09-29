# domcrate

A tiny Rust WebAssembly project that drives the DOM with wasm-bindgen.

## What it does

The crate compiles to WebAssembly and runs in the browser. It uses `web-sys` for the DOM and exports two functions back to JavaScript:

- `run_alert(item)` shows a browser alert.
- `create_stuff()` adds a `div` and a `p` to the page body.

`index.js` imports the generated module and calls both of them.

## What I learned

- How to reach the DOM from Rust with `web-sys`. Getting the `Window` and `Document`, creating `Element`s, and turning on the right features in `Cargo.toml`.
- Building nodes with `create_element`, setting `innerHTML`, and wiring them together with `append_child`, plus handling the `Option` and `Result` values those calls return.
- Declaring a browser function by hand with `#[wasm_bindgen] extern "C"`, which is how `alert` is wired up.
- Why the crate has to be `cdylib`, since that is what produces the `.wasm` the browser loads.
- What `wasm-bindgen --target web` emits, and how `domcrate.js`, `domcrate_bg.js`, and `domcrate_bg.wasm` fit together.
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
