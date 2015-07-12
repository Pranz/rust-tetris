use super::super::gamestate;
use super::grid;
use super::shapes::tetrimino::RotatedShape;

pub struct Player{
    pub pos            : grid::Pos,
    pub shape          : RotatedShape,
    pub map            : gamestate::MapId,
	pub move_time_count: f64, //Unit: seconds
	pub points         : u32,
    pub settings       : Settings,
}

pub struct Settings{
	pub move_frequency : f64, //Unit: seconds/block
}
