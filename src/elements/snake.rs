use super::reward::Reward;
use crate::elements::direction::DirectionKind;
use crate::utilities::log;

#[derive(Clone, Copy, PartialEq)]
pub struct Cell(pub usize);

impl Cell {
    pub fn index(&self) -> usize {
        self.0
    }
}

#[derive(Clone, PartialEq)]
pub struct Body(pub Vec<Cell>);

impl Body {
    pub fn contains_cell(&self, cell: &Cell) -> bool {
        self.0.contains(cell)
    }

    pub fn step(&mut self) {
        let temp: Vec<Cell> = self.get().clone();
        let len = temp.len();

        for i in 1..len {
            self.0[i] = Cell(temp[i - 1].0);
        }
    }

    pub fn length(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self) -> &Vec<Cell> {
        &self.0
    }

    pub fn add(&mut self, cell: &Reward) {
        self.0.push(Cell(cell.index()));
    }
}

pub struct Snake {
    pub direction: DirectionKind,
    pub body: Body,
}

impl Snake {
    pub fn new(spawn_index: usize, starting_size: usize) -> Snake {
        let mut body: Vec<Cell> = vec![];

        for i in 0..starting_size {
            body.push(Cell(spawn_index - i))
        }

        Snake {
            body: Body(body),
            direction: DirectionKind::Right,
        }
    }

    pub fn head(&self) -> Cell {
        self.body.0[0]
    }

    pub fn step(&mut self, width: usize, size: usize) {
        self.body.step();
        self.body.0[0] = self.gen_next_snake_cell(width, size);
    }

    fn gen_next_snake_cell(&self, width: usize, size: usize) -> Cell {
        let snake_idx: usize = self.head().index();
        let row: usize = snake_idx / width;

        return match self.direction {
            DirectionKind::Right => {
                let treshold = (row + 1) * width;
                if snake_idx + 1 == treshold {
                    Cell(treshold - width)
                } else {
                    Cell(snake_idx + 1)
                }
            }
            DirectionKind::Left => {
                let treshold = row * width;
                if snake_idx == treshold {
                    Cell(treshold + (width - 1))
                } else {
                    Cell(snake_idx - 1)
                }
            }
            DirectionKind::Up => {
                let treshold = snake_idx - (row * width);
                if snake_idx == treshold {
                    Cell((size - width) + treshold)
                } else {
                    Cell(snake_idx - width)
                }
            }
            DirectionKind::Down => {
                let treshold = snake_idx + ((width - row) * width);
                if snake_idx + width == treshold {
                    Cell(treshold - ((row + 1) * width))
                } else {
                    Cell(snake_idx + width)
                }
            }
        };
    }

    pub fn length(&self) -> usize {
        self.body.length()
    }
    pub fn body(&self) -> &Vec<Cell> {
        self.body.get()
    }

    pub fn consume(&mut self, cell: &Reward) {
        self.body.add(cell)
    }

    pub fn check_dead(&self) -> bool {
        return self.body.0[1..self.length()].contains(&self.head());
    }
}
