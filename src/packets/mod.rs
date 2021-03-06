//! # Net packets
//!
//! These packets are net entities that are used to send data from the one endpoint to another.
//! A packet is defined by a single byte in the stream and then may be read accordingly. They
//! resemble commands, and may be understood differently depending on the type of endpoint.

extern crate byteorder;

pub mod spawn_entity;
pub use self::spawn_entity::SpawnEntity;

impl Packet {
	/// # Get a packet's size
	///
	/// This function will check which packet the given id refers to and provide its size, so that
	/// it can be read from the stream and then constructed.
	pub fn get_matching_size(id: u8) -> u32 {
		match id {
			0 => SpawnEntity::SIZE,
			_ => panic!("Incorrect packet id.")
		}
	}
}

pub trait Packet {
	/// # The packets id
	///
	/// Every packet has a unique number, through which it can be identified.
	fn id() -> u8;

	/// # The data size
	///
	/// Every packet must have a constant data size, so that it is clear how much has to be read
	/// from the sockets stream.
	const SIZE: u32;

	/// # Create from bytes
	///
	/// This function takes a slice of data, from which the packet can be constructed the size of
	/// the slice must exactly match the associated constant "size".
	fn from_bytes(data: &[u8]) -> Self;

	/// # Write to bytes
	///
	/// This function writes the packet into a slice of data which has the length of the associated
	/// constant "size".
	fn to_bytes(&self) -> Vec<u8>;
}
