//! # The controlling player
//!
//! The player is used to process all input from the actual player, and translate it into commands
//! in the game, that a character can execute.

use graphics::RenderWindow;
use sdl2::keyboard::Keycode;
use sdl2::event::Event;
use character::Character;
use std::fmt::{Display, Error, Formatter};
use sys::{Movable, Vector};

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
		let mut mov_vec: Vector<f32> = Vector::new(0.0, 0.0);

		if self.left_pressed {
			mov_vec.x = -1.0;
		}
		else if self.right_pressed {
			mov_vec.x = 1.0;
		}

		if self.up_pressed {
			mov_vec.y = -1.0;
		}
		else if self.down_pressed {
			mov_vec.y = 1.0;
		}

		// Normalize the vector, so that the speed is the same in every direction.
		if mov_vec != Vector::new(0.0, 0.0) {
			let normalised = mov_vec.normalise();

			mov_vec = normalised * 400.0 * game_window.frame_duration();
		}

		self.character.translate(mov_vec.x, mov_vec.y);
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
