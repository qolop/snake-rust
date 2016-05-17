extern crate piston_window;
extern crate rand;
use std::collections::VecDeque;

// Colors
const GREY: [f32; 4] = [0.5, 0.5, 0.5, 1.0];
const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
const YELLOW: [f32; 4] = [1.0, 1.0, 0.0, 1.0];
const PURPLE: [f32; 4] = [0.5, 0.0, 0.5, 1.0];
const ORANGE: [f32; 4] = [0.8, 0.5, 0.0, 1.0];
const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

// Default game values
const TILE_SIZE: u8 = 20;
const ROWS: u16 = 30;
const COLS: u16 = 30;
const SNAKE_LENGTH: i32 = 5;

enum Direction {
    Up,
    Down,
    Left,
    Right,
    None,
}

struct Snake {
    p: VecDeque<(i32, i32)>,
    d: Direction,
}

impl Snake {
    fn collide_with_tail(&self) -> bool {
        let h = self.p.front().unwrap();
        self.p.iter().skip(1).any(|&p| p == *h)
    }

    fn collide_with_food(&self, food: &Food) -> bool {
        self.p.iter().any(|&p| p == food.p)
    }

    fn set_direction(&mut self, d: Direction) {
        match (&self.d, &d) {
            (&Direction::Up, &Direction::Down) |
            (&Direction::Down, &Direction::Up) |
            (&Direction::Left, &Direction::Right) |
            (&Direction::Right, &Direction::Left) => {}
            _ => self.d = d,
        }
    }

    fn collide_with_edge(&self) -> bool {
        self.p.iter().any(|&p| {
            (p.0 < 0) | (p.1 < 0) | (p.1 < 0) | (p.1 >= COLS as i32) | (p.0 >= ROWS as i32)
        })
    }
}

struct Food {
    p: (i32, i32),
    f: FoodType,
}

// impl Food {
// fn new() {
// self.p = range((0, 30), (0, 30)).collect();
// println!("{:?}", self.p);
// }
//
// fn is_free_block(&mut self) {}
// }
//

// Fruit
enum FoodType {
    Apple,
    Grape,
    Blueberry,
    Orange,
    Banana,
}

enum GameState {
    Playing,
    Paused,
    GameOver,
}

struct Game {
    rows: u16,
    cols: u16,
    tile_size: u8,
    snake: Snake,
    food: Food,
    update_freq: f64,
    time: f64,
    state: GameState,
}

impl Game {
    fn new() -> Game {
        let mut g = Game {
            rows: ROWS,
            cols: COLS,
            tile_size: TILE_SIZE,
            snake: Snake {
                p: std::collections::VecDeque::new(),
                d: Direction::None,
            },
            food: Food {
                p: (0, 0),
                f: FoodType::Apple,
            },
            update_freq: 0.08,
            time: 0.0,
            state: GameState::Playing,
        };
        g.spawn_food();
        // Initiate snake with VecDeques
        for i in 0..SNAKE_LENGTH + 1 {
            g.snake.p.push_front((i, 0));
        }
        g
    }

    fn spawn_food(&mut self) {
        use rand::{thread_rng, sample};
        let mut rng = thread_rng();
        // This is a fancy way for us to generate a random value between 0 and number
        // of columns and rows, and the 1 in the args samples 1 of them.
        // rng: &mut R, iterable: I, amount: usize
        let mut rng = thread_rng();
        // This is a fancy way for us to generate a random value between 0 and number
        // of columns and rows, and the 1 in the args samples 1 of them.
        // rng: &mut R, iterable: I, amount: usize
        let x = sample(&mut rng, 0..self.cols, 1).pop().unwrap() as i32;
        let y = sample(&mut rng, 0..self.rows, 1).pop().unwrap() as i32;
        self.food.p = (x, y); // Find out wtf sample does.

        self.food.f = match self.food.f {
            FoodType::Apple => FoodType::Banana,
            FoodType::Banana => FoodType::Grape,
            FoodType::Grape => FoodType::Blueberry,
            FoodType::Blueberry => FoodType::Orange,
            FoodType::Orange => FoodType::Apple,
        };
        // for c in self.snake.p {
        // match c {
        // (x, y) => {
        // println!("{:?}", (x, y));
        // println!("Initiated new value");
        // x = sample(&mut rng, 0..self.cols, 1).pop().unwrap() as i32;
        // y = sample(&mut rng, 0..self.rows, 1).pop().unwrap() as i32;
        // self.food.p = (x, y);
        // }
        // }
        // }
        //
    }

