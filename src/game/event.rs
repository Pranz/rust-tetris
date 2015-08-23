use super::super::data::grid;
use super::super::data::shapes::tetromino::{Shape,RotatedShape};

///Events which can occur ingame.
///These should get signaled by the game state and listened to by a event listener.
#[derive(Copy,Clone,Debug)]
pub enum Event<Player,World>{
	//WorldStart(World),
	//WorldUpdate(World),
	//WorldEnd(World),
	PlayerAdd{
		player: Player,
		world: World
	},
	//PlayerRemove(PlayerId,World),
	//PlayerWorldChange(PlayerId,World,World),
	//PlayerRotate(PlayerId),
	//PlayerRotateCollide(PlayerId,World),
	//PlayerMove(PlayerId,World,grid::PosAxis,grid::PosAxis),
	//PlayerMoveCollide(PlayerId,World,grid::PosAxis,grid::PosAxis),
	PlayerMoveGravity{
		player: Player,
		world: World,
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
