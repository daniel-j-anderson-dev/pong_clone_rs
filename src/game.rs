pub mod moving_objects;

use self::moving_objects::{ball::Ball, paddle::Paddle, Rectangle, MovingObject};

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

fn paddle_width() -> f64 {
    return 20.0;
}

fn paddle_height() -> f64 {
    return 100.0
}

fn ball_size() -> f64 {
    return paddle_width();
}

pub struct State {
    player0: Paddle,
    player1: Paddle,
    ball: Ball,
    pub background_color: Color,
    pub winner: Option<u32>,
}
impl State {
    pub fn new() -> State {
        return State {
            player0: Paddle {
                player_number: 0,
                velocity: DVec2::ZERO,
                boundary: Rectangle::new_top_left(paddle_width(), paddle_height()),
                color: RED
            },
            player1: Paddle {
                player_number: 1,
                boundary: Rectangle::new_top_right(paddle_width(), paddle_height()),
                velocity: DVec2::ZERO,
                color: BLUE
            },
            ball: Ball {
                boundary: Rectangle::new_centered(ball_size(), ball_size()),
                velocity: DVec2::new(1.5, 1.5),
                color: WHITE
            },
            background_color: BLACK,
            winner: None,
        };
    }

    pub fn update(&mut self) {
        match self.winner {
            Some(player_number) => {
                // handle gameover
            }
            None => {
                self.player0.update_position();
                self.player1.update_position();

                self.player0.handle_input();
                self.player1.handle_input();

                if let Some(intersection) = self.ball.boundary.get_intersection_with(&self.player0.boundary) {
                    println!("ball intersected with player0, {intersection:?}");
                    self.player0.handle_collision(&mut self.ball);
                } else if let Some(intersection) = self.ball.boundary.get_intersection_with(&self.player1.boundary) {
                    println!("ball intersected with player0, {intersection:?}");
                    self.player1.handle_collision(&mut self.ball);
                }

                self.ball.update_position();
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
                let game_over_message = &format!("player {} wins", player_number + 1);
                draw_text(
                    game_over_message,
                    screen_width() / 2.0,
                    screen_height() / 2.0,
                    30.0,
                    BLACK,
                );
            }
        }
    }
}