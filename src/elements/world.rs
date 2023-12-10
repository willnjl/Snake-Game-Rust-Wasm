use wasm_bindgen::prelude::*;

use crate::elements::direction::DirectionKind;
use crate::elements::gamestate::GameState;
use crate::elements::gamestate::GameStateKind;
use crate::elements::reward::Reward;
use crate::elements::snake::Cell;
use crate::elements::snake::Snake;
use crate::utilities::rnd;

#[wasm_bindgen]
pub struct World {
    width: usize,
    size: usize,
    snake: Snake,
    points: usize,
    reward: Option<Reward>,
    starting_size: usize,
    state: GameState,
}

#[wasm_bindgen]
impl World {
    pub fn new(width: usize, starting_size: usize) -> World {
        let size: usize = width * width;
        let starting_index = World::starting_index(size);
        let snake: Snake = Snake::new(starting_index, starting_size, width, size);
        let reward = Some(Reward::new(size, &snake));
        let state: GameState = GameState::new();

        World {
            reward,
            width,
            size,
            snake,
            state,
            starting_size,
            points: 0,
        }
    }

    pub fn get_direction(&self) -> DirectionKind {
        self.snake.get_direction()
    }

    pub fn game_status_text(&self) -> String {
        self.state.status()
    }

    pub fn game_btn_text(&self) -> String {
        self.state.btn()
    }

    pub fn handle_click(&mut self) {
        match self.state.get() {
            None => self.state.playing(),
            Some(GameStateKind::Paused) => self.state.playing(),
            Some(GameStateKind::Played) => self.state.pause(),
            _ => self.restart(),
        }
    }

    pub fn step(&mut self) {
        match self.state.get() {
            Some(GameStateKind::Played) => {
                self.snake.step();
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

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn size(&self) -> usize {
        self.size
    }

    /**
     * *const is raw pointer
     * borrowing rules dont apply
     */
    pub fn snake_cells(&self) -> *const Cell {
        self.snake.body().as_ptr()
    }

    pub fn snake_length(&self) -> usize {
        self.snake.length()
    }

    pub fn reward_cell(&self) -> usize {
        return match self.reward {
            Some(cell) => cell.index(),
            _ => self.size() + 1,
        };
    }

    pub fn change_snake_direction(&mut self, direction: DirectionKind) {
        self.snake.change_direction(direction)
    }

    pub fn points(&self) -> usize {
        self.points
    }

    fn consume_reward(&mut self) {
        if self.snake.length() < self.size {
            self.snake.consume();
            self.points += 1;
            self.reward = Some(Reward::new(self.size, &self.snake))
        } else {
            self.win();
        }
    }

    fn win(&mut self) {
        self.reward = None;
        self.state.won();
        self.reward = None;
    }

    fn lose(&mut self) {
        self.state.lost();
        self.reward = None;
    }

    fn starting_index(max: usize) -> usize {
        rnd(max)
    }

    fn restart(&mut self) {
        self.points = 0;
        self.snake = Snake::new(
            World::starting_index(self.size),
            self.starting_size,
            self.width,
            self.size,
        );
        self.reward = Some(Reward::new(self.size, &self.snake));
        self.state.playing();
    }
}
