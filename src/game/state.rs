use core::cmp;
use piston::input::UpdateArgs;
use rand::{self,Rand};

use ::data::{grid,Cell,Grid};
use ::data::grid::RectangularBound;
use ::data::shapes::tetromino::{Shape,RotatedShape};
use ::game::{data,event,Data,Event};
use ::game::data::{world,player,Player,PlayerId,World,WorldId};

///The ingame game state
pub struct State<W,Rng>
	where W: World
{
	///Data of the game state
	pub data: Data<W>,

	///Random number generator mappings.
	pub rngs: data::Mappings<Rng>,

	///Function that maps a shape's cell to the world's cell
	pub imprint_cell: fn(&RotatedShape) -> <W as Grid>::Cell,

	///Function that returns the origin position of a player based on shape and world
	pub respawn_pos: fn(&RotatedShape,&W) -> grid::Pos
}

impl<W,Rng> State<W,Rng>
	where W: World
{
	///A simple constructor
	pub fn new(
		rng: Rng,
		imprint_cell: fn(&RotatedShape) -> <W as Grid>::Cell,//TODO: ?? Shape to Grid?
		respawn_pos : fn(&RotatedShape,&W) -> grid::Pos
	) -> Self{State{
		data        : Data::new(),
		rngs        : data::Mappings::new(rng),
		imprint_cell: imprint_cell,
		respawn_pos : respawn_pos,
	}}

	///Updates the game state
	pub fn update<EL>(&mut self, args: &UpdateArgs,event_listener: &mut EL)
		where W: World,
		      <W as Grid>::Cell: Cell,
		      Rng: rand::Rng,
		      EL: FnMut(Event<PlayerId,WorldId>)
	{
		//Players
		'player_loop: for (player_id,player) in self.data.players.iter_mut(){
			let player_id = player_id    as PlayerId;
			let world_id  = player.world as WorldId;

			if let Some(&mut(ref mut world,ref mut paused)) = self.data.worlds.get_mut(player.world as usize){
				if !*paused{
					//Add the time since the last update to the time counts
					player.gravityfall_time_count -= args.dt;

					//Gravity: If the time count is greater than the shape move frequency, then repeat until it is smaller
					while player.gravityfall_time_count <= 0.0{
						//Add one step of frequency
						player.gravityfall_time_count += player.settings.gravityfall_frequency;

						//If able to move (no collision below)
						if move_player(player,world,grid::Pos{x: 0,y: 1}){
							event_listener(Event::PlayerMoved{
								player: player_id,
								world: world_id,
								old: player.pos,
								new: player.pos,
								cause: event::MovementCause::Gravity,
							});
						}else{
							//Imprint the current shape onto the world
							world.imprint_shape(&player.shape,player.pos,&self.imprint_cell);

							//Handles the filled rows (Optimization: Only the rows the imprinted shape occupies needs to be checked)
							let min_y = cmp::max(0,player.pos.y) as grid::SizeAxis;
							let max_y = cmp::min(min_y + player.shape.height(),world.height());
							let full_rows = if min_y!=max_y{
								world.handle_full_rows(min_y .. max_y)
							}else{
								0
							};

							event_listener(Event::WorldImprintedShape{
								world: world_id,
								shape: (player.shape,player.pos),
								full_rows: full_rows,
								cause: event::ShapeImprintCause::PlayerInflicted(player_id),
							});

							//Respawn player and check for collision at spawn position
							let shape = player.next_shape(<Shape as Rand>::rand(self.rngs.player_get_mut(world_id,player_id)));
							if !respawn_player((player_id,player),(world_id,world),shape,self.respawn_pos,event_listener){
								*paused = true;
							}
						}
					}
				}
			}
		}
	}

	///Adds a player to the specified world and with the specified player settings
	///Returns the new player id
	pub fn add_player<EL>(&mut self,world_id: WorldId,settings: player::Settings,event_listener: &mut EL) -> Option<PlayerId>
		where W: World,
		      Rng: rand::Rng,
		      EL: FnMut(Event<PlayerId,WorldId>)
	{
		//If the world exists
		if let Some(&mut(ref mut world,_)) = self.data.worlds.get_mut(world_id as usize){
			//Id is incremental
			let new_id = self.data.players.len();
			//Use a random shape with a random rotation
			let shape = RotatedShape::new(<Shape as rand::Rand>::rand(self.rngs.player_get_mut(world_id,new_id as PlayerId)));

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

			event_listener(Event::PlayerAdded{
				player: new_id as PlayerId,
				world: world_id,
			});

			Some(new_id as PlayerId)
		}else{
			None
		}
	}

	///Resets the specified world, respawning all players and resetting time counts
	pub fn reset_world<EL>(&mut self,world_id: WorldId,event_listener: &mut EL)
		where W: World,
		      <W as Grid>::Cell: Cell,
		      Rng: rand::Rng,
		      EL: FnMut(Event<PlayerId,WorldId>)
	{
		if let Some(&mut(ref mut world,ref mut paused)) = self.data.worlds.get_mut(world_id as usize){
			//Clear world
			world.clear();
			*paused = false;

			//Reset all players in the world
			for (player_id,player) in self.data.players.iter_mut().filter(|&(_,ref player)| player.world == world_id){
				//Respawns player with a new random shape
				respawn_player(
					(player_id as PlayerId,player),
					(world_id,world),
					player.next_shape(<Shape as Rand>::rand(self.rngs.player_get_mut(world_id,player_id as PlayerId))),
					self.respawn_pos,
					event_listener
				);
				//Resets the gravity trigger time counter
				player.gravityfall_time_count = player.settings.gravityfall_frequency;
			}
		};
	}
}

///Moves player if there are no collisions at the new position.
///Returns whether the movement was successful or not due to collisions.
pub fn move_player<W>(player: &mut Player,world: &W,delta: grid::Pos) -> bool
	where W: World
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
pub fn resolve_transformed_player<W>(player: &mut Player,shape: RotatedShape,world: &W) -> bool
	where W: World
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
pub fn respawn_player<W,EL>((player_id,player): (PlayerId,&mut Player),(world_id,world): (WorldId,&W),new_shape: Shape,respawn_pos: fn(&RotatedShape,&W) -> grid::Pos,event_listener: &mut EL) -> bool
	where W: World,
	      EL: FnMut(Event<PlayerId,WorldId>)
{
	//Select a new shape at random, setting its position to the starting position
	let pos = respawn_pos(&player.shape,world);

	event_listener(Event::PlayerChangedShape{
		player: player_id,
		world: world_id,
		shape: new_shape,
		pos: pos,
		cause: event::ShapeChangeCause::NewAfterImprint,
	});

	player.shape = RotatedShape::new(new_shape);
	player.pos = pos;

	//Updates the shadow position
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
///The position of the projected shape on the ground
pub fn fastfallen_shape_pos<W>(shape: &RotatedShape,world: &W,shape_pos: grid::Pos) -> grid::Pos
	where W: World
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
