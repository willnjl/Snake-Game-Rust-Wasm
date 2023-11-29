use crate::elements::direction::Direction;

#[derive(Clone, Copy, PartialEq)]
pub struct SnakeCell(pub usize);

#[derive(Clone, PartialEq)]
pub struct Body(Vec<SnakeCell>);

impl Body {
    pub fn contains_cell(&self, cell: &SnakeCell) -> bool {
        self.0.contains(cell)
    }
}

pub struct Snake {
    pub direction: Direction,
    pub next_cell: Option<SnakeCell>,
    body: Body,
}

impl Snake {
    pub fn new(spawn_index: usize, starting_size: usize) -> Snake {
        let mut body: Vec<SnakeCell> = vec![];

        for i in 0..starting_size {
            body.push(SnakeCell(spawn_index - i))
        }

        Snake {
            body: Body(body),
            direction: Direction::Right,
            next_cell: None,
        }
    }

    pub fn body(&self) -> &Body {
        &self.body
    }
}
