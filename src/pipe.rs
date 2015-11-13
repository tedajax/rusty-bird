use algebra::Vec2;
use algebra::Rect;

#[derive(Debug, Copy, Clone)]
pub struct Pipe {
    center_x: f32,
    width: f32,
    top: f32,
    bottom: f32,
}

impl Pipe {
    pub fn new(center_x: f32, center_y: f32) -> Pipe {
        let half_gap = 216f32 / 2f32;
        Pipe {
            center_x: center_x,
            width: 64_f32,
            top: center_y - half_gap,
            bottom: center_y + half_gap,
        }
    }

    pub fn get_top_rect(self) -> Rect {
        Rect {
            position: Vec2 {
                x: self.center_x - self.width / 2f32,
                y: 0f32
            },
            width: self.width,
            height: self.top,
        }
    }

    pub fn get_bottom_rect(self) -> Rect {
        Rect {
            position: Vec2 {
                x: self.center_x - self.width / 2f32,
                y: self.bottom,
            },
            width: self.width,
            height: 600_f32 - self.bottom,
        }
    }
}