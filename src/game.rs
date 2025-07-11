use piston_window::types::Color;
use piston_window::*;

use rand::prelude::*;

use crate::draw::{draw_block, draw_rectangle, draw_string};
use crate::snake::{Direction, Snake};
use crate::TEXT_COLOR;

const FOOD_COLOR: Color = [0.2235, 0.6862, 0.8431, 1.0];
const BORDER_COLOR: Color = [0.14, 0.14, 0.14, 1.0];
const GAMEOVER_COLOR: Color = [0.6235, 0.3, 0.1137, 0.3333];
const PAUSE_COLOR: Color = [0.2235, 0.6862, 0.8431, 0.3333];

const RESTART_TIME: f64 = 1.0;

pub struct Game {
    snake: Snake,

    food_exists: bool,
    food_x: i32,
    food_y: i32,
    score: i32,

    // Game Board Info
    width: i32,
    height: i32,

    // Game State Info
    game_over: bool,
    waiting_time: f64,
    pause: bool,
}

impl Game {
    pub fn new(width: i32, height: i32) -> Game {
        Game {
            snake: Snake::new(2, 2),
            food_exists: true,
            food_x: 6,
            food_y: 4,
            score: 0,
            width,
            height,
            waiting_time: 0.0,
            game_over: false,
            pause: false,
        }
    }

    pub fn key_pressed(&mut self, key: Key) {
        if self.game_over {
            return;
        }

        if key == Key::Space {
            self.pause = !self.pause;
        }

        let dir = match key {
            Key::Up => Some(Direction::Up),
            Key::Down => Some(Direction::Down),
            Key::Right => Some(Direction::Right),
            Key::Left => Some(Direction::Left),
            Key::W => Some(Direction::Up),
            Key::S => Some(Direction::Down),
            Key::D => Some(Direction::Right),
            Key::A => Some(Direction::Left),
            _ => return,
        };

        if dir.unwrap() == self.snake.head_direction().opposite() {
            return;
        }

        self.update_snake(dir);
    }

    pub fn draw(&self, con: &Context, g: &mut G2d, d: &mut GfxDevice, glyphs: &mut Glyphs) {
        self.snake.draw(con, g);

        if self.food_exists {
            draw_block(FOOD_COLOR, self.food_x, self.food_y, con, g);
        }

        draw_rectangle(BORDER_COLOR, 0, 0, self.width, 1, con, g);
        draw_rectangle(BORDER_COLOR, 0, self.height - 1, self.width, 1, con, g);
        draw_rectangle(BORDER_COLOR, 0, 0, 1, self.height, con, g);
        draw_rectangle(BORDER_COLOR, self.width - 1, 0, 1, self.height, con, g);

        if self.game_over {
            draw_rectangle(GAMEOVER_COLOR, 0, 0, self.width, self.height, con, g);
        }
        if self.pause {
            draw_rectangle(PAUSE_COLOR, 0, 0, self.width, self.height, con, g);
            draw_string(
                con,
                g,
                d,
                glyphs,
                TEXT_COLOR,
                self.width / 2 - 4,
                self.height / 2 - 2,
                format!("Paused!"),
                30,
            );
        }

        draw_string(
            con,
            g,
            d,
            glyphs,
            TEXT_COLOR,
            1,
            1,
            format!("Score: {}", self.score),
            15,
        );
    }

    pub fn update(&mut self, delta_time: f64) {
        if self.pause {
            return;
        }
        self.waiting_time += delta_time;

        if self.game_over {
            if self.waiting_time > RESTART_TIME {
                self.restart();
            }
            return;
        }

        if !self.food_exists {
            self.add_food();
        }

        if self.waiting_time > self.snake.speed {
            self.update_snake(None);
        }
    }

    fn check_eating(&mut self) {
        let (head_x, head_y): (i32, i32) = self.snake.head_position();
        if self.food_exists && self.food_x == head_x && self.food_y == head_y {
            self.food_exists = false;
            self.snake.restore_tail();
        }
    }

    fn check_if_snake_alive(&self, dir: Option<Direction>) -> bool {
        let (next_x, next_y) = self.snake.next_head(dir);
        if self.snake.overlap_tail(next_x, next_y) {
            return false;
        }

        // make sure we aren't overlapping the border (0,0,width-1,height-1)
        // if any of these are false it will return false else true
        next_x > 0 && next_y > 0 && next_x < self.width - 1 && next_y < self.height - 1
    }

    fn add_food(&mut self) {
        let mut rng = thread_rng();

        let mut new_x = rng.gen_range(1..self.width - 1);
        let mut new_y = rng.gen_range(1..self.width - 1);
        while self.snake.overlap_tail(new_x, new_y) {
            new_x = rng.gen_range(1..self.width - 1);
            new_y = rng.gen_range(1..self.width - 1);
        }

        self.food_x = new_x;
        self.food_y = new_y;
        self.food_exists = true;
        self.score += 1;
        self.snake.update_speed(self.score);
    }

    fn update_snake(&mut self, dir: Option<Direction>) {
        if self.check_if_snake_alive(dir) {
            self.snake.move_forward(dir);
            self.check_eating();
        } else {
            self.game_over = true;
        }
        self.waiting_time = 0.0;
    }

    fn restart(&mut self) {
        self.snake = Snake::new(2, 2);
        self.food_x = 6;
        self.food_y = 4;
        self.score = 0;
        self.food_exists = true;
        self.waiting_time = 0.0;
        self.game_over = false;
    }
}
