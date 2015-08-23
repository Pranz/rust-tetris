use core::mem;
use serde::{de,Serialize,Serializer,Deserialize,Deserializer};

pub struct ProtocolId;
impl Serialize for ProtocolId{
	#[inline]
	fn serialize<S>(&self,serializer: &mut S) -> Result<(),S::Error>
		where S: Serializer
	{
		serializer.visit_str("TETR")
	}
}
impl Deserialize for ProtocolId{
	#[inline]
	fn deserialize<D>(deserializer: &mut D) -> Result<Self,D::Error>
		where D: Deserializer
	{
		struct V;
		impl de::Visitor for V{
			type Value = ProtocolId;

			fn visit_str<E>(&mut self,s: &str) -> Result<Self::Value,E>
				where E: de::Error,
			{
				if s=="TETR"{
					Ok(ProtocolId)
				}else{
					Err(de::Error::syntax("Expected `TETR` as the protocol id"))
				}
			}

			fn visit_bytes<E>(&mut self,s: &[u8]) -> Result<Self::Value,E>
				where E: de::Error,
			{
				if s==b"TETR"{
					Ok(ProtocolId)
				}else{
					Err(de::Error::syntax("Expected `TETR` as the protocol id"))
				}
			}
		}
		deserializer.visit_string(V)
	}
}

pub type Id = u16;

#[derive(Serialize,Deserialize)]
pub struct Packet<Data: Serialize + Deserialize>{
	pub protocol: ProtocolId,
	pub packet: Id,
	pub data: Data,
}

impl<Data> Packet<Data>
		where Data: Serialize + Deserialize
{
	pub fn serialize(&self) -> Vec<u8>
	{
		::bincode::serde::serialize(self,::bincode::SizeLimit::Bounded(256)).unwrap()
	}
}

pub type ProtocolVersion = u16;
pub type ConnectionId = u32;
pub type PlayerNetworkId = u32;

#[inline(always)]
pub fn buffer() -> [u8; 256]{unsafe{mem::uninitialized()}}//TODO: 256 is just a constant made up on top of my head. This should be of the size of the largest packet sent/received.
