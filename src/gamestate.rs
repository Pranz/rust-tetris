use vec_map::VecMap;
use core::cmp;
use piston::input::UpdateArgs;
use rand::{self,Rand};
use serde::{Serialize,Serializer};

use data::{grid,world,player,Cell,Grid,Player,World as WorldTrait};
use data::grid::RectangularBound;
use data::shapes::tetromino::{Shape,RotatedShape};
use game::Event;
use tmp_ptr::TmpPtr;

///Type of the world id
pub type WorldId = u8;
///Type of the player id
pub type PlayerId = u8;

///The ingame game state
pub struct GameState<World,Rng>
	where World: WorldTrait
{
	///Data of the game state
	pub data: Data<World>,

	///Random number generator mappings.
	pub rngs: data_map::Mappings<Rng>,

	///Function that maps a shape's cell to the world's cell
	pub imprint_cell: fn(&RotatedShape) -> <World as Grid>::Cell,

	///Function that returns the origin position of a player based on shape and world
	pub respawn_pos: fn(&RotatedShape,&World) -> grid::Pos
}

impl<World,Rng> GameState<World,Rng>
	where World: WorldTrait
{
	///A simple constructor
	pub fn new(
		rng: Rng,
		imprint_cell: fn(&RotatedShape) -> <World as Grid>::Cell,
		respawn_pos : fn(&RotatedShape,&World) -> grid::Pos
	) -> Self{GameState{
		data: Data{
			worlds : VecMap::new(),
			players: VecMap::new(),
		},
		rngs        : data_map::Mappings::new(rng),
		imprint_cell: imprint_cell,
		respawn_pos : respawn_pos,
	}}

	///Updates the game state
	pub fn update<EL>(&mut self, args: &UpdateArgs,event_listener: &mut EL)
		where World: WorldTrait,
		      <World as Grid>::Cell: Cell,
		      Rng: rand::Rng,
		      EL: FnMut(Event<(PlayerId,TmpPtr<Player>),(WorldId,TmpPtr<World>)>)
	{
		//After action
		enum Action{
			None,
			ResetWorld(WorldId)
		}let mut action = Action::None;

		//Players
		'player_loop: for (player_id,player) in self.data.players.iter_mut(){
			let player_id = player_id  as PlayerId;
			let world_id  = player.world as WorldId;

			if let Some(world) = self.data.worlds.get_mut(&(player.world as usize)){
				//Add the time since the last update to the time counts
				player.gravityfall_time_count -= args.dt;

				//Gravity: If the time count is greater than the shape move frequency, then repeat until it is smaller
				while player.gravityfall_time_count <= 0.0{
					//Add one step of frequency
					player.gravityfall_time_count += player.settings.gravityfall_frequency;

					//If able to move (no collision below)
					if move_player(player,world,grid::Pos{x: 0,y: 1}){
						event_listener(Event::PlayerMoveGravity{
							player: (player_id,TmpPtr::new(player as &_)),
							world: (world_id,TmpPtr::new(world as &_))
						});
					}else{
						//Imprint the current shape onto the world
						world.imprint_shape(&player.shape,player.pos,&self.imprint_cell);

						//Handles the filled rows
						let min_y = cmp::max(0,player.pos.y) as grid::SizeAxis;
						let max_y = cmp::min(min_y + player.shape.height(),world.height());
						let full_rows = if min_y!=max_y{
							world.handle_full_rows(min_y .. max_y)
						}else{
							0
						};

						event_listener(Event::WorldImprintShape{
							world: (world_id,TmpPtr::new(world as &_)),
							shape: (player.shape,player.pos),
							full_rows: full_rows,
							cause: Some((player_id,TmpPtr::new(player as &_))),
						});

						//Respawn player and check for collision at spawn position
						let shape = player.next_shape(<Shape as Rand>::rand(self.rngs.player_get(world_id,player_id)));
						if !respawn_player((player_id,player),(world_id,world),shape,self.respawn_pos,event_listener){
							action = Action::ResetWorld(world_id);
							break 'player_loop;
						}
					}
				}
			}
		}

		match action{
			Action::None => (),
			Action::ResetWorld(world_id) => self.reset_world(world_id,event_listener),
		};
	}

	///Adds a player to the specified world and with the specified player settings
	///Returns the new player id
	pub fn add_player<EL>(&mut self,world_id: WorldId,settings: player::Settings,event_listener: &mut EL) -> Option<PlayerId>
		where World: WorldTrait,
		      Rng: rand::Rng,
		      EL: FnMut(Event<(PlayerId,TmpPtr<Player>),(WorldId,TmpPtr<World>)>)
	{
		if let Some(world) = self.data.worlds.get_mut(&(world_id as usize)){
			let new_id = self.data.players.len();
			let shape = RotatedShape::new(<Shape as rand::Rand>::rand(self.rngs.player_get(world_id,new_id as PlayerId)));

			self.data.players.insert(new_id,Player{
				pos                   : (self.respawn_pos)(&shape,world),
				shadow_pos            : None,
				shapes_lookahead      : None,
				shape                 : shape,
				world                 : world_id,
				points                : 0,
				gravityfall_time_count: settings.gravityfall_frequency,
				settings              : settings
			});
			let player = self.data.players.get_mut(&new_id).unwrap();

			event_listener(Event::PlayerAdd{
				player: (new_id as PlayerId,TmpPtr::new(player as &_)),
				world: (world_id,TmpPtr::new(world as &_)),
			});

			Some(new_id as PlayerId)
		}else{
			None
		}
	}

	///Resets the specified world, respawning all players and resetting time counts
	pub fn reset_world<EL>(&mut self,world_id: WorldId,event_listener: &mut EL)
		where World: WorldTrait,
		      <World as Grid>::Cell: Cell,
		      Rng: rand::Rng,
		      EL: FnMut(Event<(PlayerId,TmpPtr<Player>),(WorldId,TmpPtr<World>)>)
	{
		if let Some(world) = self.data.worlds.get_mut(&(world_id as usize)){
			//Clear world
			world.clear();

			for (player_id,player) in self.data.players.iter_mut().filter(|&(_,ref player)| player.world == world_id){
				//Reset all players in the world
				let shape = player.next_shape(<Shape as Rand>::rand(self.rngs.player_get(world_id,player_id as PlayerId)));
				respawn_player((player_id as PlayerId,player),(world_id,world),shape,self.respawn_pos,event_listener);
				player.gravityfall_time_count = player.settings.gravityfall_frequency;
			}
		};
	}
}

