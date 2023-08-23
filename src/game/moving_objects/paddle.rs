use crate::game::moving_objects::{MovingObject, Rectangle, WindowSide};

use macroquad::prelude::{
    Color,
    KeyCode,
    DVec2,
    screen_width,
    screen_height,
    is_key_down,
};

const PADDLE_SPEED: f64 = 10.0;

pub struct Paddle {
    pub player_number: usize,
    pub score: u8,
    pub boundary: Rectangle,
    pub velocity: DVec2,
    pub color: Color,
}
impl Paddle {
    pub fn new(player_number: usize, boundary: Rectangle, color: Color) -> Self {
        return Self {
            player_number,
            score: 0,
            boundary,
            velocity: DVec2::ZERO,
            color,
        };
    }
    pub fn handle_input(&mut self) {
        // controls
        self.velocity = DVec2::ZERO;
        match self.player_number {
            0 => {
                if is_key_down(KeyCode::W) {
                    self.velocity.y = -1.0;
                }
                if is_key_down(KeyCode::S) {
                    self.velocity.y = 1.0;
                }
                // if is_key_down(KeyCode::A) {
                //     self.velocity.x = -1.0;
                // }
                // if is_key_down(KeyCode::D) {
                //     self.velocity.x = 1.0;
                // }
            }
            1 => {
                if is_key_down(KeyCode::Up) {
                    self.velocity.y = -1.0;
                }
                if is_key_down(KeyCode::Down) {
                    self.velocity.y = 1.0;
                }
                // if is_key_down(KeyCode::Left) {
                //     self.velocity.x = -1.0;
                // }
                // if is_key_down(KeyCode::Right) {
                //     self.velocity.x = 1.0;
                // }
            }
            _ => {/*add more players*/}
        }
        self.velocity = self.velocity.normalize_or_zero() * PADDLE_SPEED;
    }
}
impl MovingObject for Paddle {
    fn update_position(&mut self)-> Option<WindowSide> {
        let window_collision: Option<WindowSide> = self.keep_in_window();
        self.boundary.x += self.velocity.x;
        self.boundary.y += self.velocity.y;
        return window_collision;
    }
    fn keep_in_window(&mut self)-> Option<WindowSide> {
        let mut window_collision: Option<WindowSide> = None;

        if self.boundary.x + self.velocity.x < 0.0 {
            self.velocity.x = 0.0;
            self.boundary.x = 0.0;
            window_collision = Some(WindowSide::Left);
        }
        if self.boundary.x + self.boundary.w + self.velocity.x > screen_width() as f64 {
            self.velocity.x = 0.0;
            self.boundary.x = screen_width() as f64 - self.boundary.w;
            window_collision = Some(WindowSide::Right);
        }

        if self.boundary.y + self.velocity.y < 0.0 {
            self.velocity.y = 0.0;
            self.boundary.y = 0.0;
            window_collision = Some(WindowSide::Top);
        }
        if self.boundary.y + self.boundary.h + self.velocity.y > screen_height() as f64{
            self.velocity.y = 0.0;
            self.boundary.y = screen_height() as f64 - self.boundary.h;
            window_collision = Some(WindowSide::Bottom);
        }

        return window_collision;
    }
    fn boundary(&self) -> &Rectangle {
        return &self.boundary;
    }
    fn color(&self) -> &Color {
        return &self.color;
    }
    fn velocity(&self) -> &DVec2 {
        return &self.velocity;
    } 
    fn invert_x_velocity(&mut self) {
        self.velocity.x *= -1.0;
    }
    fn invert_y_velocity(&mut self) {
        self.velocity.y *= -1.0;
    }
    fn handle_ball_collision<MovingObj: MovingObject>(&mut self, ball: &mut MovingObj) {
        if let Some(intersection) = ball.boundary().get_intersection_with(self.boundary()) {
            if ball.left_side() + ball.velocity().x < self.right_side() {
                ball.invert_x_velocity();
            } else if ball.right_side() + ball.velocity().x > self.left_side() {
                ball.invert_x_velocity();
            } else if ball.top_side() + ball.velocity().y < self.bottom_side() {
                ball.invert_y_velocity();
            } else if ball.bottom_side() + ball.velocity().y > self.top_side() {
                ball.invert_y_velocity();
            }
        }
    }
}