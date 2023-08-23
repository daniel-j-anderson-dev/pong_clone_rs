use crate::game::moving_objects::{
    MovingObject,
    paddle::Paddle,
    Rectangle,
    WindowSide,
};

use macroquad::prelude::{
    Color,
    DVec2,
    screen_width,
    screen_height,
};

pub struct Ball {
    pub boundary: Rectangle,
    pub velocity: DVec2,
    pub color: Color,
}
impl Ball {
    pub fn new(boundary: Rectangle, velocity: DVec2, color: Color) -> Self {
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
            self.velocity = DVec2 {
                x: reflection_angle.cos() * self.velocity.length(),
                y: reflection_angle.sin() * self.velocity.length(),
            };
        }
    }
}
impl MovingObject for Ball {
    fn update_position(&mut self) -> Option<WindowSide> {
        let window_collision: Option<WindowSide> = self.keep_in_window();
        self.boundary.x += self.velocity.x;
        self.boundary.y += self.velocity.y;
        return window_collision;
    }
    fn keep_in_window(&mut self) -> Option<WindowSide> {
        let mut window_collision: Option<WindowSide> = None;

        if self.boundary.x + self.velocity.x < 0.0 {
            self.velocity.x *= -1.0;
            window_collision = Some(WindowSide::Left);
        }
        if self.boundary.x + self.boundary.w + self.velocity.x > screen_width() as f64{
            self.velocity.x *= -1.0;
            window_collision = Some(WindowSide::Right);
        }
        if self.boundary.y + self.velocity.y < 0.0 {
            self.velocity.y *= -1.0;
            window_collision = Some(WindowSide::Top);
        }
        if self.boundary.y + self.boundary.h + self.velocity.y > screen_height() as f64{
            self.velocity.y *= -1.0;
            window_collision = Some(WindowSide::Bottom);
        }

        return window_collision;
    }
    fn handle_ball_collision<MovingObj: MovingObject>(&mut self, other: &mut MovingObj) { 
        if let Some(intersection) = other.boundary().get_intersection_with(self.boundary()) {
            if other.left_side() + other.velocity().x < self.right_side() {
                other.invert_x_velocity();
            } else if other.right_side() + other.velocity().x > self.right_side() {
                other.invert_y_velocity();
            }
        }
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
}