pub mod data_map{
	use core::mem;
	use std::collections::hash_map::HashMap;

	///Kind of data mappings
	#[derive(Copy,Clone,Debug,Eq,PartialEq,Hash)]
	pub enum MappingKey{
		World(super::WorldId),
		Player(super::PlayerId)
	}

	///Contains mappings with a global fallback.
	///The first field contains the global data.
	///The second field contains a map of datas.
	///When looking up a mapping and it does not exist, it falls back to the more global one in the following order:
	///  Player -> World -> Global
	pub struct Mappings<T>(T,pub HashMap<MappingKey,T>);

	impl<T> Mappings<T>{
		///Constructs a RNG mappings container with a default global fallback
		pub fn new(global: T) -> Self{Mappings(global,HashMap::new())}

		///Gets the global fallback data
		#[inline]pub fn global(&mut self) -> &mut T{&mut self.0}

		///Adds a copy of the global fallback data to the specified mapping
		#[inline]pub fn insert_from_global(&mut self,mapping: MappingKey)
			where T: Clone
		{
			self.1.insert(mapping,self.0.clone());
		}

		///Lookup data from a world mapping with fallbacks.
		pub fn world_get(&mut self,world: super::WorldId) -> &mut T{
			self.1.get_mut(&MappingKey::World(world)).unwrap_or(&mut self.0)
		}

		///Lookup data from a player mapping with fallbacks.
		pub fn player_get(&mut self,world: super::WorldId,player: super::PlayerId) -> &mut T{
			let mappings1: &mut HashMap<MappingKey,T> = unsafe{mem::transmute(&mut self.1)};
			let mappings2: &mut HashMap<MappingKey,T> = unsafe{mem::transmute(&mut self.1)};

			match mappings1.get_mut(&MappingKey::Player(player)){
				Some(rng) => return rng,
				None => ()
			};

			match mappings2.get_mut(&MappingKey::World(world)){
				Some(rng) => return rng,
				None => ()
			};

			&mut self.0
		}
	}
}

///Moves player if there are no collisions at the new position.
///Returns whether the movement was successful or not due to collisions.
pub fn move_player<World>(player: &mut Player,world: &World,delta: grid::Pos) -> bool
	where World: WorldTrait
{
	//Collision check
	match world.shape_intersects(&player.shape,player.pos + delta){
		//Collided => cannot move
		world::CellIntersection::Imprint(_) |
		world::CellIntersection::OutOfBounds(_) => false,

		//No collision, able to move and does so
		world::CellIntersection::None => {
			//Change position
			player.pos.x += delta.x;
			player.pos.y += delta.y;

			//Recalcuate fastfall shadow position when moving horizontally
			if player.settings.fastfall_shadow && delta.x!=0{
				player.shadow_pos = Some(fastfallen_shape_pos(&player.shape,world,player.pos));
			}

			true
		}
	}
}

