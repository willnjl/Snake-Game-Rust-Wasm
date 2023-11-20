use wasm_bindgen::prelude::*;

use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[wasm_bindgen(module = "/www/utils/rnd.js")]
extern "C" {
    fn rnd(max: usize) -> usize;
}

#[derive(Clone, Copy, PartialEq)]
pub struct SnakeCell(usize);

struct Snake {}

impl Snake {}

#[derive(PartialEq)]
#[wasm_bindgen]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone, Copy)]
#[wasm_bindgen]
pub enum GameState {
    Won,
    Lost,
    Played,
}
#[wasm_bindgen]
pub struct World {}

#[wasm_bindgen]
impl World {}
