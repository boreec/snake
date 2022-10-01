use crate::game_logic::*;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

// The width and height in pixels for the main window.
pub const WINDOW_SIZE: u32 = 800;

// The width and height in pixels to represent a CELL.
pub const CELL_SIZE: u32 = WINDOW_SIZE / BOARD_SIZE;

pub const COLOR_BACKGROUND: sdl2::pixels::Color = Color::WHITE;
pub const COLOR_APPLE: sdl2::pixels::Color = Color::RED;
pub const COLOR_SNAKE_HEAD: sdl2::pixels::Color = Color::GREEN;
pub const COLOR_SNAKE_TAIL: sdl2::pixels::Color = Color::RGB(0,200,0);

pub fn draw_game_over(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
    canvas.set_draw_color(Color::RED);
    canvas.clear();
}

pub fn clear_window(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>){
    canvas.set_draw_color(COLOR_BACKGROUND);
    canvas.clear();
}

pub fn draw_board(gs: &GameState, canvas:  &mut sdl2::render::Canvas<sdl2::video::Window>){
    // 1. Draw the board (apple, walls, ...).
    for (i, row) in gs.board.rows_iter().enumerate() {
        for (j, element) in row.enumerate() {
            match element {
                CELL::EMPTY => {draw_cell(i as i32, j as i32, COLOR_BACKGROUND, canvas)}
                CELL::APPLE => {draw_cell(i as i32, j as i32, COLOR_APPLE, canvas)}
            }
        }
    }
    //2. Draw the snake.
    draw_cell(gs.snake.pos.0 as i32, gs.snake.pos.1 as i32, COLOR_SNAKE_HEAD, canvas);

    for i in &gs.snake.tail {
        draw_cell(i.0 as i32, i.1 as i32, COLOR_SNAKE_TAIL, canvas);
    }
}

pub fn draw_cell(x: i32, y: i32, color: sdl2::pixels::Color, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>){
    canvas.set_draw_color(color);
    let cell_rect = Rect::new(x * (CELL_SIZE as i32), y * (CELL_SIZE as i32), CELL_SIZE, CELL_SIZE);
    canvas.fill_rect(cell_rect).unwrap();
}
