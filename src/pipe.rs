use algebra::Vec2;
use algebra::Rect;

#[derive(Debug, Copy, Clone)]
pub struct Pipe {
    center: Vec2,
    half_gap: f32,
    width: f32,
    is_passed: bool,
}

impl Pipe {
    pub fn new(center: Vec2) -> Pipe {
        Pipe {
            center: center,
            half_gap: 108_f32,
            width: 64_f32,
            is_passed: false,
        }
    }

    pub fn get_center(self) -> Vec2 { self.center }
    pub fn get_width(self) -> f32 { self.width }

    pub fn set_center(&mut self, center: Vec2) {
        self.center = center;
        self.is_passed = false;
    }

    pub fn try_pass(&mut self, position: Vec2) -> bool {
        if self.is_passed {
            return false;
        }

        if position.x >= self.center.x {
            self.is_passed = true;
            return true;
        }

        return false;
    }

    pub fn get_top_rect(self) -> Rect {
        Rect {
            position: Vec2 {
                x: self.center.x - self.width / 2f32,
                y: 0f32
            },
            width: self.width,
            height: self.center.y - self.half_gap,
        }
    }

    pub fn get_bottom_rect(self) -> Rect {
        Rect {
            position: Vec2 {
                x: self.center.x - self.width / 2f32,
                y: self.center.y + self.half_gap,
            },
            width: self.width,
            height: 600_f32 - self.center.y + self.half_gap,
        }
    }
}