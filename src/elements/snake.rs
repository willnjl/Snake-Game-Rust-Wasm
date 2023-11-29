use super::reward::Reward;
use crate::elements::direction::Direction;
use crate::utilities::log;

#[derive(Clone, Copy, PartialEq)]
pub struct SnakeCell(pub usize);

impl SnakeCell {
    pub fn index(&self) -> usize {
        self.0
    }
}

#[derive(Clone, PartialEq)]
pub struct Body(pub Vec<SnakeCell>);

impl Body {
    pub fn contains_cell(&self, cell: &SnakeCell) -> bool {
        self.0.contains(cell)
    }

    pub fn step(&mut self) {
        let temp: Vec<SnakeCell> = self.get().clone();
        let len = temp.len();

        for i in 1..len {
            self.0[i] = SnakeCell(temp[i - 1].0);
        }
    }

    pub fn length(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self) -> &Vec<SnakeCell> {
        &self.0
    }

    pub fn add(&mut self, cell: &Reward) {
        self.0.push(SnakeCell(cell.index()));
    }
}

pub struct Snake {
    pub direction: Direction,
    pub body: Body,
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
        }
    }

    pub fn head(&self) -> SnakeCell {
        self.body.0[0]
    }

    pub fn step(&mut self, width: usize, size: usize) {
        self.body.step();
        self.body.0[0] = self.gen_next_snake_cell(width, size);
    }

    fn gen_next_snake_cell(&self, width: usize, size: usize) -> SnakeCell {
        let snake_idx: usize = self.head().index();
        let row: usize = snake_idx / width;

        return match self.direction {
            Direction::Right => {
                let treshold = (row + 1) * width;
                if snake_idx + 1 == treshold {
                    SnakeCell(treshold - width)
                } else {
                    SnakeCell(snake_idx + 1)
                }
            }
            Direction::Left => {
                let treshold = row * width;
                if snake_idx == treshold {
                    SnakeCell(treshold + (width - 1))
                } else {
                    SnakeCell(snake_idx - 1)
                }
            }
            Direction::Up => {
                let treshold = snake_idx - (row * width);
                if snake_idx == treshold {
                    SnakeCell((size - width) + treshold)
                } else {
                    SnakeCell(snake_idx - width)
                }
            }
            Direction::Down => {
                let treshold = snake_idx + ((width - row) * width);
                if snake_idx + width == treshold {
                    SnakeCell(treshold - ((row + 1) * width))
                } else {
                    SnakeCell(snake_idx + width)
                }
            }
        };
    }

    pub fn length(&self) -> usize {
        self.body.length()
    }
    pub fn body(&self) -> &Vec<SnakeCell> {
        self.body.get()
    }

    pub fn consume(&mut self, cell: &Reward) {
        self.body.add(cell)
    }

    pub fn check_dead(&self) -> bool {
        return self.body.0[1..self.length()].contains(&self.head());
    }
}
