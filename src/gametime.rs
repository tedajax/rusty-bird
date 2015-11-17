extern crate time;

#[derive(Debug, Clone, Copy)]
pub struct GameTime {
    prev_frame_time: time::Timespec,
    curr_frame_time: time::Timespec,
}

#[allow(dead_code)]
impl GameTime {
    pub fn new() -> GameTime {
        let now = time::get_time();
        GameTime {
            prev_frame_time: now,
            curr_frame_time: now,
        }
    }

    pub fn reset(&mut self) {
        let now = time::get_time();
        self.prev_frame_time = now;
        self.curr_frame_time = now;
    }

    pub fn update(&mut self) {
        self.prev_frame_time = self.curr_frame_time;
        self.curr_frame_time = time::get_time();
    }

    pub fn delta(self) -> time::Duration {
        self.curr_frame_time - self.prev_frame_time
    }

    pub fn dt(self) -> f32 {
        (self.delta().num_nanoseconds().unwrap() as f32) / 1_000_000_000_f32
    }
}