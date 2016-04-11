use collections::borrow::Cow;

use ::data::grid;
use ::data::shapes::tetromino::{Shape,RotatedShape,Rotation};
use ::game::data::Input;

///Events which can occur ingame.
///These should get signaled by the game state and listened to by a event listener.
#[derive(Clone,Debug,Serialize,Deserialize)]
pub enum Event<P,W>{//TODO: Merge with server packets in online multiplayer if possible and practical?
	PlayerAdded{
		player: P,
		world: W
	},
	PlayerRemoved{
		player: P,
		world: W
	},
	//PlayerWorldMoved(P,W,W),
	//PlayerRotatedCollide(P,W),
	//PlayerMovedCollide(P,W,grid::PosAxis,grid::PosAxis),
	PlayerRotated{//TODO: Implement and check if all the current ones are implemented in all cases
		player: P,
		world: W,
		old: Rotation,
		new: Rotation,
		cause: RotationCause,
	},
	PlayerMoved{
		player: P,
		world: W,
		old: grid::Pos,
		new: grid::Pos,
		cause: MovementCause,
	},
	PlayerChangedShape{
		player: P,
		world: W,
		shape: Shape,
		pos: grid::Pos,
		cause: ShapeChangeCause,
	},
	WorldImprintedShape{
		world: W,
		shape: (RotatedShape,grid::Pos),
		full_rows: grid::SizeAxis,
		cause: ShapeImprintCause<P>,
	},
	//WorldAdded(W),
	//WorldUpdated(W),
	//WorldRemoved(W),
}

#[derive(Clone,Debug,Serialize,Deserialize)]
pub enum MovementCause{
	Gravity,
	Input(Input),
	Desync,
	Other(Option<Cow<'static,str>>)
}

#[derive(Clone,Debug,Serialize,Deserialize)]
pub enum RotationCause{
	Input(Input),
	Desync,
	Other(Option<Cow<'static,str>>)
}

#[derive(Clone,Debug,Serialize,Deserialize)]
pub enum ShapeChangeCause{
	NewAfterImprint,
	Desync,
	Other(Option<Cow<'static,str>>)
}

#[derive(Clone,Debug,Serialize,Deserialize)]
pub enum ShapeImprintCause<P>{
	PlayerInflicted(P),
	Desync,
	Other(Option<Cow<'static,str>>)
}
