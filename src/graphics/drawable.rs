/*
 * A simple trait that is implemented by structures that are drawn to the screen.
 */

extern crate sdl2;
use sdl2::render::Renderer;

pub trait Drawable {
	fn draw(&self, renderer: &mut Renderer);
}
