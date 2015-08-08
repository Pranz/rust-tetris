//! Online network connection related stuff
//!
//! The packets sent are packed (without padding) and its integer representations are in little endian (LE) (not network order)
//! The layout is the following: {packet_type: 1,packet_fields: n)

#[macro_use]pub mod packet;
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
