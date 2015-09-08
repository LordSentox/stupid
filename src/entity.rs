//! # The entity trait
//!
//! The entity trait is implemented for everything living, especially stuff that moves about.

pub trait Entity {
	/// # The current health
	///
	/// The most defining part of any entity is, that they have a certain health that may be
	/// depleted. Therefor, they all have a health meter.
	fn health(&self) -> u16;

	/// # The maximum health
	///
	/// The entity's maximum health should be defined by this. Since the actual implementation may
	/// vary between entities, this is not a static function.
	fn max_health(&self) -> u16;

	/// # Damage this entity
	///
	/// This function calculates the actual damage this entity takes from the raw damage provided.
	/// It then returns the damage that was actually done. In case the entity dies with less
	/// damage than was actually inflicted, the theoretical damage is returned.
	fn damage<T: Entity>(&mut self, raw_damage: u16, source: &T) -> u16;

	/// # Heal this entity
	///
	/// This function calculates the actual healing this entity gets from the raw healing provided.
	/// It then returns the actual healing that was done. In case the healing topped off the
	/// health, this function should return the theoretical amount, not the actual amount.
	fn heal<T: Entity>(&mut self, raw_healing: u16, source: &T) -> u16;
}
