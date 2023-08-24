pub mod moving_objects;

use self::moving_objects::{ball::Ball, paddle::Paddle, Rectangle, MovingObject, WindowSide};

use macroquad::{
    prelude::{
        DVec2,
        clear_background,
        draw_text,
        screen_width,
        screen_height,
    },
    color::{
        Color,
        colors::*
    },
};

pub struct State {
    player0: Paddle,
    player1: Paddle,
    ball: Ball,
    pub background_color: Color,
    pub winner: Option<usize>,
}
impl State {
    pub fn new() -> State {
        return State {
            player0: Paddle {
                player_number: 0,
                score: 0,
                velocity: DVec2::ZERO,
                boundary: Rectangle::new_top_left(paddle_width(), paddle_height()),
                color: RED
            },
            player1: Paddle {
                player_number: 1,
                score: 0,
                boundary: Rectangle::new_top_right(paddle_width(), paddle_height()),
                velocity: DVec2::ZERO,
                color: BLUE
            },
            ball: Ball {
                boundary: Rectangle::new_centered(ball_size(), ball_size()),
                velocity: DVec2::new(-3.0, 3.0),
                color: WHITE
            },
            background_color: BLACK,
            winner: None,
        };
    }

    pub fn update(&mut self) {
        if self.winner.is_none() {
            let _ = self.player0.update_position();
            let _ = self.player1.update_position();

            self.player0.handle_input();
            self.player1.handle_input();

            self.player0.handle_ball_collision(&mut self.ball);
            self.player1.handle_ball_collision(&mut self.ball);

            if let Some(window_collision) = self.ball.update_position() {
                match window_collision {
                    WindowSide::Right => self.player0.score += 1,
                    WindowSide::Left => self.player1.score += 1,
                    _ => {}
                }
            }

            if self.player0.score >= 5 {
                self.winner = Some(self.player0.player_number);
            } else if self.player1.score >= 5 {
                self.winner = Some(self.player1.player_number);
            }
        }
    }

    pub fn draw(&self) {
        match self.winner {
            None => {
                clear_background(self.background_color);
                self.player0.draw();
                self.player1.draw();
                self.ball.draw();
            }
            Some(player_number)=> {
                clear_background(WHITE);
                let game_over_message = &format!("player {} wins\nPress escape to quit\nPress backspace to restart", player_number + 1);
                draw_text(
                    game_over_message,
                    screen_width() / 2.5,
                    screen_height() / 2.0,
                    30.0,
                    BLACK,
                );
            }
        }
    }
}

fn paddle_width() -> f64 {
    return 20.0;
}

fn paddle_height() -> f64 {
    return 100.0
}

fn ball_size() -> f64 {
    return paddle_width();
}