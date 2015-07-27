//! Online network connection related stuff
//!
//! The packets sent are packed (without padding) and its integer representations are in little endian (LE) (not network order)
//! The layout is the following: {packet_type: 1,packet_fields: n)

use std::net;

pub enum ConnectionType{
	Server,
	Client(net::UdpSocket,net::SocketAddr),
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

macro_rules! impl_FromPacketBytes{
	( $ty:ident : $variant:ident ) => {
		impl $ty{
			pub fn from_packet_bytes(bytes: &[u8]) -> &Self{
				use $crate::online::Packet;
				use core::mem;
				use core::raw::Repr;

				let bytes = bytes.repr();
				debug_assert_eq!(bytes.len,mem::size_of::<Packet<$variant,Self>>());
				unsafe{&(*(bytes.data as *const Packet<$variant,Self>)).1}
			}

			pub fn into_packet(self) -> $crate::online::Packet<$variant,Self>{
				$crate::online::Packet($variant::$ty,self)
			}
		}
	};
}

pub mod client{
	pub mod packet{
		use endian_type::types::*;
		use num::FromPrimitive;

		use data::input::Input;

		///Size in bytes of the biggest packet sent by the client
		pub const SIZE: usize = (1) + (4+4+1);//Type + PlayerInput

		pub type PacketBytes = [u8; SIZE];

		///Type of packet sent from the clients
		///Guaranteed to be of size 1 byte (8 bits)
		enum_from_primitive!{
		#[derive(Copy,Clone,Debug,Eq,PartialEq)]
		#[repr(u8)]
		pub enum Type{//TODO: Make all documentation to doc blocks when https://github.com/rust-lang/rust/issues/24189 is fixed
			Invalid,

			//Sent when connecting
			//
			//Fields:
			//  protocol_version: [2] u16
			Connect,

			//Sent when a ConnectionEstablished from the server is received with the same id
			//
			//Fields:
			//  connection_id: [4] u32
			ConnectionEstablishedResponse,

			//Sent when disconnecting
			//
			//Fields:
			//  connection_id    : [4] u32
			Disconnect,

			//Sent when a new local player has been added
			//
			//Fields:
			//  connection_id    : [4] u32
			//  player_network_id: [4] u32
			PlayerCreate,

			//Sent when a local player has been removed
			//
			//Fields:
			//  connection_id    : [4] u32
			//  player_network_id: [4] u32
			PlayerRemove,

			//Sent when a input command from a local player is registered
			//
			//Fields:
			//  connection_id    : [4] u32
			//  player_network_id: [4] u32
			//  input            : [1] Input|u8
			PlayerInput,

			//Sent when pinging the server, waiting for a pong from the server
			//The pong response will contain the same data as sended
			//
			//Fields:
			//  data: [4] u32
			Ping,

			//Sent when ponging a server, in response of a ping from the server
			//The ping response must contain the same data as the received ping
			//
			//Fields:
			//  data: [4] u32
			Pong,
		}}
		impl Type{
			pub fn from_packet_bytes(bytes: &[u8]) -> Option<Self>{
				debug_assert!(bytes.len() > 0);

				Type::from_u8(bytes[0])
			}
		}
//TODO: Make a macro/syntax extension for these kind of things
		#[derive(Copy,Clone,Debug,Eq,PartialEq)]#[repr(packed)]pub struct Connect                      {pub protocol_version: u16_le}                                         impl_FromPacketBytes!(Connect                      : Type);
		#[derive(Copy,Clone,Debug,Eq,PartialEq)]#[repr(packed)]pub struct ConnectionEstablishedResponse{pub connection_id: u32_le}                                            impl_FromPacketBytes!(ConnectionEstablishedResponse: Type);
		#[derive(Copy,Clone,Debug,Eq,PartialEq)]#[repr(packed)]pub struct Disconnect                   {pub connection_id: u32_le}                                            impl_FromPacketBytes!(Disconnect                   : Type);
		#[derive(Copy,Clone,Debug,Eq,PartialEq)]#[repr(packed)]pub struct PlayerCreate                 {pub connection_id: u32_le,pub player_network_id: u32_le}              impl_FromPacketBytes!(PlayerCreate                 : Type);
		#[derive(Copy,Clone,Debug,Eq,PartialEq)]#[repr(packed)]pub struct PlayerRemove                 {pub connection_id: u32_le,pub player_network_id: u32_le}              impl_FromPacketBytes!(PlayerRemove                 : Type);
		#[derive(Copy,Clone,Debug,Eq,PartialEq)]#[repr(packed)]pub struct PlayerInput                  {pub connection_id: u32_le,pub player_network_id: u32_le,pub input: u8}impl_FromPacketBytes!(PlayerInput                  : Type);
		#[derive(Copy,Clone,Debug,Eq,PartialEq)]#[repr(packed)]pub struct Ping                         {pub data: u32_le}                                                     impl_FromPacketBytes!(Ping                         : Type);
		#[derive(Copy,Clone,Debug,Eq,PartialEq)]#[repr(packed)]pub struct Pong                         {pub data: u32_le}                                                     impl_FromPacketBytes!(Pong                         : Type);
	}
}

pub mod server{
	pub mod packet{
		use endian_type::types::*;
		use num::FromPrimitive;

