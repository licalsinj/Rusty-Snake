// Lets you draw rectangles, 2d Graphics
use piston_window::*;
// Lets you do colors including alpha
use piston_window::types::Color;

const BLOCK_SIZE: f64 = 25.0;

// takes an integer coordinate on the game screen and returns a scaled location on the board.
pub fn to_coord(game_coord: i32) -> f64 {
    (game_coord as f64) * BLOCK_SIZE
}

pub fn to_coord_u32(game_coord: i32) -> u32 {
    to_coord(game_coord) as u32
}
// draws a block on the screen
// given a color, coordinate, and the windo context/graphics
pub fn draw_block(color: Color, x: i32, y: i32, con: &Context, g: &mut G2d) {
    let gui_x = to_coord(x);
    let gui_y = to_coord(y);

    rectangle(
        color,
        [gui_x, gui_y, BLOCK_SIZE, BLOCK_SIZE],
        con.transform,
        g,
    );
}

// variation of draw block that takes into account the width & height
pub fn draw_rectangle(
    color: Color,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    con: &Context,
    g: &mut G2d,
) {
    let gui_x = to_coord(x);
    let gui_y = to_coord(y);

    rectangle(
        color,
        [
            gui_x,
            gui_y,
            BLOCK_SIZE * (width as f64),
            BLOCK_SIZE * (height as f64),
        ],
        con.transform,
        g,
    );
}

pub fn draw_string(
    con: &Context,
    g: &mut G2d,
    d: &mut GfxDevice,
    glyphs: &mut Glyphs,
    color: Color,
    x: i32,
    y: i32,
    text: String,
    font_size: u32,
) {
    text::Text::new_color(color, font_size)
        .draw(
            text.as_str(),
            glyphs,
            &con.draw_state,
            con.transform.trans(to_coord(x), to_coord(y)),
            g,
        )
        .expect("Issue unwrapping draw text.");
    glyphs.factory.encoder.flush(d);
}
