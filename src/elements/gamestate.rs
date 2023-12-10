use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum GameStateKind {
    Won,
    Lost,
    Played,
    Paused,
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct GameState {
    state: Option<GameStateKind>,
}

#[wasm_bindgen]
impl GameState {
    pub fn new() -> Self {
        Self { state: None }
    }

    pub fn playing(&mut self) {
        self.state = Some(GameStateKind::Played);
    }
    pub fn lost(&mut self) {
        self.state = Some(GameStateKind::Lost);
    }
    pub fn won(&mut self) {
        self.state = Some(GameStateKind::Won);
    }

    pub fn pause(&mut self) {
        self.state = Some(GameStateKind::Paused)
    }

    pub fn get(&self) -> Option<GameStateKind> {
        self.state
    }

    pub fn status(&self) -> String {
        match self.state {
            Some(GameStateKind::Played) => String::from("You are playing"),
            Some(GameStateKind::Lost) => String::from("You Lost!"),
            Some(GameStateKind::Won) => String::from("You Won!"),
            Some(GameStateKind::Paused) => String::from("Game paused."),
            _ => String::from("Start the game"),
        }
    }

    pub fn btn(&self) -> String {
        match self.state {
            Some(GameStateKind::Played) => String::from("Pause"),
            Some(GameStateKind::Paused) => String::from("Play"),
            _ => String::from("Start New Game"),
        }
    }
}
