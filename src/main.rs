#![windows_subsystem = "windows"]
use raylib::prelude::{Color, KeyboardKey, RaylibDraw};

mod game;
mod renderer;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(360, 410)
        .title("Tetris")
        .vsync()
        .build();

    rl.set_target_fps(30);

    let mut game_state = game::GameState::new();
    let mut accumulator = 0.0;
    let mut input_accumulator = 0.0;
    let mut game_over_accumulator = 0.0;
    let update_rate = 1.0 / 2.0; // 2 times per second
    let input_rate = 1.0 / 10.0; // 10 times per second
    let game_over_time = 2.0; // 2 seconds

    while !rl.window_should_close() {
        let elapsed_time = rl.get_frame_time();
        accumulator += elapsed_time;
        input_accumulator += elapsed_time;

        if input_accumulator >= input_rate {}

        if rl.is_key_down(KeyboardKey::KEY_LEFT) {
            game_state.move_piece(-1, 0);
        }
        if rl.is_key_down(KeyboardKey::KEY_RIGHT) {
            game_state.move_piece(1, 0);
        }
        if rl.is_key_down(KeyboardKey::KEY_DOWN) {
            game_state.move_piece(0, 1);
        }
        if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
            game_state.hard_drop();
        }
        if rl.is_key_pressed(KeyboardKey::KEY_UP) {
            game_state.rotate_piece();
        }

        if rl.is_key_pressed(KeyboardKey::KEY_P) {
            game_state.toggle_pause();
        }

        if accumulator >= update_rate {
            for event in game_state.update() {
                match event {
                    game::GameStateEvent::Continue => {},
                    game::GameStateEvent::LinesCleared(lines) => {
                        println!("Lines cleared: {}", lines);
                    },
                    game::GameStateEvent::GameOver(_) => {
                        game_over_accumulator += accumulator;
                        if game_over_accumulator >= game_over_time {
                            game_state.restart();
                            game_over_accumulator = 0.0;
                        }
                    },
                }
            }
            accumulator -= update_rate;
        }


        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        renderer::draw(&game_state, &mut d);
    }
}