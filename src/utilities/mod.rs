use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/www/utils/index.js")]
extern "C" {
    pub fn rnd(max: usize) -> usize;
    pub fn log(string: String);
}
