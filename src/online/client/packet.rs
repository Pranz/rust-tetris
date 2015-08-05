use endian_type::types::*;
use num::FromPrimitive;

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
