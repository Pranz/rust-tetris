use super::super::packet::*;
use data::input::Input;
use data::player;

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
	PlayerCreatedResponse{
		player  : PlayerNetworkId,
		rng_seed: u32,
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
