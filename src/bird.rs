use algebra::Vec2;
use algebra::Rect;

#[derive(Debug, Copy, Clone)]
pub enum BirdState {
    Idle,       // Menu style
    Flying,     // In play
    Dead,       // Falling down
}

#[derive(Debug, Copy, Clone)]
struct BirdConfig {
    start_position: Vec2,
    speed: f32,
    gravity: f32,
    flap_force: f32,
}

#[derive(Debug, Copy, Clone)]
pub struct Bird {
    position: Vec2,
    velocity: Vec2,
    width: f32,
    height: f32,
    state: BirdState,
    config: BirdConfig,
}

impl Bird {
    pub fn new() -> Bird {
        Bird {
            position: Vec2::new(64_f32, 300_f32),
            velocity: Vec2::new(0_f32, 0_f32),
            width: 32_f32,
            height: 32_f32,
            state: BirdState::Flying,
            config: BirdConfig {
                start_position: Vec2::new(64_f32, 300_f32),
                speed: 200_f32,
                gravity: 2400_f32,
                flap_force: 700_f32,
            },
        }
    }

    pub fn update(&mut self, dt: f32) {
        match self.state {
            BirdState::Idle => { },
            BirdState::Flying | BirdState::Dead => {
                self.velocity.y += self.config.gravity * dt;
            },
        }

        self.position = self.position + self.velocity * dt;
    }

    pub fn set_state(&mut self, state: BirdState) {
        match state {
            BirdState::Idle => {
                self.position = self.config.start_position;
                self.velocity.x = 0_f32;
                self.velocity.y = 0_f32;
            },
            BirdState::Flying => {
                self.position = self.config.start_position;
                self.velocity.x = self.config.speed;
                self.velocity.y = 0_f32;
            },
            BirdState::Dead => {
                self.velocity.x = 0_f32;
                self.velocity.y = -1200_f32;
            },
        }

        self.state = state;
    }

    fn force_flap(&mut self) {
        self.velocity.y = -self.config.flap_force;
    }

    pub fn flap(&mut self) {
        match self.state {
            BirdState::Flying => {
                self.force_flap();
            },
            _ => {},
        }
    }

    pub fn get_position(self) -> Vec2 {
        self.position
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
