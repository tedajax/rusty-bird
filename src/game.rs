use sdl2::render::Renderer;
use sdl2::pixels::Color;
use sdl2::keyboard::Keycode;

use gametime::GameTime;
use bird::{ Bird, BirdState };
use pipe::Pipe;

use algebra::Vec2;
use algebra::Rect;

#[derive(Debug, Clone, Copy)]
struct GameConfig {
    pub pipe_count: usize,
    pub pipe_base_position: f32,
    pub pipe_spacing: f32,
}


#[derive(Debug)]
pub struct Game {
    bird: Bird,
    pipes: Vec<Pipe>,
    camera: Vec2,
    next_pipe_location: f32,
    score: i32,
    config: GameConfig,
}

impl Game {
    pub fn new() -> Game {
        Game {
            bird: Bird::new(),
            pipes: vec![],
            camera: Vec2::zero(),
            next_pipe_location: 0_f32,
            score: 0,
            config: GameConfig {
                pipe_count: 5,
                pipe_base_position: 1000f32,
                pipe_spacing: 400f32
            }
        }
    }

    pub fn init(&mut self) {
        for i in 0..self.config.pipe_count {
            let x = self.config.pipe_base_position + self.config.pipe_spacing * (i as f32);
            self.pipes.push(Pipe::new(Vec2::new(x, 300f32)));
        }

        self.bird.set_state(BirdState::Flying);
    }

    pub fn on_key_down(&mut self, keycode: Keycode) {
        match keycode {
            Keycode::Z => { self.bird.flap(); },
            _ => {}
        }
    }

    pub fn on_key_up(&mut self, keycode: Keycode) {
        match keycode {
            _ => {}
        }
    }

    pub fn update(&mut self, gametime: GameTime) {
        self.bird.update(gametime.dt());

        self.camera.x = self.bird.get_position().x - 64f32;

        for mut pipe in self.pipes.iter_mut() {
            let pipe_center = pipe.get_center();
            if pipe_center.x < self.camera.x - pipe.get_width() / 2f32 {
                pipe.set_center(Vec2::new(self.next_pipe_location, 300f32));
                self.next_pipe_location += self.config.pipe_spacing;
            }

            if pipe.try_pass(self.bird.get_position()) {
                self.score += 1;
                println!("Score! {}", self.score);
            }
        }

        {
            let bird_rect = self.bird.get_rect();

            for pipe in self.pipes.iter() {
                if bird_rect.intersects(&pipe.get_top_rect()) ||
                   bird_rect.intersects(&pipe.get_bottom_rect()) {
                    self.bird.set_state(BirdState::Dead);
                }
            }
        }
    }

    pub fn render(&self, renderer: &mut Renderer) {
        for pipe in self.pipes.iter() {
            Game::render_pipe(renderer, &self.camera, &pipe, Color::RGB(0, 255, 0));
        }

        {
            let top_rect = Rect::new(self.camera.x, 0f32, 800f32, 32f32);
            let bottom_rect = Rect::new(self.camera.x, 600f32 - 32f32, 800f32, 32f32);
            Game::render_rect(renderer, &self.camera, top_rect, Color::RGB(196, 118, 0));
            Game::render_rect(renderer, &self.camera, bottom_rect, Color::RGB(196, 118, 0));
        }

        Game::render_rect(renderer, &self.camera, self.bird.get_rect(), Color::RGB(255, 0, 0));
    }

    fn render_rect(renderer: &mut Renderer, camera: &Vec2, rect: Rect, color: Color) {
        let r = rect - *camera;
        let sdl_rect = r.into();
        renderer.set_draw_color(color);
        renderer.fill_rect(sdl_rect);
    }

    fn render_pipe(renderer: &mut Renderer, camera: &Vec2, pipe: &Pipe, color: Color) {
        Game::render_rect(renderer, camera, pipe.get_top_rect(), color);
        Game::render_rect(renderer, camera, pipe.get_bottom_rect(), color);
    }
}