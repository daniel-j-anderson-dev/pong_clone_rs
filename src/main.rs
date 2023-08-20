use macroquad::prelude::*;

pub mod game;
use game::GameMode;

#[macroquad::main("pong_clone")]
async fn main() {
    let mut game_state = game::State::new(GameMode::TwoPlayerLocal);
    loop {
        if is_key_down(KeyCode::Escape) {
            break;
        }
        if is_key_down(KeyCode::Backspace) {
            game_state = game::State::new(GameMode::TwoPlayerLocal);
        }

        game_state.draw();

        game_state.update();

        next_frame().await;
    }
}