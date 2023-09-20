use raylib::color::Color;
use raylib::drawing::{RaylibDraw, RaylibDrawHandle};
use crate::game::GameState;

const BLOCK_SIZE: i32 = 20;

fn draw_board(game_state: &GameState, d: &mut RaylibDrawHandle) {
    for y in 0..game_state.board.len() {
        d.draw_line(0, y as i32 * BLOCK_SIZE, 10 * BLOCK_SIZE, y as i32 * BLOCK_SIZE, Color::DARKGRAY);
        for x in 0..game_state.board[0].len() {
            d.draw_line(x as i32 * BLOCK_SIZE, 0, x as i32 * BLOCK_SIZE, 20 * BLOCK_SIZE, Color::DARKGRAY);
        }
    }
}

fn draw_locked_pieces(game_state: &GameState, d: &mut RaylibDrawHandle) {
    for (y, row) in game_state.board.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if cell != 0 {
                let x_pos = x as i32 * BLOCK_SIZE;
                let y_pos = y as i32 * BLOCK_SIZE;
                d.draw_rectangle(x_pos, y_pos, BLOCK_SIZE, BLOCK_SIZE, Color::RED);
            }
        }
    }
}

pub fn draw(game_state: &GameState, d: &mut RaylibDrawHandle) {
    draw_board(game_state, d);

    if !game_state.is_game_over {
        draw_locked_pieces(game_state, d);
        draw_current_piece(game_state, d);
        draw_next_piece(game_state, d);
    }

    draw_ui(&game_state, d);
}

fn draw_ui(game_state: &&GameState, d: &mut RaylibDrawHandle) {
    d.draw_text("Next piece:", 12 * BLOCK_SIZE, 0, 20, Color::WHITE);
    d.draw_text(&format!("Score: {}", game_state.score), 12 * BLOCK_SIZE, 5 * BLOCK_SIZE, 20, Color::WHITE);

    if game_state.is_paused {
        d.draw_text("Paused", 12 * BLOCK_SIZE, 10 * BLOCK_SIZE, 20, Color::WHITE);
    }

    if game_state.is_game_over {
        d.draw_text("Game Over", 12 * BLOCK_SIZE, 10 * BLOCK_SIZE, 20, Color::WHITE);
    }
}

fn draw_next_piece(game_state: &GameState, d: &mut RaylibDrawHandle) {
    for (y, row) in game_state.next_piece.shape.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if cell != 0 {
                let x_pos = (x as i32 + 12) * BLOCK_SIZE;
                let y_pos = (y as i32 + 1) * BLOCK_SIZE;
                d.draw_rectangle(x_pos, y_pos, BLOCK_SIZE, BLOCK_SIZE, Color::DARKGREEN);
            }
        }
    }
}

fn draw_current_piece(game_state: &GameState, d: &mut RaylibDrawHandle) {
    for (y, row) in game_state.current_piece.shape.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if cell != 0 {
                let x_pos = (x as i32 + game_state.current_piece.x) * BLOCK_SIZE;
                let y_pos = (y as i32 + game_state.current_piece.y) * BLOCK_SIZE;
                d.draw_rectangle(x_pos, y_pos, BLOCK_SIZE, BLOCK_SIZE, Color::GREEN);
            }
        }
    }
}