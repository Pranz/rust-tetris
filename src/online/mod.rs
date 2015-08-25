//! Online network connection related stuff
//!
//! The packets sent are serialized by `serde` and `bincode`

pub mod packet;
pub mod client;
pub mod server;

pub use self::packet::Packet;



use std::net;

pub enum ConnectionType{
	Server,
	Client(net::UdpSocket,net::SocketAddr),
	//TODO: PeerToPeer
	None
}
