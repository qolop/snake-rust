extern crate piston_window;
extern crate rand;


const BLACK : [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const GREEN : [f32; 4] = [0.0, 1.0, 0.0, 1.0];
const RED : [f32; 4] = [1.0, 0.0, 0.0, 1.0];


enum Direction {
    Up,
    Down,
    Left,
    Right,
    None,
}


struct Snake {
    p: std::collections::VecDeque<(i32, i32)>,
    d: Direction,
}

impl Snake {
    fn collide_with_tail(&self) -> bool {
        let h = self.p.front().unwrap();
        self.p.iter().skip(1).any(|&p| p == *h)
    }
}

struct Food {
    p: (i32, i32),
}

impl Food {
    fn spawn(&mut self, rows: u16, cols: u16) {
        use rand::{thread_rng, sample};
        let mut rng = thread_rng();
        let x = sample(&mut rng, 0..cols, 1).pop().unwrap() as i32;
        let y = sample(&mut rng, 0..rows, 1).pop().unwrap() as i32;
        self.p = (x, y);
    }
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
    update_time: f64,
    time: f64,
    state: GameState,
}

impl Game {
    fn new() -> Game {
        let mut g = Game {
            rows: 30,
            cols: 30,
            tile_size: 20,
            snake: Snake {
                p: std::collections::VecDeque::new(),
                d: Direction::None,
            },
            food: Food {
                p: (0, 0),
            },
            update_time: 0.08,
            time: 0.0,
            state: GameState::Playing,
        };
        g.food.spawn(g.rows, g.cols);
        g.snake.p.push_front((0, 0));
        g.snake.p.push_front((1, 0));
        g.snake.p.push_front((2, 0));
        g.snake.p.push_front((3, 0));
        g.snake.p.push_front((4, 0));
        g.snake.p.push_front((5, 0));
        g
    }

    fn collide_with_food(&self) -> bool {
        self.snake.p.iter().any(|&p| p == self.food.p)
    }

    fn on_update(&mut self, args: piston_window::UpdateArgs) {
        match self.state {
            GameState::Paused => return,
            GameState::GameOver => return,
            _ => {},
        }

        self.time += args.dt;
        if self.time < self.update_time {
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
            Direction::None => {},
        }
        if p.0 < 0 {
            p.0 = self.cols as i32 - 1;
        } else if p.0 >= self.cols as i32 {
            p.0 -= self.cols as i32;
        }

        if p.1 < 0 {
            p.1 = self.rows as i32 - 1;
        } else if p.1 >= self.rows as i32 {
            p.1 -= self.rows as i32;
        }

        if self.snake.collide_with_tail() {
            self.state = GameState::GameOver;
            return;
        }

        match self.snake.d {
            Direction::None => {},
            _ => {
                self.snake.p.push_front(p);
                if !self.collide_with_food() {
                    self.snake.p.pop_back();
                }
            }
        }

        if self.collide_with_food() {
            self.food.spawn(self.rows, self.cols);
        }
    }

    fn on_render(&mut self, _args: piston_window::RenderArgs, e: piston_window::PistonWindow) {
        e.draw_2d(|c, g| {
            use piston_window::{Transformed, clear, rectangle};
            let square = rectangle::square(0.0, 0.0, (1 * self.tile_size as i32) as f64);
            clear(BLACK, g);
            for &(x, y) in &self.snake.p {
                let t = c.transform.trans((x * self.tile_size as i32) as f64,
                                          (y * self.tile_size as i32) as f64);

                rectangle(GREEN, square, t, g);
            }
            rectangle(RED, square, c.transform.trans((self.food.p.0 * self.tile_size as i32) as f64,
                                                     (self.food.p.1 * self.tile_size as i32) as f64), g);

        });
    }

    fn on_input(&mut self, args: piston_window::Input) {
        use piston_window::{Button, Key};
        match args {
            piston_window::Input::Press(b) => {
                match b {
                    Button::Keyboard(Key::Up) => self.snake.d = Direction::Up,
                    Button::Keyboard(Key::Down) => self.snake.d = Direction::Down,
                    Button::Keyboard(Key::Left) => self.snake.d = Direction::Left,
                    Button::Keyboard(Key::Right) => self.snake.d = Direction::Right,
                    Button::Keyboard(Key::Return) => self.state = GameState::Paused,
                    Button::Keyboard(Key::R) => self.state = GameState::Playing,
                    _ => {},
                }
            },
            _ => {},
        }
    }

}


fn main() {
    let window: piston_window::PistonWindow = piston_window::WindowSettings::new("snake", [600, 600])
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
            _ => {},
        }
    }
}
