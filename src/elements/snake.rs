use crate::elements::direction::DirectionKind;

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

    pub fn add(&mut self, cell: Cell) {
        self.0.push(cell);
    }
}

pub struct Snake {
    pub direction: DirectionKind,
    pub body: Body,
    world_width: usize,
    world_size: usize,
}

impl Snake {
    pub fn new(
        spawn_index: usize,
        starting_size: usize,
        world_width: usize,
        world_size: usize,
    ) -> Snake {
        let mut body: Vec<Cell> = vec![];

        for i in 0..starting_size {
            body.push(Cell(spawn_index - i))
        }

        Snake {
            body: Body(body),
            direction: DirectionKind::Right,
            world_width,
            world_size,
        }
    }

    pub fn head(&self) -> Cell {
        self.body.0[0]
    }

    pub fn step(&mut self) {
        self.body.step();
        self.body.0[0] = self.gen_next_snake_cell(&self.direction);
    }

    fn gen_next_snake_cell(&self, direction: &DirectionKind) -> Cell {
        let width = self.world_width;
        let size = self.world_size;

        let snake_idx: usize = self.head().index();
        let row: usize = snake_idx / width;

        return match direction {
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

    pub fn consume(&mut self) {
        self.body.add(self.body.0[1]);
    }

    pub fn check_dead(&self) -> bool {
        return self.body.0[1..self.length()].contains(&self.head());
    }

    pub fn change_direction(&mut self, direction: DirectionKind) {
        let next_cell = self.gen_next_snake_cell(&direction);
        if !self.body.contains_cell(&next_cell) {
            self.direction = direction
        }
    }
}
