use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum GameStateKind {
    Won,
    Lost,
    Played,
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct GameState {
    state: Option<GameStateKind>,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            state: Some(GameStateKind::Played),
        }
    }

    pub fn playing(&mut self) {
        self.state = Some(GameStateKind::Played);
    }
    pub fn lost(&mut self) {
        self.state = Some(GameStateKind::Won);
    }
    pub fn won(&mut self) {
        self.state = Some(GameStateKind::Lost);
    }

    pub fn get_state(&self) -> Option<GameStateKind> {
        self.state
    }
}
