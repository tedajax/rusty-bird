extern crate sdl2;

mod algebra;
mod bird;
mod pipe;
mod gametime;
mod game;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

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

    let mut gametime = gametime::GameTime::new();
    let mut game = game::Game::new();
    game.init();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(keycode), ..} => {
                    game.on_key_down(keycode);
                },
                Event::KeyUp { keycode: Some(keycode), ..} => {
                    game.on_key_up(keycode);
                }
                _ => {}
            }
        }

        gametime.update();

        game.update(gametime);

        renderer.set_draw_color(Color::RGB(0, 0, 0));
        renderer.clear();

        game.render(&mut renderer);

        renderer.present();
    }
}
