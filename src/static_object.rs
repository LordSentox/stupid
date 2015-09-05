/*
 * Represents an object that is a solid part of the map, hence not moving, eventhough it may be
 * animated.
 */

extern crate sdl2;
use self::sdl2::render::{Texture, Renderer};
use self::sdl2::rect::Rect;
use std::rc::Rc;

use graphics::TextureManager;

pub struct StaticObject {
	texture: Rc<Texture>,
	// The position is defined by the textures bottommost, leftmost point, since that point is
	// always "on the ground" or at least nearest to the ground.
	x: i32,
	y: i32,
	width: u32,
	height: u32
}

impl StaticObject {
	pub fn new(texture_manager: &mut TextureManager, texture_name: &str, x: i32, y: i32, width: Option<u32>, height: Option<u32>) -> Result<StaticObject, String> {
		let texture_entry = match texture_manager.load_texture(texture_name) {
			Some(texture_entry) => texture_entry,
			None => return Err(String::from("Could not load texture."))
		};

		let width = match width {
			Some(custom_width) => custom_width,
			None => texture_entry.width()
		};

		let height = match height {
			Some(custom_height) => custom_height,
			None => texture_entry.height()
		};

		Ok(StaticObject {
			texture: texture_entry.texture(),
			x: x,
			y: y,
			width: width,
			height: height
		})
	}

	// TODO: Move this to a seperate trait, like Drawable in SFML.
	pub fn draw(&self, renderer: &mut Renderer) {
		// TODO: The textures should be saved in some sort of database, since this is way too
		// error-prone in many different stages.
		renderer.copy(&self.texture, None, Some(Rect::new_unwrap(self.x, self.y, self.width, self.height)));
	}
}
