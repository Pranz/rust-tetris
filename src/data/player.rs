use fixed_circular_buffer::CircularBuffer;

use super::super::gamestate;
use super::grid;
use super::shapes::tetrimino::{RotatedShape,Shape};

///Player state data
#[derive(Clone,PartialEq)]
pub struct Player{
    pub pos                   : grid::Pos,
    pub shadow_pos            : Option<grid::Pos>,
    pub shapes_lookahead      : Option<CircularBuffer<Shape>>,
    pub shape                 : RotatedShape,//TODO: Consider only having a circular buffer with shapes, and a separate rotation field. Then the circular buffer won't need to be wrapped in a Option because it is guaranteed to be non-empty
    pub map                   : gamestate::MapId,
    pub points                : u32,
    pub gravityfall_time_count: f64,//Unit: seconds
    pub settings              : Settings,
}

///Player settings
#[derive(Copy,Clone,PartialEq)]
pub struct Settings{
	pub gravityfall_frequency: f64,//Unit: seconds/block
    pub fastfall_shadow      : bool,
}
