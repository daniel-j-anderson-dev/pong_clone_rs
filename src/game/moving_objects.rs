use macroquad::prelude::{
    DVec2,
    Color,
    screen_width,
    screen_height,
    draw_rectangle,
};


pub mod ball;
pub mod paddle;

pub enum WindowSide {
    Top,
    Bottom,
    Left,
    Right,
}

pub trait MovingObject {
    fn update_position(&mut self) -> Option<WindowSide>;
    fn handle_ball_collision<MovingObj: MovingObject>(&mut self, other: &mut MovingObj);
    fn keep_in_window(&mut self)-> Option<WindowSide>;
    fn draw(&self) {
        draw_rectangle(self.boundary().x as f32, self.boundary().y as f32, self.boundary().w as f32, self.boundary().h as f32, *self.color())
    }
    fn boundary(&self) -> &Rectangle;
    fn velocity(&self) -> &DVec2;
    fn invert_x_velocity(&mut self);
    fn invert_y_velocity(&mut self);
    fn color(&self) -> &Color;
    fn left_side(&self) -> f64 {
        return self.boundary().left_side();
    }
    fn right_side(&self) -> f64 {
        return self.boundary().right_side();
    }
    fn top_side(&self) -> f64 {
        return self.boundary().top_side();
    }
    fn bottom_side(&self) -> f64 {
        return self.boundary().bottom_side();
    }
}

// x and y define the top left corner 
#[derive(Debug)]
pub struct Rectangle {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
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
    pub fn left_side(&self) -> f64 {
        return self.x;
    }
    pub fn right_side(&self) -> f64 {
        return self.x + self.w;
    }
    pub fn top_side(&self) -> f64 {
        return self.y;
    }
    pub fn bottom_side(&self) -> f64 {
        return self.y + self.h;
    }
    pub fn center(&self) -> DVec2 {
        let center = DVec2 {
            x: self.x + (self.w / 2.0),
            y: self.y + (self.h / 2.0)
        };
        return center;
    }
    pub fn new(x: f64, y: f64, w: f64, h: f64) -> Rectangle {
        return Rectangle { x, y, w, h };
    }
    pub fn new_top_right(w: f64, h: f64) -> Rectangle {
        return Rectangle { x: screen_width() as f64 - w, y: 0.0 , w, h };
    }
    pub fn new_top_left(w: f64, h: f64) -> Rectangle {
        return Rectangle { x: 0.0, y: 0.0, w, h };
    }
    pub fn new_centered(w: f64, h: f64) -> Rectangle {
        return Rectangle { x: screen_width() as f64 / 2.0, y: screen_height() as f64 / 2.0, w, h };
    }
    pub fn set_position(&mut self, x: f64, y: f64) {
        self.x = x;
        self.y = y;
    }
}
