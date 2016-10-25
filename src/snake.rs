use std::collections::VecDeque;

const SNAKE_LENGTH: i32 = 5;
const ROWS: u8 = 30;
const COLS: u8 = 30;

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    None,
}

pub struct Snake {
    pub p: VecDeque<(i32, i32)>,
    pub d: Direction,
}

pub struct Food {
    pub p: (i32, i32),
    pub f: FoodType,
}

// Fruit
pub enum FoodType {
    Apple,
    Grape,
    Blueberry,
    Orange,
    Banana,
}

impl Food {
    pub fn new() -> Self {
        Food {
            p: (0, 0),
            f: FoodType::Apple,
        }
    }
}

impl Snake {
    pub fn new() -> Self {
        Snake {
            p: VecDeque::new(),
            d: Direction::None,
        }
    }
    pub fn collide_with_tail(&self) -> bool {
        let h = self.p.front().unwrap();
        self.p.iter().skip(1).any(|&p| p == *h)
    }

    pub fn collide_with_food(&self, food: &Food) -> bool {
        self.p[0] == food.p
    }

    pub fn set_direction(&mut self, d: Direction) {
        match (&self.d, &d) {
            (&Direction::Up, &Direction::Down) |
            (&Direction::Down, &Direction::Up) |
            (&Direction::Left, &Direction::Right) |
            (&Direction::Right, &Direction::Left) => {}
            _ => self.d = d,
        }
    }

    pub fn collide_with_edge(&self) -> bool {
        (self.p[0].0 < 0) | (self.p[0].1 < 0) | (self.p[0].1 < 0) | (self.p[0].1 >= COLS as i32) | (self.p[0].0 >= ROWS as i32)
    }
}
