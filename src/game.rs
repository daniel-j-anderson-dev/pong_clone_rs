pub mod moving_objects;

use self::moving_objects::{ball::Ball, paddle::Paddle, Rectangle, MovingObject};

use macroquad::prelude::*;

pub enum GameMode {
    // OnePlayerLocal,
    TwoPlayerLocal,
    // add more game modes later
}

pub struct State {
    players: Vec<Paddle>,
    balls: Vec<Ball>,
    pub background_color: Color,
    pub winner: Option<u32>,
}
impl State {
    pub fn new(game_mode: GameMode) -> State {
        let paddle_width = 20.0;
        let paddle_height = 100.0;
        let ball_size = 20.0;
        match game_mode {
            GameMode::TwoPlayerLocal => {
                let player0 = Paddle::new(
                    Rectangle::new_top_left(paddle_width, paddle_height),
                    RED
                );
                let player1 = Paddle::new(
                    Rectangle::new_top_right(paddle_width, paddle_height),
                    BLUE
                );
                let ball = Ball::new(
                    Rectangle::new_centered(ball_size, ball_size),
                    Vec2::new(1.5, 1.5),
                    WHITE
                );
                let players = vec![player0, player1];
                let balls = vec![ball];

                return State {
                    players,
                    balls,
                    background_color: BLACK,
                    winner: None,
                };
            }
        }
    }

    pub fn update(&mut self) {
        match self.winner {
            Some(player_number) => {
                // handle gameover
            }
            None => {
                for (player_number, player) in self.players.iter_mut().enumerate() {  
                    for ball in self.balls.iter_mut() {
                        ball.keep_in_window();

                        ball.handle_paddle_collision(player);
    
                        ball.update_position();
                        
                        player.handle_input(player_number);
    
                        player.keep_in_window();
                        
                        player.update_position();
                    }
                }
            }
        }
    }
    fn handle_collisions(&mut self) {
        // check for paddle collisions
        // TODO: fix me
        // for ball in self.balls.iter_mut() {
        //     for player in self.players.iter_mut() {
        //         if let Some(intersection) = ball.boundary.get_intersection_with(&player.boundary) {
        //             let ball_center = Vec2 {
        //                 x: ball.boundary.x + (ball.boundary.w / 2.0),
        //                 y: ball.boundary.y + (ball.boundary.h / 2.0)
        //             };
        //             let intersection_center = Vec2 {
        //                 x: intersection.x + (intersection.w / 2.0),
        //                 y: intersection.y + (intersection.h / 2.0)
        //             };
        //             let reflection_angle = (ball_center.x - intersection_center.x) / (intersection.w / 2.0).atan();
        //             let velocity = Vec2 {
        //                 x: reflection_angle.cos() * ball.velocity.length(),
        //                 y: reflection_angle.sin() * ball.velocity.length(),
        //             };
        //             ball.velocity = velocity;
        //         }
        //     }
        // }
    }

    pub fn draw(&self) {
        match self.winner {
            None => {
                clear_background(self.background_color);

                for player in self.players.iter() {
                    player.draw();
                }
                for ball in self.balls.iter() {
                    ball.draw();
                }
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