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

			pub fn into_packet(self) -> $crate::online::Packet<$variant,Self>{
				$crate::online::Packet($variant::$ty,self)
			}
		}
	};
}
