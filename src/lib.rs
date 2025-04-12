// This file contains the Rust code that will be compiled to WebAssembly. It exports functions that can be called from JavaScript.

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    let greeting = format!("Hello, {}!", name);
    alert(&greeting);
}