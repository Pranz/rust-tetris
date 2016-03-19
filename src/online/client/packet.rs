use super::super::packet::*;
use ::data::{player,Input};

///Type of packet sent from the clients
#[derive(Copy,Clone,Debug,PartialEq,Serialize,Deserialize)]
pub enum Data{
	Invalid,

	///Sent when connecting
	Connect{
		protocol_version: ProtocolVersion,
	},

	///Sent when a ConnectionEstablished from the server is received with the same id
	ConnectionEstablishedResponse{
		connection: ConnectionId,
	},

	///Sent when disconnecting
	Disconnect{
		connection: ConnectionId,
	},

	///Sent when a new local player has been added
	PlayerCreateRequest{
		connection: ConnectionId,
		settings  : player::Settings,
	},

	///Sent when a local player has been removed
	PlayerRemoveRequest{
		connection: ConnectionId,
		player    : PlayerNetworkId,
	},

	///Sent when a input command from a local player is registered
	PlayerInput{
		connection: ConnectionId,
		player    : PlayerNetworkId,
		input     : Input
	},
}

impl Data{
	pub fn into_packet(self,id: Id) -> Packet<Self>{
		Packet{
			protocol: ProtocolId,
			packet: id,
			data: self
		}
	}
}
