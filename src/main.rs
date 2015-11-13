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

    let pipe = Pipe::new(Vec2::new(300_f32, 300_f32));

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

        render_rect(&mut renderer, bird.get_rect(), Color::RGB(255, 0, 0));
        render_rect(&mut renderer, pipe.get_top_rect(), Color::RGB(0, 255, 0));
        render_rect(&mut renderer, pipe.get_bottom_rect(), Color::RGB(0, 255, 0));

        renderer.present();
    }
}

fn render_rect(renderer: &mut Renderer, rect: Rect, color: Color) {
    let sdl_rect = sdl2::rect::Rect::new(rect.position.x as i32,
        rect.position.y as i32,
        rect.width as u32,
        rect.height as u32).unwrap();

    match sdl_rect {
        Some(r) => {
            renderer.set_draw_color(color);
            renderer.fill_rect(r);            
        },
        _ => {}
    }

}
