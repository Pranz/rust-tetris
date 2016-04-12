pub mod input;
pub mod mappings;
pub mod player;
pub mod world;

pub use self::input::Input;
pub use self::mappings::Mappings;
pub use self::player::Player;
pub use self::world::World;



use serde::{Serialize,Serializer};
use vec_map::VecMap;

use ::data::{grid,Cell,Grid};

///Type of the world id
pub type WorldId = u8;

///Type of the player id
pub type PlayerId = u8;

#[derive(Clone)]
pub struct Data<W>{
	///Mappings of world ids to worlds
	pub worlds: VecMap<(W,bool)>,

	///Mappings of player ids to players
	pub players: VecMap<Player>,
}

impl<W> Data<W>{
	pub fn new() -> Self{Data{
		worlds : VecMap::new(),
		players: VecMap::new(),
	}}
}

impl<W> Serialize for Data<W>
	where W: World,
	      W: Grid,
	      <W as Grid>::Cell: Cell + Copy
{
	#[inline]
	fn serialize<S>(&self,serializer: &mut S) -> Result<(),S::Error>
		where S: Serializer
	{
		grid::serde::GridSerializer::<_,W>::new(&self.worlds[0].0).visit(serializer)
	}
}
/*impl Deserialize for Data<W>{
	#[inline]
	fn deserialize<D>(deserializer: &mut D) -> Result<Self,D::Error>
		where D: Deserializer
	{
		struct V<W>;
		impl de::Visitor for V<W>{
			type Value = Data<W>;

			fn visit_str<E>(&mut self,s: &str) -> Result<Self::Value,E>
				where E: de::Error,
			{
				if s=="TETR"{
					Ok(Data<W>)
				}else{
					Err(de::Error::syntax("Expected `TETR` as the protocol id"))
				}
			}

			fn visit_bytes<E>(&mut self,s: &[u8]) -> Result<Self::Value,E>
				where E: de::Error,
			{
				if s==b"TETR"{
					Ok(Data<W>)
				}else{
					Err(de::Error::syntax("Expected `TETR` as the protocol id"))
				}
			}
		}
		deserializer.visit_string(V)
	}
}*/