    fn collide_with_food(&self) -> bool {
        self.snake.collide_with_food(&self.food)
    }

    fn on_update(&mut self, args: piston_window::UpdateArgs) {
        match self.state {
            GameState::Paused => return,
            GameState::GameOver => {
                self.snake = Snake {
                    p: std::collections::VecDeque::new(),
                    d: Direction::None,
                };

                for i in 0..SNAKE_LENGTH + 1 {
                    self.snake.p.push_front((i, 0));
                }
                self.state = GameState::Playing;
                println!("Restarted.");
                return;
            }
            _ => {}
        }

        self.time += args.dt;

        if self.time < self.update_freq {
            return;
        } else {
            self.time = 0.0
        }

        let mut p = self.snake.p.front().unwrap().clone();

        match self.snake.d {
            Direction::Up => p.1 -= 1,
            Direction::Down => p.1 += 1,
            Direction::Left => p.0 -= 1,
            Direction::Right => p.0 += 1,
            Direction::None => {}
        }

        if self.snake.collide_with_tail() | self.snake.collide_with_edge() {
            println!("Game over.");
            let score = self.snake.p.len() as i32 - SNAKE_LENGTH - 1;
            println!("You ate {} pieces of fruit for a score of {}.",
                     score,
                     score * 50);
            self.state = GameState::GameOver;
            return;
        }


        match self.snake.d {
            Direction::None => {}
            _ => {
                self.snake.p.push_front(p);
                if !self.collide_with_food() {
                    self.snake.p.pop_back();
                }
            }
        }

        if self.collide_with_food() {
            self.spawn_food();
        }
    }

    fn on_render(&mut self, _args: piston_window::RenderArgs, e: piston_window::PistonWindow) {
        e.draw_2d(|c, g| {
            use piston_window::{Transformed, clear, rectangle};
            let square = rectangle::square(0.0, 0.0, (1 * self.tile_size as i32) as f64);
            clear(GREY, g);

            for &(x, y) in &self.snake.p {
                let t = c.transform.trans((x * self.tile_size as i32) as f64,
                                          (y * self.tile_size as i32) as f64);
                rectangle(GREEN, square, t, g);
            }

            let x = (self.food.p.0 * self.tile_size as i32) as f64;
            let y = (self.food.p.1 * self.tile_size as i32) as f64;
            let food_color = match &self.food.f {
                &FoodType::Apple => RED,
                &FoodType::Banana => YELLOW,
                &FoodType::Grape => PURPLE,
                &FoodType::Blueberry => BLUE,
                &FoodType::Orange => ORANGE,
            };
            rectangle(food_color, square, c.transform.trans(x, y), g);
        });
    }

    fn on_input(&mut self, args: piston_window::Input) {
        use piston_window::{Button, Key};
        match args {
            piston_window::Input::Press(b) => {
                match b {
                    Button::Keyboard(Key::Up) => self.snake.set_direction(Direction::Up),
                    Button::Keyboard(Key::Down) => self.snake.set_direction(Direction::Down),
                    Button::Keyboard(Key::Left) => self.snake.set_direction(Direction::Left),
                    Button::Keyboard(Key::Right) => self.snake.set_direction(Direction::Right),
                    Button::Keyboard(Key::Space) => {
                        self.state = match self.state {
                            GameState::Playing => GameState::Paused,
                            _ => GameState::Playing,
                        }
                    }
                    Button::Keyboard(Key::R) => {
                        match self.state {
                            GameState::Paused => self.state = GameState::Playing,
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}


fn main() {
    let window: piston_window::PistonWindow = piston_window::WindowSettings::new("snake",
                                                                                 [600, 600])
        .exit_on_esc(true)
        .build()
        .unwrap();
    let mut game = Game::new();

    for e in window {
        use piston_window::Event;
        match e.event {
            Some(Event::Update(a)) => game.on_update(a),
            Some(Event::Render(a)) => game.on_render(a, e),
            Some(Event::Input(i)) => game.on_input(i),
            _ => {}
        }
    }
}
