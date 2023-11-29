use crate::elements::snake::{Cell, Snake};
use crate::utilities::rnd;
#[derive(Clone, Copy, PartialEq)]
pub struct Reward(Cell);

impl Reward {
    pub fn new(max: usize, snake: &Snake) -> Reward {
        let reward_cell: usize;
        let mut random_cell: usize;

        loop {
            random_cell = rnd(max);
            if !snake.body.contains_cell(&Cell(random_cell)) {
                reward_cell = random_cell;
                break;
            }
        }

        Reward(Cell(reward_cell))
    }

    pub fn index(&self) -> usize {
        self.0.index()
    }

    pub fn check_consumed(reward: &Option<Reward>, head: &Cell) -> bool {
        return match reward {
            Some(reward) => return reward.index() == head.index(),
            None => false,
        };
    }
}
