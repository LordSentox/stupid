/*
 * The connection hub class is there to manage incoming connections, aswell as establishing new
 * connections. It is the central hub to communicate with specific clients.
 */

use std::net::{TcpListener, UdpSocket, SocketAddr, ToSocketAddrs};

use std::sync::{Arc, Mutex};
use std::thread;

use std::collections::HashMap;
use srv::User;
use packets::Packet;

type SafeUserMap = Arc<Mutex<HashMap<SocketAddr, User>>>;

pub struct ConnectionHub {
	listener: TcpListener,
	udp_socket: UdpSocket,
	users: SafeUserMap
}

impl ConnectionHub {
	pub fn new(port: u16) -> Result<ConnectionHub, String> {
		// Try to bind the server to the port specified.
		let listener = match TcpListener::bind(("127.0.0.1", port)) {
			Ok(listener) => listener,
			Err(err) => return Err(format!("Unable to open port on Tcp. {}", err))
		};

		let udp_socket = match UdpSocket::bind(("127.0.0.1", port)) {
			Ok(socket) => socket,
			Err(err) => return Err(format!("Unable to open port on Udp. {}", err))
		};

		let connection_hub = ConnectionHub {
			listener: listener,
			udp_socket: udp_socket,
			users: Arc::new(Mutex::new(HashMap::new()))
		};

		let listener_clone = connection_hub.listener.try_clone().unwrap();
		let users_clone = connection_hub.users.clone();
		thread::spawn(move || {
			ConnectionHub::accept_connections(listener_clone, users_clone);
		});

		let udp_socket_clone = connection_hub.udp_socket.try_clone().unwrap();
		let users_clone = connection_hub.users.clone();
		thread::spawn(move || {
			ConnectionHub::receive_udp(udp_socket_clone, users_clone);
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

	fn receive_udp(udp_socket: UdpSocket, users: SafeUserMap) {
	}

	/// # Send a packet using TCP
	///
	/// This is slower that send_udp(), however one can assume that all packets are received intact
	/// and the order is precisely as it was before.
	pub fn send_tcp<A: ToSocketAddrs, P: Packet>(&mut self, addr: &A, data: &P) -> Result<usize, String> {
		// Most beautiful. (*cough* TODO *cough*)
		let mut user_map = self.users.lock().unwrap();
		let mut user = user_map.get_mut(&addr.to_socket_addrs().unwrap().nth(0).unwrap()).unwrap();

		user.send_tcp(data)
	}

	/// # Send a packet using UDP
	///
	/// This is useful for packets that ought to arrive fast and may have certain flaws. If it is
	/// important that the packet's integrity is guaranteed or a series of packets arrives in the
	/// correct order, consider using send_tcp instead.
	pub fn send_udp<A: ToSocketAddrs, P: Packet>(&mut self, addr: &A, data: &P) -> Result<usize, String> {
		let user_map = self.users.lock().unwrap();
		let addr = addr.to_socket_addrs().unwrap().nth(0).unwrap();

		if !user_map.contains_key(&addr) {
			return Err(format!("A user with address {} is not connected.", addr));
		}

		let packet_id = P::id();
		let mut vec = vec![packet_id];
		vec.append(&mut data.to_bytes());

		match self.udp_socket.send_to(&vec[..], &addr) {
			Ok(size) => Ok(size),
			Err(err) => Err(format!("{}", err))
		}
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
