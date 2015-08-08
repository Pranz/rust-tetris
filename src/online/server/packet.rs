use endian_type::types::*;
use num::FromPrimitive;

use super::super::packet::HEADER_SIZE;

///Size in bytes of the biggest packet sent by the server
pub const SIZE: usize = HEADER_SIZE + (4+1);//PacketHeader + PlayerInput

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

	//Sent when a new player request has been confirmed
	//
	//Fields:
	//  player_network_id: [4] u32
	//  rng_seed         : [4] u32
	PlayerCreatedResponse,

	//Sent when a new player has been added
	//
	//Fields:
	//  player_network_id: [4] u32
	//  TODO: rng_seed         : [4] u32
	//  TODO: settings         : [_] player::Settings
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

#[derive(Copy,Clone,Debug,Eq,PartialEq)]#[repr(packed)]pub struct ConnectionEstablished{pub connection_id: u32_le}                         impl_FromPacketBytes!(ConnectionEstablished: Type);
#[derive(Copy,Clone,Debug,Eq,PartialEq)]#[repr(packed)]pub struct ConnectionInvalid;                                                       impl_FromPacketBytes!(ConnectionInvalid    : Type);
#[derive(Copy,Clone,Debug,Eq,PartialEq)]#[repr(packed)]pub struct PlayerCreatedResponse{pub player_network_id: u32_le,pub rng_seed: u32_le}impl_FromPacketBytes!(PlayerCreatedResponse: Type);
#[derive(Copy,Clone,Debug,Eq,PartialEq)]#[repr(packed)]pub struct PlayerCreate         {pub player_network_id: u32_le}                     impl_FromPacketBytes!(PlayerCreate         : Type);
#[derive(Copy,Clone,Debug,Eq,PartialEq)]#[repr(packed)]pub struct PlayerRemove         {pub player_network_id: u32_le}                     impl_FromPacketBytes!(PlayerRemove         : Type);
#[derive(Copy,Clone,Debug,Eq,PartialEq)]#[repr(packed)]pub struct PlayerInput          {pub player_network_id: u32_le,pub input: u8}       impl_FromPacketBytes!(PlayerInput          : Type);
#[derive(Copy,Clone,Debug,Eq,PartialEq)]#[repr(packed)]pub struct Ping                 {pub data: u32_le}                                  impl_FromPacketBytes!(Ping                 : Type);
#[derive(Copy,Clone,Debug,Eq,PartialEq)]#[repr(packed)]pub struct Pong                 {pub data: u32_le}                                  impl_FromPacketBytes!(Pong                 : Type);
