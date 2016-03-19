use core::mem;
use std::collections::hash_map::HashMap;

///Kind of data mappings
#[derive(Copy,Clone,Debug,Eq,PartialEq,Hash)]
pub enum Key{
	World(super::WorldId),
	Player(super::PlayerId)
}

///Contains mappings with a global fallback.
///The first field contains the global data.
///The second field contains a map of datas.
///When looking up a mapping and it does not exist, it falls back to the more global one in the following order:
///  Player -> World -> Global
pub struct Mappings<T>(T,pub HashMap<Key,T>);

impl<T> Mappings<T>{
	///Constructs a RNG mappings container with a default global fallback
	pub fn new(global: T) -> Self{Mappings(global,HashMap::new())}

	///Gets the global fallback data
	#[inline]pub fn global(&mut self) -> &mut T{&mut self.0}

	///Adds a copy of the global fallback data to the specified mapping
	#[inline]pub fn insert_from_global(&mut self,mapping: Key)
		where T: Clone
	{
		self.1.insert(mapping,self.0.clone());
	}

	///Lookup data from a world mapping with fallbacks.
	pub fn world_get_mut(&mut self,world: super::WorldId) -> &mut T{
		self.1.get_mut(&Key::World(world)).unwrap_or(&mut self.0)
	}

	///Lookup data from a player mapping with fallbacks.
	pub fn player_get_mut(&mut self,world: super::WorldId,player: super::PlayerId) -> &mut T{
		let mappings1: &mut HashMap<Key,T> = unsafe{mem::transmute(&mut self.1)};
		let mappings2: &mut HashMap<Key,T> = unsafe{mem::transmute(&mut self.1)};

		match mappings1.get_mut(&Key::Player(player)){
			Some(rng) => return rng,
			None => ()
		};

		match mappings2.get_mut(&Key::World(world)){
			Some(rng) => return rng,
			None => ()
		};

		&mut self.0
	}

	///Lookup data from a world mapping with fallbacks.
	pub fn world_get(&self,world: super::WorldId) -> &T{
		self.1.get(&Key::World(world)).unwrap_or(&self.0)
	}

	///Lookup data from a player mapping with fallbacks.
	pub fn player_get(&self,world: super::WorldId,player: super::PlayerId) -> &T{
		let mappings1: &HashMap<Key,T> = unsafe{mem::transmute(&self.1)};
		let mappings2: &HashMap<Key,T> = unsafe{mem::transmute(&self.1)};

		match mappings1.get(&Key::Player(player)){
			Some(rng) => rng,
			None => match mappings2.get(&Key::World(world)){
				Some(rng) => rng,
				None => &self.0
			}
		}
	}
}
