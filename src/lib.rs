extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
use web_sys::{Document, Element};

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn run_alert(item: &str) {
    alert(item);
}

#[wasm_bindgen]
pub fn create_stuff() {
    let window = web_sys::window().expect("no global window exists");

    let document: Document = window.document().expect("no document exists");

    let div: Element = document
        .create_element("div")
        .expect("failed to create div");

    let p: Element = document.create_element("p").expect("unable to create p");

    p.set_inner_html("Hello from WASM");
    div.append_child(&p).expect("failed to append p to div");

    document
        .body()
        .expect("document has no body")
        .append_child(&div)
        .expect("failed to append div to document");
}
