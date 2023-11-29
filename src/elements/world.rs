use wasm_bindgen::prelude::*;

use crate::elements::reward::Reward;
use crate::elements::snake::Snake;

#[wasm_bindgen]
pub struct World {
    width: usize,
    size: usize,
    snake: Snake,
    points: usize,
    reward: Option<Reward>,
}

#[wasm_bindgen]
impl World {
    pub fn new(width: usize, starting_index: usize, starting_size: usize) -> World {
        let size: usize = width * width;
        let snake: Snake = Snake::new(starting_index, starting_size);
        let reward = Some(Reward::new(size, &snake));

        World {
            reward,
            width,
            size,
            snake,
            points: 0,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn size(&self) -> usize {
        self.size
    }
}
