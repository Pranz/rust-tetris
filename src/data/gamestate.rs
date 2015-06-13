use core::default::Default;
use piston::event;
use rand::{self,Rand};

use super::map::{self,Map};
use super::shapes::tetrimino::{Shape,BlockVariant};

pub struct GameState<Rng>{
    pub map                 : map::dynamic_map::Map,
    pub block_move_frequency: f64,//Unit: seconds/block
    pub time_count          : f64,
    pub block               : BlockVariant,
    pub block_x             : map::PosAxis,
    pub block_y             : map::PosAxis,
    pub rng                 : Rng
}

impl<Rng: rand::Rng> GameState<Rng>{
    pub fn new(mut rng: Rng) -> Self {
        GameState{
            map                 : map::dynamic_map::Map::new(30, 25),
            block_move_frequency: 1.0,
            time_count          : 0.0,
            block               : BlockVariant::new(<Shape as Rand>::rand(&mut rng),0),
            block_x             : 0,//TODO: Maybe move some of these fields to a Player struct? (Multiplayer preparations)
            block_y             : 0,
            rng                 : rng,
        }
    }

    pub fn update(&mut self, args: &event::UpdateArgs){
        //Add the time since the last update to the time count
        self.time_count += args.dt;

        //If the time count is bigger than the block move frequency, then repeat until it is smaller
        while self.time_count >= self.block_move_frequency{
            //Subtract one step of frequency
            self.time_count -= self.block_move_frequency;

            //If there are a collision below
            if self.map.block_intersects(&self.block, self.block_x as map::PosAxis, self.block_y as map::PosAxis + 1).is_some() {
                //Imprint the current block onto the map
                self.map.imprint_block(&self.block,self.block_x,self.block_y,|_| 1);

                //Handles the filled rows
                self.map.handle_full_rows(self.block_y as u8 + 4);//TODO: 4? Magic constant

                //Select a new block at random, setting its position to the starting position
                self.block = BlockVariant::new(<Shape as Rand>::rand(&mut self.rng),0);
                self.block_x = 2;//TODO: Top middle of map
                self.block_y = 0;
                //If the new block at the starting position also collides with another block
                if self.map.block_intersects(&self.block, self.block_x, self.block_y).is_some() {
                    //Reset the map
                    self.map.clear();
                }
            }
            else{
                //Move the current block downwards
                self.block_y += 1;
            }
        }
    }

    ///Moves the current block if there are no collisions at the new position.
    ///Returns whether the movement was successful due to collisions.
    pub fn move_block(&mut self, dx: map::PosAxis, dy: map::PosAxis) -> bool{
        //Collision check
        if self.map.block_intersects(&self.block, self.block_x + dx, self.block_y + dy).is_some(){
            //Collided => cannot move
            false
        }else{
            //No collision, able to move and does so
            self.block_x += dx;
            self.block_y += dy;
            true
        }
    }

    ///Try to rotate (forwards). If this results in a collision, try to resolve this collision by
    ///moving in the x axis. If the collision cannot resolve, amend the rotation and return false,
    ///otherwise return true.
    pub fn rotate_and_resolve(&mut self) -> bool {
        self.block.next_rotation();
        if let Some((x,_)) = self.map.block_intersects(&self.block, self.block_x, self.block_y) {
            let center_x = self.block_x + 2;//TODO: Magic constants everywhere
            let sign = if x < center_x {1} else {-1};
            for i in 1..3 {
                if self.move_block(i * sign, 0) {return true;}
            }
            self.block.previous_rotation();
            return false;
        }
    true
    }
}
