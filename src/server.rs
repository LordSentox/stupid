//! # Stupid game server
//!
//! The server accepts connections and shares data between the clients. Most of the games internal
//! logic is handled by the server.

#![allow(dead_code)]

#![feature(associated_consts)]

mod packets;
mod srv;
mod sys;

use srv::ConnectionHub;

pub fn main() {
	println!("This is awesome main function.");
}
