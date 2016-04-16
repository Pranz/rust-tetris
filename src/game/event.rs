use collections::borrow::Cow;

use ::data::grid;
use ::data::shapes::tetromino::{Shape,RotatedShape,Rotation};
use ::game::data::Input;

//TODO: Document when the events triggers. If one triggers before or after mutation of the structure

///Events which can occur ingame.
///These should get signaled by the game state and listened to by a event listener.
#[derive(Clone,Debug,Serialize,Deserialize)]
pub enum Event<P,W>{//TODO: Merge with server packets in online multiplayer if possible and practical?
	PlayerAdded{
		player: P,
	},
	PlayerRemoved{//TODO: Implement
		player: P,
	},
	PlayerMovedWorld{//TODO: Implement
		player: P,
		old: W,
		new: W
	},
	PlayerCollidedOnRotation{//TODO: Implement
		player: P,
		current: Rotation,
		target: Rotation,
		cause: RotationCause,
	},
	PlayerCollidedOnMovement{//TODO: Implement
		player: P,
		current: grid::PosAxis,
		target: grid::PosAxis,
		cause: RotationCause,
	},
	PlayerRotated{//TODO: Implement
		player: P,
		old: Rotation,
		new: Rotation,
		cause: RotationCause,
	},
	PlayerMoved{
		player: P,
		old: grid::Pos,
		new: grid::Pos,
		cause: MovementCause,
	},
	PlayerChangedShape{
		player: P,
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
	WorldAdded{//TODO: Implement
		world: W,
	},
	WorldUpdated{//TODO: Implement
		world: W,
	},
	WorldRemoved{//TODO: Implement
		world: W,
	},
	WorldPaused{//TODO: Implement
		world: W,
	},
	WorldUnpaused{//TODO: Implement
		world: W,
	},
	GamePaused,//TODO: Implement
	GameUnpaused,//TODO: Implement
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
