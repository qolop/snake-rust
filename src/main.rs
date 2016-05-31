extern crate piston_window;
extern crate rand;
use piston_window::*;
use std::collections::VecDeque;

type Color = [f32; 4];
// Colors
const GREY: Color = [0.5, 0.5, 0.5, 1.0];
const BLUE: Color = [0.0, 0.0, 1.0, 1.0];
const GREEN: Color = [0.0, 1.0, 0.0, 1.0];
const YELLOW: Color = [1.0, 1.0, 0.0, 1.0];
const PURPLE: Color = [0.5, 0.0, 0.5, 1.0];
const ORANGE: Color = [0.8, 0.5, 0.0, 1.0];
const RED: Color = [1.0, 0.0, 0.0, 1.0];

const SCORE_MULTIPLIER: i32 = 50;

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
        // The purpose of making a new VecDeque is so that we can track what coordinates have been
        // occupied by the snake, and which ones haven't. Upon eating food, a new VecDeque is generated
        // using the values on the board that the snake is not occupying. This makes it so the food
        // can't be generated on a point occupied by the snake upon generation.
        use rand::Rng;
        let mut ring: VecDeque<(i32, i32)> = VecDeque::with_capacity(900);
        for col in 0..COLS as i32 {
            for row in 0..ROWS as i32 {
                // We use a functional approach here. If we created another for loop, we would see
                // ring value be (961 * snake length) - the occupied coordinates.
                if !self.snake.p.iter().any(|&p| p == (row, col)) {
                    ring.push_front((row, col));
                }
            }
        }

        self.food.p = ring[rand::thread_rng().gen_range(0, ring.len())];

        self.food.f = match &self.food.f {
            &FoodType::Apple => FoodType::Banana,
            &FoodType::Banana => FoodType::Grape,
            &FoodType::Grape => FoodType::Blueberry,
            &FoodType::Blueberry => FoodType::Orange,
            &FoodType::Orange => FoodType::Apple,
        };
    }

    fn collide_with_food(&self) -> bool {
        self.snake.collide_with_food(&self.food)
    }

    fn on_update(&mut self, args: &UpdateArgs) {
        match &self.state {
            &GameState::Paused => return,
            &GameState::GameOver => {
                self.snake = Snake {
                    p: VecDeque::new(),
                    d: Direction::None,
                };

                for i in 0..SNAKE_LENGTH + 1 {
                    // Foo = inclusive, Bar = exclusive
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

        match &self.snake.d {
            &Direction::Up => p.1 -= 1,
            &Direction::Down => p.1 += 1,
            &Direction::Left => p.0 -= 1,
            &Direction::Right => p.0 += 1,
            &Direction::None => {}
        }

        if self.snake.collide_with_tail() | self.snake.collide_with_edge() {
            println!("Game over.");
            let score = self.snake.p.len() as i32 - SNAKE_LENGTH - 1;
            println!("You ate {} pieces of fruit for a score of {}.",
                     score,
                     score * SCORE_MULTIPLIER);
            self.state = GameState::GameOver;
            return;
        }

        match &self.snake.d {
            &Direction::None => {}
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

    fn on_render(&self, c: Context, g: &mut G2d) {
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
    }

    fn on_input(&mut self, args: &Button) {
        match args {
            &Button::Keyboard(Key::Up) |
            &Button::Keyboard(Key::W) => self.snake.set_direction(Direction::Up),
            &Button::Keyboard(Key::Down) |
            &Button::Keyboard(Key::S) => self.snake.set_direction(Direction::Down),
            &Button::Keyboard(Key::Left) |
            &Button::Keyboard(Key::A) => self.snake.set_direction(Direction::Left),
            &Button::Keyboard(Key::Right) |
            &Button::Keyboard(Key::D) => self.snake.set_direction(Direction::Right),
            &Button::Keyboard(Key::Space) => {
                self.state = match self.state {
                    GameState::Playing => GameState::Paused,
                    _ => GameState::Playing,
                }
            }
            &Button::Keyboard(Key::R) => {
                match &self.state {
                    &GameState::Paused => self.state = GameState::Playing,
                    _ => {}
                }
            }
            _ => {}
        }
    }
}


fn main() {

    let mut game = Game::new();
    let opengl = OpenGL::V3_2;
    let mut window: PistonWindow = WindowSettings::new("5n4k3", (600, 600))
        .exit_on_esc(true)
        .opengl(opengl)
        .build()
        .unwrap();

    while let Some(e) = window.next() {
        match e {
            Event::Render(_) => {
                window.draw_2d(&e, |c, g| game.on_render(c, g));
            }

            Event::Input(Input::Press(a)) => game.on_input(&a),
            Event::Update(a) => game.on_update(&a),
            _ => {}
        }
    }
}
