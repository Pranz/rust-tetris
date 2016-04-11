//! Online network connection related stuff
//!
//! The packets sent are packed (without padding) and its integer representations are in little endian (LE) (not network order)
//! The layout is the following: {packet_type: 1,packet_fields: n)

pub mod packet;
pub mod client;
pub mod server;

pub use self::packet::Packet;



use std::net;

use data::pair_map::PairMap;
use gamestate::PlayerId;
use self::packet::{ConnectionId,PlayerNetworkId};

pub enum ConnectionType{
	Server(PairMap<PlayerNetworkId,PlayerId>),
	Client(ConnectionId,PairMap<PlayerNetworkId,PlayerId>,net::UdpSocket,net::SocketAddr),
	//TODO: PeerToPeer
	None
}
