use algebra::Vec2;
use algebra::Rect;

#[derive(Debug, Copy, Clone)]
pub struct Bird {
    position: Vec2,
    velocity: Vec2,
    width: f32,
    height: f32,
    gravity: f32,
    flap_force: f32,
}

impl Bird {
    pub fn new(position: Vec2) -> Bird {
        Bird {
            position: position,
            velocity: Vec2 { x: 0f32, y: 0f32 },
            width: 32_f32,
            height: 32_f32,
            gravity: 75_f32,
            flap_force: 125_f32,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.velocity.y += self.gravity * dt;
        self.position = self.position + self.velocity * dt;
    }

    pub fn flap(&mut self) {
        self.velocity.y = -self.flap_force;
    }

    pub fn get_rect(self) -> Rect {
        Rect {
            position: Vec2 {
                x: self.position.x - self.width / 2_f32,
                y: self.position.y - self.height / 2_f32,
            },
            width: self.width,
            height: self.height,
        }
    }
}
