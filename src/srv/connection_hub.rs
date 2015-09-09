/*
 * The connection hub class is there to manage incoming connections, aswell as establishing new
 * connections. It is the central hub to communicate with specific clients.
 */

use std::io::prelude::*;
use std::net::{TcpListener, TcpStream, SocketAddr, ToSocketAddrs};
use std;

use std::sync::{Arc, Mutex};
use std::thread;

use std::collections::HashMap;
use srv::User;

type SafeUserMap = Arc<Mutex<HashMap<SocketAddr, User>>>;

pub struct ConnectionHub {
	listener: TcpListener,
	users: SafeUserMap
}

impl ConnectionHub {
	pub fn new(port: u16) -> Result<ConnectionHub, String> {
		// Try to bind the server to the port specified.
		let listener = match TcpListener::bind(("127.0.0.1", port)) {
			Ok(listener) => listener,
			Err(err) => return Err(format!("Unable to open port. {}", err))
		};

		let connection_hub = ConnectionHub {
			listener: listener,
			users: Arc::new(Mutex::new(HashMap::new()))
		};

		let listener_clone = connection_hub.listener.try_clone().unwrap();
		let users_clone = connection_hub.users.clone();
		thread::spawn(move || {
			ConnectionHub::accept_connections(listener_clone, users_clone);
		});

		Ok(connection_hub)
	}

	fn accept_connections(listener: TcpListener, users: SafeUserMap) {
		for stream in listener.incoming() {
			let user = User::new(stream.unwrap());

			let mut user_map = users.lock().unwrap();
			user_map.insert(user.remote_address(), user);
		}

		drop(listener);
	}

	// TODO: Instead of a function that simply calls another function, it would be nice to get the
	// client back. However at the moment I am just too tired to fight the borrow-checker, sorry.
	pub fn send_message<A: ToSocketAddrs>(&mut self, addr: &A, msg: &str) -> Result<usize, String> {
		// Most beautiful. (*cough* TODO *cough*)
		let mut user_map = self.users.lock().unwrap();
		let mut user = user_map.get_mut(&addr.to_socket_addrs().unwrap().nth(0).unwrap()).unwrap();

		user.send_message(msg)
	}
}

impl Drop for ConnectionHub {
	fn drop(&mut self) {
		let mut user_map = self.users.lock().unwrap();
		for user in &*user_map {
			drop(user);
		}

		user_map.clear();
		println!("All users have been dropped.");
	}
}
