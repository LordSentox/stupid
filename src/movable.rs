//! # Trait for anything moving.
//!
//! Mostly the trait movable applies to anything that can be moved about on the screen, but
//! theoretically it can be used in more abstract ways, too.

pub trait Movable {
	/// # Move the element
	///
	/// Move the element relative to its current position.
	fn translate(&mut self, x: f32, Y: f32);

	/// # Set the position
	///
	/// Set the absolute position of the element. This can also be viewed as a move relative to the
	/// (0, 0) starting vector.
	fn set_pos(&mut self, pos: (f32, f32));

	/// Get the x position.
	fn x(&self) -> f32;

	/// Get the y position.
	fn y(&self) -> f32;

	/// Get the position vector (x, y).
	fn pos(&self) -> (f32, f32);
}
