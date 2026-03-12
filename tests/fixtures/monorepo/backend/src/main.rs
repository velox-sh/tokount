mod db;
mod routes;

use std::net::SocketAddr;

/// Application entry point
fn main() {
	let addr: SocketAddr = "0.0.0.0:8080".parse().unwrap();
	println!("listening on {addr}");
}
