// TODO: This should be removed when the code is not in the first implementation phase any longer,
// and must be removed before production, to avoid overhead.
#![allow(dead_code)]

// This is necessary for handy String-conversions, which are currently awaiting RFC revision.
#![feature(convert)]

#![feature(rc_unique)]

extern crate sdl2;
extern crate sdl2_image;

mod static_object;
mod graphics;

use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::keyboard::Keycode;
use sdl2::event::Event;

// fn draw_grid(buffer: &mut [u8], pitch: usize, width: u32, height: u32) {
//     for y in (0..height as usize) {
//         for x in (0..width as usize) {
//             let offset = y*pitch + x*3;
//
//             if (y % 32 == 0) | (x % 32 == 0) {
//                 buffer[offset + 0] = 20;
//                 buffer[offset + 1] = 20;
//                 buffer[offset + 2] = 20;
//             }
//             else {
//                 buffer[offset + 0] = 240;
//                 buffer[offset + 1] = 240;
//                 buffer[offset + 2] = 240;
//             }
//         }
//     }
// }
//
// pub fn main() {
//     let sdl_context = sdl2::init().unwrap();
//     let video_subsystem = sdl_context.video().unwrap();
//
//     let window = video_subsystem.window("Powerd by Stupid Engine", 832, 640)
//         .position_centered()
//         .opengl()
//         .build()
//         .unwrap();
//
//     let mut renderer = window.renderer().build().unwrap();
//
//     let mut texture = renderer.create_texture_streaming(PixelFormatEnum::RGB24, (832, 640)).unwrap();
//     // create a grid, with fields the size 64x64
//     texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
//         let (width, height) = renderer.output_size().unwrap();
//         draw_grid(buffer, pitch, width, height);
//     }).unwrap();
//
//     renderer.clear();
//     renderer.copy(&texture, None, Some(Rect::new_unwrap(0, 0, 832, 640)));
//     renderer.present();
//
//     let mut running = true;
//     let mut event_pump = sdl_context.event_pump().unwrap();
//
//     while running {
//         for event in event_pump.poll_iter() {
//             use sdl2::event::Event;
//
//             match event {
//                 Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
//                     running = false
//                 },
//                 Event::Window {win_event_id: event_id, ..} => {
//                     if event_id == sdl2::event::WindowEventId::Resized {
//
//                         let (width, height) = renderer.output_size().unwrap();
//                         let mut texture = renderer.create_texture_streaming(PixelFormatEnum::RGB24, (width, height)).unwrap();
//                         texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
//                             draw_grid(buffer, pitch, width, height);
//                         }).unwrap();
//
//                         renderer.clear();
//                         renderer.copy(&texture, None, Some(Rect::new_unwrap(0, 0, width, height)));
//                         renderer.present();
//                     }
//                 },
//                 _ => {}
//             }
//         }
//     }
// }

fn main() {
    let sdl_context = sdl2::init().unwrap();
    sdl2_image::init(sdl2_image::INIT_PNG);
    let mut window = graphics::RenderWindow::new(&sdl_context, "Stupid is awesome", 800, 600);

    let test_sprite = window.create_sprite("test_sprite.png", (0.0, 0.0), None).unwrap();

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
