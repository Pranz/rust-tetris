pub mod tetrimino;

use data::gamestate::{GameState,MapPosAxis,MapSizeAxis};

pub struct State{
	shape: tetrimino::Shape,
	rotation: u8
}
