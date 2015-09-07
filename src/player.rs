//! # The controlling player
//!
//! The player is used to process all input from the actual player, and translate it into commands
//! in the game, that a character can execute.

use graphics::RenderWindow;
use sdl2::keyboard::Keycode;
use sdl2::event::Event;
use character::Character;
use std::fmt::{Display, Error, Formatter};
use movable::Movable;

pub struct Player {
	character: Character,

	// The players input state.
	left_pressed: bool,
	right_pressed: bool,
	up_pressed: bool,
	down_pressed: bool
}

impl Player {
	pub fn new(game_window: &mut RenderWindow) -> Player {
		let character = Character::new(game_window);

		Player {
			character: character,
			left_pressed: false,
			right_pressed: false,
			up_pressed: false,
			down_pressed: false
		}
	}

	pub fn process_event(&mut self, event: &Event) {
		match event {
			&Event::KeyDown{keycode: Some(Keycode::Left), ..} => self.left_pressed = true,
			&Event::KeyUp{keycode: Some(Keycode::Left), ..} => self.left_pressed = false,

			&Event::KeyDown{keycode: Some(Keycode::Right), ..} => self.right_pressed = true,
			&Event::KeyUp{keycode: Some(Keycode::Right), ..} => self.right_pressed = false,

			&Event::KeyDown{keycode: Some(Keycode::Up), ..} => self.up_pressed = true,
			&Event::KeyUp{keycode: Some(Keycode::Up), ..} => self.up_pressed = false,

			&Event::KeyDown{keycode: Some(Keycode::Down), ..} => self.down_pressed = true,
			&Event::KeyUp{keycode: Some(Keycode::Down), ..} => self.down_pressed = false,

			_ => {}
		}
	}

	pub fn update(&mut self, game_window: &RenderWindow) {
		let mut mov_vec: (f32, f32) = (0.0, 0.0);

		if self.left_pressed {
			mov_vec.0 = -1.0;
		}
		else if self.right_pressed {
			mov_vec.0 = 1.0;
		}

		if self.up_pressed {
			mov_vec.1 = -1.0;
		}
		else if self.down_pressed {
			mov_vec.1 = 1.0;
		}

		// Normalize the vector, so that the speed is the same in every direction.
		// TODO: A vector class could do much good, especially later on!
		if mov_vec != (0.0, 0.0) {
			let length = (mov_vec.0.powi(2) + mov_vec.1.powi(2)).sqrt();

			mov_vec.0 = (mov_vec.0 / length) * 400.0 * game_window.frame_duration();
			mov_vec.1 = (mov_vec.1 / length) * 400.0 * game_window.frame_duration();
		}

		self.character.translate(mov_vec.0, mov_vec.1);
	}

	pub fn character(&self) -> &Character {
		&self.character
	}
}

impl Display for Player {
	fn fmt(&self, formatter: &mut Formatter) -> Result<(), Error> {
		formatter.write_str(format!("Left: {}, Right: {}", self.left_pressed, self.right_pressed).as_str())
	}
}
