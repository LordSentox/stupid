extern crate sdl2;
use sdl2::render::{Renderer, Texture};
use graphics::Drawable;

pub struct Sprite<'a> {
	texture: &'a Texture
}

impl<'a> Drawable for Sprite<'a> {
	fn draw(&self, renderer: &mut Renderer) {
		// Yeah baby, this is going to be awesome!
		// TODO: Implement that Animation class you thought about.
	}
}
