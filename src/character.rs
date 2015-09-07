//! # A players character
//!
//! The entity of a player. The entities that players control are splitted in two, because in a
//! multiplayer game, it makes more sense to have a simple entity and have an extra part that is
//! controlled by the players input.

use entity::Entity;
use sdl2::render::Renderer;
use movable::Movable;
use graphics::{Sprite, Drawable, RenderWindow};

const MAX_HEALTH: u16 = 10;

pub struct Character {
	health: u16,
	sprite: Sprite
}

impl Character {
	pub fn new(game_window: &mut RenderWindow) -> Character {
		let sprite = game_window.create_sprite("character.bmp", (0.0, 0.0), None).unwrap();

		Character {
			health: MAX_HEALTH,
			sprite: sprite
		}
	}
}

impl Entity for Character {
	fn health(&self) -> u16 {
		self.health
	}

	fn max_health(&self) -> u16 {
		10
	}

	fn damage<T: Entity>(&mut self, raw_damage: u16, _: &T) -> u16 {
		let actual_damage = raw_damage;

		if actual_damage < self.health {
			self.health -= actual_damage;
		}
		else {
			self.health = 0;
		}

		actual_damage
	}

	fn heal<T: Entity>(&mut self, raw_healing: u16, _: &T) -> u16 {
		let actual_healing = raw_healing;

		if self.health + actual_healing < self.max_health() {
			self.health += actual_healing;
		}
		else {
			self.health = self.max_health();
		}

		actual_healing
	}
}

impl Movable for Character {
	fn translate(&mut self, x: f32, y: f32) {
		self.sprite.translate(x, y);
	}

	fn set_pos(&mut self, pos: (f32, f32)) {
		self.sprite.set_pos(pos);
	}

	fn x(&self) -> f32 {
		self.sprite.x()
	}

	fn y(&self) -> f32 {
		self.sprite.y()
	}

	fn pos(&self) -> (f32, f32) {
		self.sprite.pos()
	}
}

impl Drawable for Character {
	fn draw(&self, renderer: &mut Renderer) {
		self.sprite.draw(renderer);
	}
}
