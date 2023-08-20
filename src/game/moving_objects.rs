use macroquad::prelude::*;

pub mod ball;
pub mod paddle;

pub trait MovingObject {
    fn update_position(&mut self);
    fn draw(&self);
    fn keep_in_window(&mut self);
}

// x and y define the top left corner 
pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}
impl Rectangle {
    pub fn get_intersection_with(&self, other: &Rectangle) -> Option<Rectangle> {
        let (self_left, self_right)   = (self.x,  self.x  + self.w);
        let (self_top, self_bottom)   = (self.y,  self.y  + self.h);

        let (other_left, other_right) = (other.x, other.x + other.w);
        let (other_top, other_bottom) = (other.y, other.y + other.h);

        let x_overlap = (self_left <= other_right)  && (self_right  >= other_left);
        let y_overlap = (self_top  <= other_bottom) && (self_bottom >= other_top);

        if x_overlap && y_overlap {
            let (x, y) = (self_left.max(other_left), self_top.max(other_top));
            let (w, h) = (self_right.min(other_right) - x, self_bottom.min(other_bottom) - y);
            let intersection = Rectangle { x,y,w,h };
            return Some(intersection);
        } else {
            return None;
        }
    }
    pub fn center(&self) -> Vec2 {
        let center = Vec2 {
            x: self.x + (self.w / 2.0),
            y: self.y + (self.h / 2.0)
        };
        return center;
    }
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Rectangle {
        return Rectangle { x, y, w, h };
    }
    pub fn new_top_right(w: f32, h: f32) -> Rectangle {
        return Rectangle { x: screen_width() - w, y: 0.0 , w, h };
    }
    pub fn new_top_left(w: f32, h: f32) -> Rectangle {
        return Rectangle { x: 0.0, y: 0.0, w, h };
    }
    pub fn new_centered(w: f32, h: f32) -> Rectangle {
        return Rectangle { x: screen_width() / 2.0, y: screen_height() / 2.0, w, h };
    }
    pub fn set_position(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }
}
