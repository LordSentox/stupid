/*
 * The user represents a structure that can receive data from  or send data to a certain user that
 * the program is currently connected to.
 */

use std::io::prelude::*;
use std::net::{TcpStream, SocketAddr, Shutdown};
use std::thread::{self, JoinHandle};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

pub struct User {
	stream: TcpStream,
	open: Arc<AtomicBool>
}

impl User {
	/// # New User
	///
	/// Creates a new user from the stream provided. If the streams read channel has been shutdown,
	/// The user is assumed to be dead.
	pub fn new(stream: TcpStream) -> User {
		let receive_stream = stream.try_clone().unwrap();
		let open = Arc::new(AtomicBool::new(true));
		let open_clone = open.clone();
		thread::spawn(move || {
			User::receive_messages(receive_stream, open_clone);
		});

		println!("Connected to {}", stream.peer_addr().unwrap());

		User {
			stream: stream,
			open: open
		}
	}

	/// # Receive Messages
	///
	/// Private function used to
	fn receive_messages(mut stream: TcpStream, open: Arc<AtomicBool>) {
		while open.load(Ordering::Relaxed) {
			let mut data = vec![0; 10];

			let size = match stream.read(&mut data) {
				Ok(size) => size,
				Err(err) => {
					println!("{:?} has been closed due to: {}", stream, err);
					open.store(false, Ordering::Relaxed);
					break
				}
			};

			if size == 0 {
				println!("{} disconnected.", stream.peer_addr().unwrap());
				open.store(false, Ordering::Relaxed);
			}
			else {
				let message = String::from_utf8(data).unwrap();
				println!("Message received from '{}':", stream.peer_addr().unwrap());
				println!("{}", message);
			}
		}

		println!("Closing stream: {:?}", stream);
		stream.shutdown(Shutdown::Both).unwrap();
		drop(stream);
	}

	pub fn send_message(&mut self, msg: &str) -> Result<usize, String> {
		let data = msg.as_bytes();

		// TODO: This is a stupid workaround, because the io::error::Error type is private.
		match self.stream.write(&data) {
			Ok(size) => Ok(size),
			Err(err) => Err(format!("{}", err))
		}
	}

	pub fn remote_address(&self) -> SocketAddr {
		self.stream.peer_addr().unwrap()
	}

	pub fn is_open(&self) -> bool {
		self.open.load(Ordering::Relaxed)
	}

	pub fn close(&mut self) {
		self.stream.shutdown(Shutdown::Both).unwrap();
	}
}

impl Drop for User {
	fn drop(&mut self) {
		// Closing the stream will make the receiving thread realize it has been cancelled.
		println!("Dropping user: {}", self.remote_address());
		self.stream.shutdown(Shutdown::Both).unwrap();

		// TODO: This is a workaround because I cannot use a JoinHandle here. This is because it
		// be owned in order for the thread to be joinable.
		while self.is_open() {
			thread::sleep_ms(50);
		}

		println!("User has been dropped properly.");
	}
}
