use wasm_bindgen::prelude::*;

use crate::elements::gamestate::GameState;
use crate::elements::gamestate::GameStateKind;
use crate::elements::reward::Reward;
use crate::elements::snake::Snake;

#[wasm_bindgen]
pub struct World {
    width: usize,
    size: usize,
    snake: Snake,
    points: usize,
    reward: Option<Reward>,
    pub state: GameState,
}

#[wasm_bindgen]
impl World {
    pub fn new(width: usize, starting_index: usize, starting_size: usize) -> World {
        let size: usize = width * width;
        let snake: Snake = Snake::new(starting_index, starting_size);
        let reward = Some(Reward::new(size, &snake));
        let state = GameState::new();

        World {
            reward,
            width,
            size,
            snake,
            state,
            points: 0,
        }
    }

    pub fn step(&mut self) {
        match self.state.get_state() {
            Some(GameStateKind::Played) => {
                self.snake.step(self.width, self.size);

                if Reward::check_consumed(&self.reward, &self.snake.head()) {
                    self.consume_reward();
                }

                if self.snake.check_dead() {
                    self.lose();
                }
            }
            _ => {}
        }
    }

    fn consume_reward(&mut self) {
        if self.snake.length() < self.size {
            self.snake.consume(&self.reward.unwrap());
            self.points += 1;
        } else {
            self.win();
        }
    }

    fn win(&mut self) {
        self.reward = None;
        self.state.won();
    }

    fn lose(&mut self) {
        self.state.lost()
    }
    pub fn width(&self) -> usize {
        self.width
    }

    pub fn size(&self) -> usize {
        self.size
    }
}
