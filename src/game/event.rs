use collections::borrow::Cow;

use ::data::grid;
use ::data::shapes::tetromino::{Shape,RotatedShape};

///Events which can occur ingame.
///These should get signaled by the game state and listened to by a event listener.
#[derive(Clone,Debug,Serialize,Deserialize)]
pub enum Event<Player,World>{//TODO: Move to ::data:: or keep it here? Also, merge with server packets in online multiplayer?
	//WorldCreate(World),
	//WorldUpdate(World),
	//WorldEnd(World),
	PlayerAdd{
		player: Player,
		world: World
	},
	//PlayerRemove(PlayerId,World),
	//PlayerWorldMove(PlayerId,World,World),
	//PlayerRotate(PlayerId),
	//PlayerRotateCollide(PlayerId,World),
	//PlayerMoveCollide(PlayerId,World,grid::PosAxis,grid::PosAxis),
	PlayerMove{
		player: Player,
		world: World,
		old: grid::Pos,
		new: grid::Pos,
		cause: Cow<'static,str>,
	},
	WorldImprintShape{
		world: World,
		shape: (RotatedShape,grid::Pos),
		full_rows: grid::SizeAxis,
		cause: Option<Player>,
	},
	PlayerChangeShape{
		player: Player,
		world: World,
		shape: Shape,
		pos: grid::Pos
	},
}

pub mod move_cause{
	pub const GRAVITY: &'static str = "gravity";//TODO: Replace with an enum
}
