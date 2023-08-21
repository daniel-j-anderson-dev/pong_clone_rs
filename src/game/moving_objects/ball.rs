use crate::game::moving_objects::{MovingObject, paddle::Paddle, Rectangle};

use macroquad::prelude::{
    Color,
    Vec2,
    screen_width,
    screen_height,
    draw_rectangle,
};

pub struct Ball {
    pub boundary: Rectangle,
    pub velocity: Vec2,
    pub color: Color,
}
impl Ball {
    pub fn new(boundary: Rectangle, velocity: Vec2, color: Color) -> Self {
        return Self {
            boundary,
            velocity,
            color,
        };
    }
    pub fn handle_paddle_collision(&mut self, paddle: &Paddle) {
        if let Some(intersection) = self.boundary.get_intersection_with(&paddle.boundary) {
            let reflection_angle =
                (self.boundary.center().x - intersection.center().x) / (intersection.w / 2.0).atan();
            self.velocity = Vec2 {
                x: reflection_angle.cos() * self.velocity.length(),
                y: reflection_angle.sin() * self.velocity.length(),
            };
        }
    }
}
impl MovingObject for Ball {
    fn update_position(&mut self) {
        self.boundary.x += self.velocity.x;
        self.boundary.y += self.velocity.y;
    }
    fn draw(&self) {
        draw_rectangle(self.boundary.x, self.boundary.y, self.boundary.w, self.boundary.h, self.color);
    }
    fn keep_in_window(&mut self) {
        if self.boundary.x < 0.0 {
            self.velocity.x *= -1.0;
            // self.winner = Some(1);
        }
        if self.boundary.x + self.boundary.w > screen_width() {
            // self.winner = Some(0)
            self.velocity.x *= -1.0;
        }
        if self.boundary.y < 0.0 {
            self.velocity.y *= -1.0;
        }
        if self.boundary.y + self.boundary.h > screen_height() {
            self.velocity.y *= -1.0;
        }
    }
    fn handle_collision(&mut self, other: Box<dyn MovingObject>) {
        
    }
}
