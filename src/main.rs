use macroquad::prelude::{
    Conf, 
    KeyCode,
    is_key_down,
    next_frame,
};

pub mod game;

fn window_configuration() -> Conf {
    return Conf {
        window_title: "Pong clone".to_owned(),
        fullscreen: false,
        window_width: 800,
        window_height: 600,
        high_dpi: true,
        sample_count: 1,
        window_resizable: true,
        // platform: ,
        // icon: ,
        ..Default::default()
    };
}

#[macroquad::main(window_configuration())]
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