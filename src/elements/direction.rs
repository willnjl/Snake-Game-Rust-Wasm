use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(PartialEq, Clone, Copy)]
pub enum DirectionKind {
    Up,
    Right,
    Down,
    Left,
}
