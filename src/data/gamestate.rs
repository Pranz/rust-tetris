use core::default::Default;
use piston::event;
use rand::{self,Rand};

use super::map::{self,Map};
use super::shapes::tetrimino::{data,Shape};

pub struct GameState<Rng>{
	map                     : Map,
    pub block_move_frequency: u16,
    pub frames_passed       : u16,
    pub block               : &'static [data::Block],
    pub block_rotation      : u8,
    pub block_x             : map::PosAxis,
    pub block_y             : map::PosAxis,
    pub rng                 : Rng
}

impl<Rng: rand::Rng> GameState<Rng>{
    pub fn new(rng: Rng) -> Self {
        let mut state = GameState {
    	    map: Map::default(),
            block_move_frequency: 60,//Unit: frames/block
            frames_passed       : 0,
            block               : &data::L,
            block_rotation      : 0,
            block_x             : 0,
            block_y             : 0,
            rng                 : rng,
		};
		state
	}

    pub fn update(&mut self, _: &event::UpdateArgs){
        self.frames_passed += 1;
        if self.frames_passed == self.block_move_frequency {
            self.frames_passed = 0;
            if self.map.block_intersects(self.block,self.block_rotation, self.block_x as map::PosAxis, self.block_y as map::PosAxis + 1) {
                let (x, y) = (self.block_x,self.block_y);
                self.map.imprint_block(self.block,self.block_rotation,x,y);

                self.block = Shape::rand(&mut self.rng).data();
                self.block_x = 2;//TODO: Top middle of map
                self.block_y = 0;
                self.block_rotation = 0;//TODO: Randomize
                if self.map.block_intersects(self.block,self.block_rotation, self.block_x, self.block_y) {
                    self.map.clear();
                }
            }
            else {
                self.block_y += 1;
            }
        }
    }

    pub fn next_rotation(&mut self) -> bool{
        self.block_rotation = (self.block_rotation + 1) % self.block.len() as u8;

        true
    }

    pub fn previous_rotation(&mut self) -> bool{
        self.block_rotation = if self.block_rotation == 0{
            self.block.len() as u8
        }else{
            self.block_rotation
        } - 1;

        true
    }

    pub fn move_block(&mut self, dx: map::PosAxis, dy: map::PosAxis) -> bool{
        if self.map.block_intersects(self.block,self.block_rotation,self.block_x + dx, self.block_y + dy){
            false
        }else{
            self.block_x += dx;
            self.block_y += dy;
            true
        }
    }
}
