/*
 * Represents an object that is a solid part of the map, hence not moving, eventhough it may be
 * animated.
 */

extern crate sdl2;
use self::sdl2::render::{Texture, Renderer};
use self::sdl2::rect::Rect;

pub struct StaticObject<'a> {
	texture: &'a Texture,
	// The position is defined by the textures bottommost, leftmost point, since that point is
	// always "on the ground" or at least nearest to the ground.
	x: i32,
	y: i32,
	width: u16,
	height: u16
}

impl<'a> StaticObject<'a> {
	pub fn new(texture: &'a Texture, x: i32, y: i32, width: u16, height: u16) -> StaticObject {
		StaticObject {
			texture: texture,
			x: x,
			y: y,
			width: width,
			height: height
		}
	}

	// TODO: Move this to a seperate trait, like Drawable in SFML.
	pub fn draw(&self, renderer: &'a mut Renderer) {
		// TODO: The textures should be saved in some sort of database, since this is way too
		// error-prone in many different stages.
		renderer.copy(self.texture, None, Some(Rect::new_unwrap(self.x, self.y, self.width as u32, self.height as u32)));
	}
}
