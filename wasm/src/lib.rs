use wasm_bindgen::prelude::*;
use lib::print;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    log(&format!("Hello, {} and {}!", print().as_str(), name));
}
