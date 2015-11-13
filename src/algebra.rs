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