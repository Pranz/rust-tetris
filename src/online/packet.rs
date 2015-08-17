use endian_type::types::*;

pub const HEADER_SIZE: usize = 1 + 2;//Packet type + Packet id

pub type Id = u16_le;

#[repr(packed)]
pub struct Header<Type: Sized + Copy>(pub Type,pub Id);

#[repr(packed)]
pub struct Packet<Type: Sized + Copy,Data: Sized + Copy>(pub Header<Type>,pub Data);
impl<Type: Sized + Copy,Data: Sized + Copy> Packet<Type,Data>{
	pub fn as_bytes(&self) -> &[u8]{
		unsafe{::core::slice::from_raw_parts(
			self as *const Self as *const u8,
			::core::mem::size_of::<Self>()
		)}
	}
}

macro_rules! impl_FromPacketBytes{
	( $ty:ident : $variant:ident ) => {
		impl $ty{
			pub fn from_packet_bytes(bytes: &[u8]) -> &Self{
				use $crate::online::Packet;
				use core::mem;
				use core::raw::Repr;

				let bytes = bytes.repr();
				debug_assert_eq!(bytes.len,mem::size_of::<Packet<$variant,Self>>());
				unsafe{&(*(bytes.data as *const Packet<$variant,Self>)).1}
			}

			pub fn into_packet(self,id: $crate::online::packet::Id) -> $crate::online::Packet<$variant,Self>{
				$crate::online::Packet($crate::online::packet::Header($variant::$ty,id),self)
			}
		}
	};
}
