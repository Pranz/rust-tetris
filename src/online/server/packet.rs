use super::super::packet::*;
use ::data::shapes::tetromino::Shape;
use ::game::data::{player,Input};

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

	///Sent when a new player has been added
	PlayerCreated{
		player  : PlayerNetworkId,
		settings: player::Settings,
	},

	///Sent when a player has been removed
	PlayerRemoved{
		player: PlayerNetworkId,
	},

	///Sent when a player input command is registered
	PlayerInput{
		player: PlayerNetworkId,
		input : Input,
	},

	///Sent when a player's shape
	PlayerQueuedShape{
		player: PlayerNetworkId,
		shape : Shape
	},
}

impl Data{
	#[inline(always)]
	pub fn into_packet(self,id: Id) -> Packet<Self>{
		Packet{
			protocol: ProtocolId,
			packet: id,
			data: self
		}
	}
}
