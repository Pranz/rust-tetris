use piston::event::UpdateArgs;
use rand::{self,Rand};

use super::map::{self,Map};
use super::shapes::tetrimino::{data,Shape};

pub struct GameState {
	map                  : Map,
    pub frames_until_move: u16,
    pub frames_passed    : u16,
    pub block            : &'static [data::Block],
    pub block_rotation   : u8,
    pub block_x          : map::PosAxis,
    pub block_y          : map::PosAxis,
}

impl GameState {
    pub fn new() -> Self {
        let mut state = GameState {
    	    map: Map::default(),
            frames_until_move: 60,
            frames_passed    : 0,
            block            : &data::L,
            block_rotation   : 0,
            block_x          : 0,
            block_y          : 0,
		};
		state
	}

    pub fn update(&mut self, _: &UpdateArgs){
        self.frames_passed += 1;
        if self.frames_passed == self.frames_until_move {
            self.frames_passed = 0;
            if self.map.block_intersects(self.block,self.block_rotation, self.block_x as map::PosAxis, self.block_y as map::PosAxis + 1) {
                let (x, y) = (self.block_x,self.block_y);
                self.map.imprint_block(self,x,y);

                self.block = Shape::rand(&mut rand::StdRng::new().unwrap()).data();//TODO: Store StdRng::new
                self.block_x = 2;//TODO: Top middle of map
                self.block_y = 0;
                self.block_rotation = 0;//TODO: Randomize
                if self.map.block_intersects(self.block,self.block_rotation, self.block_x, self.block_y) {
                    self.clear();
                }
            }
            else {
                self.block_y += 1;
            }
        }
    }

    pub fn next_rotation(&mut self) {
        self.block_rotation = (self.block_rotation + 1) % self.block.len() as u8;
    }

    pub fn previous_rotation(&mut self) {
        self.block_rotation = if self.block_rotation == 0{
            self.block.len() as u8
        }else{
            self.block_rotation
        } - 1;
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

    //pub fn move_row

    //check and resolve any full rows, starting to check at the specified y-position and then
    //upward.
    pub fn handle_full_rows(&mut self, lowest_y : map::SizeAxis) {
        let mut terminated_rows : map::SizeAxis = 0;
        for i in 0..4  {
            let lowest_y = lowest_y + i as map::SizeAxis - terminated_rows;
            if (0..map::WIDTH).all(|x| unsafe{self.pos(x as usize,lowest_y as usize)}) {
                terminated_rows += 1;
                for j in 0..lowest_y {
                    self.map[(lowest_y - j) as usize] = self.map[(lowest_y - j - 1) as usize];
                }
                self.map[0] = [false; map::WIDTH as usize];
            }
        }
    }
}
