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

pub struct State {
    player0: Paddle,
    player1: Paddle,
    ball: Ball,
    pub background_color: Color,
    pub winner: Option<u32>,
}
impl State {
    pub fn new() -> State {
        let paddle_width = 20.0;
        let paddle_height = 100.0;
        let ball_size = 20.0;
        let player0 = Paddle::new(
            0,
            Rectangle::new_top_left(paddle_width, paddle_height),
            RED
        );
        let player1 = Paddle::new(
            1,
            Rectangle::new_top_right(paddle_width, paddle_height),
            BLUE
        );
        let ball = Ball::new(
            Rectangle::new_centered(ball_size, ball_size),
            DVec2::new(1.5, 1.5),
            WHITE
        );

        return State {
            player0,
            player1,
            ball,
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