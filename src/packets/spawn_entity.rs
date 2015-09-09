//! # Spawn any entity
//!
//! Command to spawn an entity with the correct type and the given instructions.

use packets::Packet;
use sys::Vector;

use std::io::Cursor;
use packets::byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

pub struct SpawnEntity {
	/// The id of the entity in the game world. Later it is going to be referenced by its id only.
	pub id: u32,

	// The type of the entity.
	pub kind: u8,

	// The maximum health is sent, since it may vary between entities of the same kind.
	pub max_health: u16,

	// The health of the entity at the moment it was sent.
	pub health: u16,

	// The position of the entity.
	pub pos: Vector<f32>
}

impl Packet for SpawnEntity {
	const ID: u8 = 0;
	const SIZE: u32 = 13;

	fn from_bytes(data: &[u8]) -> SpawnEntity {
		let mut data = Cursor::new(data);
		let mut pos = Vector::new(0.0, 0.0);

		pos.x = data.read_f32::<BigEndian>().unwrap();
		pos.y = data.read_f32::<BigEndian>().unwrap();

		SpawnEntity {
			        id: data.read_u32::<BigEndian>().unwrap(),
			      kind: data.read_u8().unwrap(),
			max_health: data.read_u16::<BigEndian>().unwrap(),
			    health: data.read_u16::<BigEndian>().unwrap(),
			       pos: pos
		}
	}

	fn to_bytes(&self) -> Vec<u8> {
		let mut data = Vec::new();

		data.write_f32::<BigEndian>(self.pos.x).unwrap();
		data.write_f32::<BigEndian>(self.pos.y).unwrap();
		data.write_u32::<BigEndian>(self.id).unwrap();
		data.write_u8(self.kind).unwrap();
		data.write_u16::<BigEndian>(self.max_health).unwrap();
		data.write_u16::<BigEndian>(self.health).unwrap();

		data
	}
}
