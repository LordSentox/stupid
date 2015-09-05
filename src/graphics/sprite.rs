extern crate sdl2;
use sdl2::render::{Renderer, Texture};
use sdl2::rect::Rect;
use graphics::{Drawable};
use graphics::texture_manager::TextureEntry;
use std::rc::Rc;

pub struct Sprite {
	texture: Rc<Texture>,
	width: u32,
	height: u32,
	x: f32,
	y: f32,
	angle: f64
}

impl Sprite {
	/// # Create a new sprite
	///
	/// It takes the name of the texture as an argument. In case None is provided as the size, the
	/// texture size is taken as is, otherwise it will be stretched to fit inside the rectangle.
	pub fn new(texture_entry: &TextureEntry, pos: (f32, f32), size: Option<(u32, u32)>) -> Sprite {
		match size {
			Some(size) => {
				// Take the provided size and ignore the size of the TextureEntry.
				Sprite {
					texture: texture_entry.texture(),
					width: size.0,
					height: size.1,
					x: pos.0,
					y: pos.1,
					angle: 0.0
				}
			},
			None => {
				// Take the original size.
				Sprite {
					texture: texture_entry.texture(),
					width: texture_entry.width(),
					height: texture_entry.height(),
					x: pos.0,
					y: pos.1,
					angle: 0.0
				}
			}
		}
	}

	/// # Move the sprite
	///
	/// Move the sprite by the provided vector. While it can only be rendered by whole pixels, it
	/// can virtually be moved seamlessly.
	pub fn translate(&mut self, x: f32, y: f32) {
		// May I just say, that having move as a keyword really sucks at this moment?
		self.x += x;
		self.y += y;
	}

	/// Get the x position.
	pub fn x(&self) -> f32 {
		self.x
	}

	/// Get the y position.
	pub fn y(&self) -> f32 {
		self.y
	}

	/// Get the position vector (x, y).
	pub fn pos(&self) -> (f32, f32) {
		(self.x, self.y)
	}

	/// # Set the position
	///
	/// Set the position of the sprite by providing a completely new position vector (x, y).
	pub fn set_pos(&mut self, pos: (f32, f32)) {
		self.x = pos.0;
		self.y = pos.1;
	}

	/// # Resize the sprite
	///
	/// This function resizes the sprite relatively to the left corner. If the sprite is supposed
	/// to be resized without changing perceived position, use resize_center() instead.
	pub fn resize(&mut self, size: (u32, u32)) {
		self.width = size.0;
		self.height = size.1;
	}

	/// # Resize the sprite
	///
	/// This function resizes the sprite relatively to the center. If the sprite is supposed to be
	/// resized without changing its absolute position, use resize() instead.
	pub fn resize_center(&mut self, size: (u32, u32)) {
		// The adjusted position for the new size.
		self.x += (self.width  - size.0) as f32 / 2.0;
		self.y += (self.height - size.1) as f32 / 2.0;

		self.width = size.0;
		self.height = size.1;
	}

	/// # Set the rotation of the sprite
	///
	/// Set the sprites rotation around its center. If you want to rotate it relatively, use the
	/// rotate() function. The value is expected in degrees.
	pub fn set_rotation(&mut self, angle: f64) {
		self.angle = angle;
	}

	/// # Rotate the sprite
	///
	/// Rotate the sprite around its center. If you want to set the rotation absolutely, use the
	/// set_rotation() function. The value is expected in degrees.
	pub fn rotate(&mut self, angle: f64) {
		self.angle += angle;
	}

	/// # Get the rotation
	///
	/// This returns the current rotation of the sprite in degrees.
	pub fn rotation(&self) -> f64 {
		self.angle
	}
}

impl Drawable for Sprite {
	fn draw(&self, renderer: &mut Renderer) {
		renderer.copy_ex(&self.texture, None, Some(Rect::new_unwrap(self.x as i32, self.y as i32, self.width, self.height)), self.angle, None, (false, false));
	}
}