///Checks if the player with the transformed shape is intersecting with the stuff in the world or the world boundaries.
///If that is true, try to resolve the collision by moving in the x axis.
///If the collision cannot resolve, undo the rotation and return false, otherwise return true.
pub fn resolve_transformed_player<World>(player: &mut Player,shape: RotatedShape,world: &World) -> bool
	where World: WorldTrait
{
	'try_rotate: loop{
		match world.shape_intersects(&shape,player.pos){
			world::CellIntersection::Imprint(pos) |
			world::CellIntersection::OutOfBounds(pos) => {
				let center_x = player.pos.x + player.shape.center_x() as grid::PosAxis;
				let sign = if pos.x < center_x {1} else {-1};
				for i in 1..shape.width(){
					if let world::CellIntersection::None = world.shape_intersects(&shape,player.pos.with_x(|x| x + (i as grid::PosAxis * sign))){
						player.pos.x += i as grid::PosAxis * sign;
						break 'try_rotate;
					}
				}
			},
			_ => break 'try_rotate
		}

		return false;
	}

	{//Successfully rotated
		player.shape = shape;

		//Recalcuate fastfall shadow position when moving horizontally
		if player.settings.fastfall_shadow{
			player.shadow_pos = Some(fastfallen_shape_pos(&player.shape,world,player.pos));
		}
		return true;
	}
}

///Respawns player to its origin position
///Returns whether the respawning was successful or not due to collisions.
pub fn respawn_player<World,EL>((player_id,player): (PlayerId,&mut Player),(world_id,world): (WorldId,&World),new_shape: Shape,respawn_pos: fn(&RotatedShape,&World) -> grid::Pos,event_listener: &mut EL) -> bool
	where World: WorldTrait,
	      EL: FnMut(Event<(PlayerId,TmpPtr<Player>),(WorldId,TmpPtr<World>)>)
{
	//Select a new shape at random, setting its position to the starting position
	let pos = respawn_pos(&player.shape,world);

	event_listener(Event::PlayerChangeShape{
		player: (player_id,TmpPtr::new(player as &_)),
		world: (world_id,TmpPtr::new(world)),
		shape: new_shape,
		pos: pos,
	});

	player.shape = RotatedShape::new(new_shape);
	player.pos = pos;

	if player.settings.fastfall_shadow{
		player.shadow_pos = Some(fastfallen_shape_pos(&player.shape,world,player.pos));
	}

	//If the new shape at the starting position also collides with another shape
	match world.shape_intersects(&player.shape,player.pos){
		world::CellIntersection::Imprint(_) => false,
		_ => true
	}
}

///Returns the position of the shape if it were to fast fall downwards in the world at the given position
pub fn fastfallen_shape_pos<World>(shape: &RotatedShape,world: &World,shape_pos: grid::Pos) -> grid::Pos
	where World: WorldTrait
{
	for y in shape_pos.y .. world.height() as grid::PosAxis{
		match world.shape_intersects(&shape,shape_pos.with_y(y+1)){
			world::CellIntersection::Imprint(_)     |
			world::CellIntersection::OutOfBounds(_) => return shape_pos.with_y(y),
			_ => ()
		};
	}

	unreachable!()
}

#[derive(Clone)]
pub struct Data<W>{
	///Mappings of worlds and world ids
	pub worlds: VecMap<W>,

	///Mappings of players and player ids
	pub players: VecMap<Player>,
}

impl<W> Serialize for Data<W>
	where W: WorldTrait,
	      W: Grid,
	      <W as Grid>::Cell: Cell + Copy
{
	#[inline]
	fn serialize<S>(&self,serializer: &mut S) -> Result<(),S::Error>
		where S: Serializer
	{
		grid::serde::GridSerializer::<_,W>::new(&self.worlds[0]).visit(serializer)
	}
}
/*impl Deserialize for Data<World>{
	#[inline]
	fn deserialize<D>(deserializer: &mut D) -> Result<Self,D::Error>
		where D: Deserializer
	{
		struct V<World>;
		impl de::Visitor for V<World>{
			type Value = Data<World>;

			fn visit_str<E>(&mut self,s: &str) -> Result<Self::Value,E>
				where E: de::Error,
			{
				if s=="TETR"{
					Ok(Data<World>)
				}else{
					Err(de::Error::syntax("Expected `TETR` as the protocol id"))
				}
			}

			fn visit_bytes<E>(&mut self,s: &[u8]) -> Result<Self::Value,E>
				where E: de::Error,
			{
				if s==b"TETR"{
					Ok(Data<World>)
				}else{
					Err(de::Error::syntax("Expected `TETR` as the protocol id"))
				}
			}
		}
		deserializer.visit_string(V)
	}
}*/
