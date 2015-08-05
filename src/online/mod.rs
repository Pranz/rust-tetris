//! Online network connection related stuff
//!
//! The packets sent are packed (without padding) and its integer representations are in little endian (LE) (not network order)
//! The layout is the following: {packet_type: 1,packet_fields: n)

#[macro_use]mod packet;
pub mod client;
pub mod server;



use std::net;

pub enum ConnectionType{
	Server,
	Client(net::UdpSocket,net::SocketAddr),
	//TODO: PeerToPeer
	None
}

#[repr(packed)]
pub struct Packet<Type: Sized + Copy,Data: Sized + Copy>(pub Type,pub Data);
impl<Type: Sized + Copy,Data: Sized + Copy> Packet<Type,Data>{
	pub fn as_bytes(&self) -> &[u8]{
		unsafe{::core::slice::from_raw_parts(
			self as *const Self as *const u8,
			::core::mem::size_of::<Self>()
		)}
	}
}
