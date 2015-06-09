use core::default::Default;
use piston::event;
use rand::{self,Rand};

use super::map::{self,Map};
use super::shapes::tetrimino::{data,Shape, BlockVariant};

pub struct GameState<Rng>{
	pub map                 : Map,
    pub block_move_frequency: u16,
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
        	block_move_frequency: 60,//Unit: frames/block
        	frames_passed       : 0,
        	block               : BlockVariant::new(&mut rng),
        	block_x             : 0,
        	block_y             : 0,
        	rng                 : rng,
    	}
	}

    pub fn update(&mut self, _: &event::UpdateArgs){
        self.frames_passed += 1;
        if self.frames_passed == self.block_move_frequency {
            self.frames_passed = 0;
            if self.map.block_intersects(&self.block, self.block_x as map::PosAxis, self.block_y as map::PosAxis + 1) {
                let (x, y) = (self.block_x,self.block_y);
                self.map.imprint_block(&self.block, x, y);

                self.block = BlockVariant::new(&mut self.rng);
                self.block_x = 2;//TODO: Top middle of map
                self.block_y = 0;
                if self.map.block_intersects(&self.block, self.block_x, self.block_y) {
                    self.map.clear();
                }
            }
            else {
                self.block_y += 1;
            }
        }
    }

    pub fn move_block(&mut self, dx: map::PosAxis, dy: map::PosAxis) -> bool{
        if self.map.block_intersects(&self.block, self.block_x + dx, self.block_y + dy){
            false
        }else{
            self.block_x += dx;
            self.block_y += dy;
            true
        }
    }
}
