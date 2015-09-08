// TODO: This should be removed when the code is not in the first implementation phase any longer,
// and must be removed before production, to avoid overhead.
#![allow(dead_code)]

// This is necessary for handy String-conversions, which are currently awaiting RFC revision.
#![feature(convert)]

extern crate sdl2;
extern crate time;

mod character;
mod entity;
mod graphics;
mod player;
use player::Player;
mod static_object;
mod sys;

use sdl2::event::Event;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let mut window = graphics::RenderWindow::new(&sdl_context, "Stupid is awesome", 800, 600);

    let mut running = true;

    let mut player = Player::new(&mut window);

    while running {
        for event in window.poll_events() {
            player.process_event(&event);

            match event {
                Event::Quit {..} => running = false,
                _ => {}
            }
        }

        player.update(&window);

        window.clear();
        window.draw(player.character());
        window.present();
    }
}
