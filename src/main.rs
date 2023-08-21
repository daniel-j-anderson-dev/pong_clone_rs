use macroquad::prelude::{
    KeyCode,
    is_key_down,
    next_frame,
};

pub mod game;

#[macroquad::main("pong_clone")]
async fn main() {
    let mut game_state = game::State::new();
    loop {
        if is_key_down(KeyCode::Escape) {
            break;
        }
        if is_key_down(KeyCode::Backspace) {
            game_state = game::State::new();
        }

        game_state.draw();

        game_state.update();

        next_frame().await;
    }
}