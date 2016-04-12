//! Online network connection related stuff
//!
//! The packets sent are serialized by `serde` and `bincode`

pub mod packet;
pub mod client;
pub mod server;

pub use self::packet::Packet;



use std::net;

use self::packet::{ConnectionId,PlayerNetworkId};
use ::data::PairMap;
use ::game::data::PlayerId;

pub enum ConnectionType{
	Server(PairMap<PlayerNetworkId,PlayerId>),
	Client(PairMap<PlayerNetworkId,PlayerId>,Option<ConnectionId>,net::UdpSocket,net::SocketAddr),
	//TODO: PeerToPeer
	None
}
