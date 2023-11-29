use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(PartialEq)]
pub enum DirectionKind {
    Up,
    Right,
    Down,
    Left,
}
