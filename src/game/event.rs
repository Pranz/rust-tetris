use collections::borrow::Cow;

use ::data::grid;
use ::data::shapes::tetromino::{Shape,RotatedShape};

///Events which can occur ingame.
///These should get signaled by the game state and listened to by a event listener.
#[derive(Clone,Debug,Serialize,Deserialize)]
pub enum Event<P,W>{//TODO: Move to ::data:: or keep it here? Also, merge with server packets in online multiplayer?
	//WorldAdded(W),
	//WorldUpdated(W),
	//WorldRemoved(W),
	PlayerAdded{
		player: P,
		world: W
	},
	PlayerRemoved{
		player: P,
		world: W
	},
	//PlayerWorldMoved(P,W,W),
	//PlayerRotated(P),
	//PlayerRotatedCollide(P,W),
	//PlayerMovedCollide(P,W,grid::PosAxis,grid::PosAxis),
	PlayerMoved{
		player: P,
		world: W,
		old: grid::Pos,
		new: grid::Pos,
		cause: Cow<'static,str>,
	},
	WorldImprintedShape{
		world: W,
		shape: (RotatedShape,grid::Pos),
		full_rows: grid::SizeAxis,
		cause: Option<P>,
	},
	PlayerChangedShape{
		player: P,
		world: W,
		shape: Shape,
		pos: grid::Pos
	},
}

pub mod move_cause{
	pub const GRAVITY: &'static str = "gravity";//TODO: Replace with an enum
}
