use super::super::packet::*;
use data::{player,Input};
use data::shapes::tetromino::Shape;

///Type of packet sent from the server
#[derive(Copy,Clone,Debug,PartialEq,Serialize,Deserialize)]
pub enum Data{
	Invalid,

	///Sent when connecting and the connection is OK
	ConnectionEstablished{
		connection: ConnectionId,
	},

	///Sent when connecting and the connection is not OK
	ConnectionInvalid,

	///Sent when a new player request has been confirmed
	PlayerCreateResponse{
		player  : PlayerNetworkId,
		rng_seed: u32,//TODO: Send the whole RNG world in a packet instead?
	},

	///Sent when a new player has been added
	PlayerCreate{
		player  : PlayerNetworkId,
		rng_seed: u32,
		settings: player::Settings,
	},

	///Sent when a player has been removed
	PlayerRemove{
		player: PlayerNetworkId,
	},

	///Sent when a player input command is registered
	PlayerInput{
		player: PlayerNetworkId,
		input : Input,
	},

	///Sent when a player's shape
	PlayerQueueShape{
		player: PlayerNetworkId,
		shape : Shape
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
