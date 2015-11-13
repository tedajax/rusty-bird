extern crate sdl2;

use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;

#[derive(Debug, Copy, Clone)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

#[allow(dead_code)]
impl Vec2 {
    pub fn new(x: f32, y: f32) -> Vec2 {
        Vec2 { x: x, y: y }
    }

    pub fn magnitude_sqr(self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    pub fn magnitude(self) -> f32 {
        self.magnitude_sqr().sqrt()
    }

    pub fn normalized(self) -> Vec2 {
        let l = self.magnitude();
        Vec2 {
            x: self.x / l,
            y: self.y / l,
        }
    }
}

impl Add<Vec2> for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub<Vec2> for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul<f32> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: f32) -> Vec2 {
        Vec2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Div<f32> for Vec2 {
    type Output = Vec2;

    fn div(self, rhs: f32) -> Vec2 {
        Vec2 {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Rect {
    pub position: Vec2,
    pub width: f32,
    pub height: f32,
}

#[allow(dead_code)]
impl Rect {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Rect {
        Rect {
            position: Vec2 { x: x, y: y },
            width: w,
            height: h,
        }
    }

    pub fn new_center(center: Vec2, w: f32, h: f32) -> Rect {
        let hw = w / 2f32;
        let hh = h / 2f32;
        Rect {
            position: Vec2 {
                x: center.x - hw,
                y: center.y - hh,
            },
            width: w,
            height: h,
        }
    }

    pub fn left(self) -> f32 { self.position.x }
    pub fn right(self) -> f32 { self.position.x + self.width }
    pub fn top(self) -> f32 { self.position.y }
    pub fn bottom(self) -> f32 { self.position.y + self.height }

    pub fn intersects(self, other: &Rect) -> bool {
        self.left() <= other.right() &&
        self.right() >= other.left() &&
        self.top() <= other.bottom() &&
        self.bottom() >= other.top()
    }
}

impl Add<Vec2> for Rect {
    type Output = Rect;

    fn add(self, rhs: Vec2) -> Rect {
        Rect {
            position: self.position + rhs,
            width: self.width,
            height: self.height,
        }
    }
}

impl Sub<Vec2> for Rect {
    type Output = Rect;

    fn sub(self, rhs: Vec2) -> Rect {
        Rect {
            position: self.position - rhs,
            width: self.width,
            height: self.height,
        }
    }
}

impl Into<sdl2::rect::Rect> for Rect {
    fn into(self) -> sdl2::rect::Rect {
        let r = sdl2::rect::Rect::new(
            self.position.x as i32,
            self.position.y as i32,
            self.width as u32,
            self.height as u32,
        ).unwrap();

        match r {
            Some(rect) => { rect },
            _ => { panic!("How did you even fail this is stupid that I have to support this."); }
        }
    }
}

impl From<sdl2::rect::Rect> for Rect {
    fn from(other: sdl2::rect::Rect) -> Self {
        Rect {
            position: Vec2 {
                x: other.x() as f32,
                y: other.y() as f32,
            },
            width: other.width() as f32,
            height: other.height() as f32,
        }
    }
}
