extern crate piston_window;
extern crate rand;

mod draw;
mod game;
mod snake;

use piston_window::types::Color;
use piston_window::*;

use draw::to_coord_u32;
use game::Game;

const BACK_COLOR: Color = [0.8666, 0.8666, 0.8666, 1.0];
const TEXT_COLOR: Color = [0.992, 0.75, 0.207, 1.0];

fn main() {
    let (width, height) = (30, 30);

    let mut window: PistonWindow =
        WindowSettings::new("Snake", [to_coord_u32(width), to_coord_u32(height)])
            .exit_on_esc(true)
            .build()
            .expect("issue making window.");

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .expect("issue finding folder: assets/");
    let ref font = assets.join("BigBlueTermPlusNerdFontMono-Regular.ttf");
    let mut glyphs = window.load_font(font).expect("Issue loading font.");

    let mut game = Game::new(width, height);
    // window.next() cleans the window once the new action happens
    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        }
        window.draw_2d(&event, |c, g, d| {
            clear(BACK_COLOR, g);
            game.draw(&c, g, d, &mut glyphs);
        });

        event.update(|arg| {
            game.update(arg.dt);
        });
    }
}
