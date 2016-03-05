use serde::{de,Serialize,Serializer,Deserialize,Deserializer};

#[derive(Copy,Clone,Debug,Eq,PartialEq)]
pub struct ProtocolId;
impl Serialize for ProtocolId{
	#[inline]
	fn serialize<S>(&self,serializer: &mut S) -> Result<(),S::Error>
		where S: Serializer
	{
		serializer.serialize_bytes(b"TETR")
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
					Err(E::invalid_value("Expected `TETR` as the protocol id"))
				}
			}

			fn visit_bytes<E>(&mut self,s: &[u8]) -> Result<Self::Value,E>
				where E: de::Error,
			{
				if s==b"TETR"{
					Ok(ProtocolId)
				}else{
					Err(E::invalid_value("Expected `TETR` as the protocol id"))
				}
			}
		}
		deserializer.deserialize_string(V)
	}
}

pub type Id = u16;

#[derive(Copy,Clone,Debug,Serialize,Deserialize)]
pub struct Packet<Data: Serialize + Deserialize>{
	pub protocol: ProtocolId,
	pub packet: Id,
	pub data: Data,
}

impl<Data> Packet<Data>
	where Data: Serialize + Deserialize
{
	#[inline(always)]
	pub fn serialize(&self) -> Vec<u8>{
		::bincode::serde::serialize(self,::bincode::SizeLimit::Bounded(256)).unwrap()
	}

	#[inline(always)]
	pub fn deserialize(bytes: &[u8]) -> Result<Self,::bincode::serde::DeserializeError>{
		::bincode::serde::deserialize(bytes)
	}
}

pub type ProtocolVersion = u16;
pub type ConnectionId = u32;
pub type PlayerNetworkId = u32;
