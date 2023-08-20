use macroquad::prelude::*;

use crate::game::moving_objects::*;

const PADDLE_SPEED: f32 = 10.0;

pub struct Paddle {
    pub boundary: Rectangle,
    pub velocity: Vec2,
    pub color: Color,
}
impl Paddle {
    pub fn new(boundary: Rectangle, color: Color) -> Self {
        return Self {
            boundary,
            velocity: Vec2::ZERO,
            color,
        };
    }
    pub fn handle_input(&mut self, player_number: usize) {
        // controls
        self.velocity = Vec2::ZERO;
        match player_number {
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
    fn update_position(&mut self) {
        self.boundary.x += self.velocity.x;
        self.boundary.y += self.velocity.y;
    }
    fn draw(&self) {
        draw_rectangle(self.boundary.x, self.boundary.y, self.boundary.w, self.boundary.h, self.color);
    }
    fn keep_in_window(&mut self) {
        if self.boundary.y < 0.0 {
            self.boundary.y = 0.0;
            self.velocity.y = 0.0;
        }
        if self.boundary.y + self.boundary.h > screen_height() {
            self.boundary.y = screen_height() - self.boundary.h;
            self.velocity.y = 0.0;
        }
        if self.boundary.x < 0.0 {
            self.boundary.x = 0.0;
            self.velocity.x = 0.0;
        }
        if self.boundary.x + self.boundary.w > screen_width() {
            self.boundary.x = screen_width() - self.boundary.w;
            self.velocity.x = 0.0;
        }
    }
}