		use data::input::Input;

		///Size in bytes of the biggest packet sent by the server
		pub const SIZE: usize = (1) + (4+1);//Type + PlayerInput

		pub type PacketBytes = [u8; SIZE];

		///Type of packet sent from the server
		///Guaranteed to be of size 1 byte (8 bits)
		enum_from_primitive!{
		#[derive(Copy,Clone,Debug,Eq,PartialEq)]
		#[repr(u8)]
		pub enum Type{
			Invalid,

			//Sent when connecting and the connection is OK
			//
			//Fields:
			//  connection_id: [4] u32
			ConnectionEstablished,

			//Sent when connecting and the connection is not OK
			ConnectionInvalid,

			//Sent when a new player has been added
			//
			//Fields:
			//  player_network_id: [4] u32
			PlayerCreate,

			//Sent when a player has been removed
			//
			//Fields:
			//  player_network_id: [4] u32
			PlayerRemove,

			//Sent when a player input command is registered
			//
			//Fields:
			//  player_network_id: [4] u32
			//  input            : [1] Input|u8
			PlayerInput,

			//Sent when pinging the client, waiting for a pong from the client
			//The pong response will contain the same data as sended
			//
			//Fields:
			//  data: [4] u32
			Ping,

			//Sent when ponging a client, in response of a ping from the client
			//The ping response must contain the same data as the received ping
			//
			//Fields:
			//  data: [4] u32
			Pong,
		}}
		impl Type{
			pub fn from_packet_bytes(bytes: &[u8]) -> Option<Self>{
				debug_assert!(bytes.len() > 0);

				Type::from_u8(bytes[0])
			}
		}

		#[derive(Copy,Clone,Debug,Eq,PartialEq)]#[repr(packed)]pub struct ConnectionEstablished{pub connection_id: u32_le}                  impl_FromPacketBytes!(ConnectionEstablished: Type);
		#[derive(Copy,Clone,Debug,Eq,PartialEq)]#[repr(packed)]pub struct ConnectionInvalid;                                                impl_FromPacketBytes!(ConnectionInvalid    : Type);
		#[derive(Copy,Clone,Debug,Eq,PartialEq)]#[repr(packed)]pub struct PlayerCreate         {pub player_network_id: u32_le}              impl_FromPacketBytes!(PlayerCreate         : Type);
		#[derive(Copy,Clone,Debug,Eq,PartialEq)]#[repr(packed)]pub struct PlayerRemove         {pub player_network_id: u32_le}              impl_FromPacketBytes!(PlayerRemove         : Type);
		#[derive(Copy,Clone,Debug,Eq,PartialEq)]#[repr(packed)]pub struct PlayerInput          {pub player_network_id: u32_le,pub input: u8}impl_FromPacketBytes!(PlayerInput          : Type);
		#[derive(Copy,Clone,Debug,Eq,PartialEq)]#[repr(packed)]pub struct Ping                 {pub data: u32_le}                           impl_FromPacketBytes!(Ping                 : Type);
		#[derive(Copy,Clone,Debug,Eq,PartialEq)]#[repr(packed)]pub struct Pong                 {pub data: u32_le}                           impl_FromPacketBytes!(Pong                 : Type);
	}
}
