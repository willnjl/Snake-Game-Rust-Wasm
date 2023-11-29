use crate::elements::snake::SnakeCell;
use crate::utilities::rnd::rnd;

use super::snake::Snake;

pub struct Reward(usize);

impl Reward {
    pub fn new(max: usize, snake: &Snake) -> Reward {
        let reward_cell: usize;
        let mut random_cell: usize;

        loop {
            random_cell = rnd(max);
            if !snake.body().contains_cell(&SnakeCell(random_cell)) {
                reward_cell = random_cell;
                break;
            }
        }

        Reward(reward_cell)
    }
}
