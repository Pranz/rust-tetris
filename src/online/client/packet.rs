use super::super::packet::*;
use ::data::request;

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

	///Sent when something player related is requested to the server
	PlayerRequest{
		connection: ConnectionId,
		request: request::Player<PlayerNetworkId>
	}
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
