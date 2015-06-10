use core::default::Default;
use piston::event;
use rand::{self,Rand};

use super::map::{self,Map, Cell};
use super::shapes::tetrimino::{Shape,BlockVariant};

pub struct GameState<Rng>{
	pub map                 : Map,
    pub block_move_frequency: u16,//Unit: frames/block
    pub frames_passed       : u16,
    pub block               : BlockVariant,
    pub block_x             : map::PosAxis,
    pub block_y             : map::PosAxis,
    pub rng                 : Rng
}

impl<Rng: rand::Rng> GameState<Rng>{
    pub fn new(mut rng: Rng) -> Self {
		GameState{
	    	map                 : Map::default(),
        	block_move_frequency: 60,
        	frames_passed       : 0,
        	block               : BlockVariant::new(<Shape as Rand>::rand(&mut rng),0),
        	block_x             : 0,//TODO: Maybe move some of these fields to a Player struct? (Multiplayer preparations)
        	block_y             : 0,
        	rng                 : rng,
    	}
	}

    pub fn update(&mut self, _: &event::UpdateArgs){
        self.frames_passed += 1;
        if self.frames_passed == self.block_move_frequency {
            self.frames_passed = 0;
            if self.map.block_intersects(&self.block, self.block_x as map::PosAxis, self.block_y as map::PosAxis + 1).is_some() {
                self.map.imprint_block(&self.block,self.block_x,self.block_y);

                self.block = BlockVariant::new(<Shape as Rand>::rand(&mut self.rng),0);
                self.block_x = 2;//TODO: Top middle of map
                self.block_y = 0;
                if self.map.block_intersects(&self.block, self.block_x, self.block_y).is_some() {
                    self.map.clear();
                }
            }
            else {
                self.block_y += 1;
            }
        }
    }

    pub fn move_block(&mut self, dx: map::PosAxis, dy: map::PosAxis) -> bool{
        if self.map.block_intersects(&self.block, self.block_x + dx, self.block_y + dy).is_some(){
            false
        }else{
            self.block_x += dx;
            self.block_y += dy;
            true
        }
    }

	pub fn rotate_and_resolve(&mut self) -> bool {
		self.block.next_rotation();
		if let Some((x,y)) = self.map.block_intersects(&self.block, self.block_x, self.block_y) {
			let center_x = self.block_x + 2;
			if x < center_x {
				for i in 1..3 {
					if self.move_block(i, 0) {return true;}
				}
				self.block.previous_rotation();
				return false;
			}
			else {
				for i in 1..3 {
					if self.move_block(-i, 0) {return true;}
				}
				self.block.previous_rotation();
				return false;
			}
		}
	true
	}
}
