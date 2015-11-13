extern crate sdl2;

mod algebra;
mod bird;
mod pipe;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::Renderer;
use sdl2::pixels::Color;

use algebra::Vec2;
use algebra::Rect;

use bird::Bird;
use pipe::Pipe;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();

    let window = video.window("Rusty Bird", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut renderer = window.renderer().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut bird = Bird::new(Vec2::new(64_f32, 400_f32));
    let mut camera = Vec2::new(0_f32, 0_f32);

    const PIPE_COUNT: usize = 5;
    const PIPE_BASE_POSITION: f32 = 800f32;
    const PIPE_SPACING: f32 = 400f32;

    let mut pipes: Vec<Pipe> = vec![];

    for i in 0..PIPE_COUNT {
        pipes.push(Pipe::new(Vec2::new(PIPE_BASE_POSITION + PIPE_SPACING * (i as f32), 300f32)));
    }

    let mut next_pipe_location = PIPE_BASE_POSITION + PIPE_SPACING * (PIPE_COUNT as f32);

    let mut score: i32 = 0;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::Z), .. } => {
                    bird.flap();
                }
                _ => {}
            }
        }

        bird.update(0.016_f32);

        renderer.set_draw_color(Color::RGB(0, 0, 0));
        renderer.clear();

        camera.x = bird.get_position().x - 64f32;

        for mut pipe in pipes.iter_mut() {
            let pipe_center = pipe.get_center();
            if pipe_center.x < camera.x - pipe.get_width() / 2f32 {
                pipe.set_center(Vec2::new(next_pipe_location, 300f32));
                next_pipe_location += PIPE_SPACING;
            }

            if pipe.try_pass(bird.get_position()) {
                score += 1;
                println!("Score! {}", score);
            }
        }

        render_rect(&mut renderer, &camera, bird.get_rect(), Color::RGB(255, 0, 0));

        for pipe in pipes.iter() {
            render_pipe(&mut renderer, &camera, &pipe, Color::RGB(0, 255, 0));
        }

        renderer.present();
    }
}

fn render_rect(renderer: &mut Renderer, camera: &Vec2, rect: Rect, color: Color) {
    let r = rect - *camera;
    let sdl_rect = r.into();
    renderer.set_draw_color(color);
    renderer.fill_rect(sdl_rect);
}

fn render_pipe(renderer: &mut Renderer, camera: &Vec2, pipe: &Pipe, color: Color) {
    render_rect(renderer, camera, pipe.get_top_rect(), color);
    render_rect(renderer, camera, pipe.get_bottom_rect(), color);
}
