use super::map::{self,Map};
use super::shapes::tetrimino::{Shape,BlockVariant};

pub struct Player{
	pub x             : map::PosAxis,
	pub y             : map::PosAxis,
	pub block         : BlockVariant,
	pub move_frequency: f64,//Unit: seconds/block
}

impl Player{
	///Moves the current block if there are no collisions at the new position.
	///Returns whether the movement was successful due to collisions.
	pub fn move_block(&mut self, dx: map::PosAxis, dy: map::PosAxis) -> bool{
		//Collision check
		if self.map.block_intersects(&self.block, self.x + dx, self.y + dy).is_some(){
			//Collided => cannot move
			false
		}else{
			//No collision, able to move and does so
			self.x += dx;
			self.y += dy;
			true
		}
	}
}
