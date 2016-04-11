///Data related to players

use fixed_circular_buffer::CircularBuffer;

use ::data::grid;
use ::data::shapes::tetromino::{RotatedShape,Shape};
use ::game;

///Player state data
#[derive(Clone,PartialEq)]
pub struct Player{
	pub pos                   : grid::Pos,
	pub shadow_pos            : Option<grid::Pos>,
	pub shapes_lookahead      : Option<CircularBuffer<Shape>>,
	pub shape                 : RotatedShape,//TODO: Consider only having a circular buffer with shapes, and a separate rotation field. Then the circular buffer won't need to be wrapped in a Option because it is guaranteed to be non-empty
	pub world                 : game::data::WorldId,
	pub points                : u32,
	pub gravityfall_time_count: f64,//Unit: seconds
	pub settings              : Settings,
}

impl Player{
	///Returns the next shape from the queue while queuing the given shape
	pub fn next_shape(&mut self,queued_shape: Shape) -> Shape{
		if let Some(ref mut shapes_lookahead) = self.shapes_lookahead{
			shapes_lookahead.queue(queued_shape)
		}else{
			queued_shape
		}
	}
}

///Player settings
#[derive(Copy,Clone,Debug,PartialEq,Serialize,Deserialize)]
pub struct Settings{
	pub gravityfall_frequency: f64,//Unit: seconds/block
	pub fastfall_shadow      : bool,
}
