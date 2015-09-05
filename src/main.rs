// TODO: This should be removed when the code is not in the first implementation phase any longer,
// and must be removed before production, to avoid overhead.
#![allow(dead_code)]

// This is necessary for handy String-conversions, which are currently awaiting RFC revision.
#![feature(convert)]

extern crate sdl2;
extern crate sdl2_image;

mod static_object;
mod graphics;

use sdl2::event::Event;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    sdl2_image::init(sdl2_image::INIT_PNG);
    let mut window = graphics::RenderWindow::new(&sdl_context, "Stupid is awesome", 800, 600);

    let test_sprite = window.create_sprite("test_sprite.bmp", (0.0, 0.0), None).unwrap();

    let mut running = true;
    let mut event_pump = sdl_context.event_pump().unwrap();

    while running {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => running = false,
                _ => {}
            }
        }

        window.clear();
        window.draw(&test_sprite);
        window.present();
    }

    sdl2_image::quit();
}
