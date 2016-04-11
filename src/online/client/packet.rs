use super::super::packet::*;
use ::game::Request;

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
	Request{
		connection: ConnectionId,
		request: Request<PlayerNetworkId,WorldNetworkId>
	}
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
