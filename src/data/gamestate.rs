use core::cmp;
use piston::event;
use rand::{self,Rand};

use super::map;
use super::map::Map as MapTrait;
use super::map::dynamic_map::Map;
use super::player::Player;
use super::shapes::tetrimino::{BLOCK_COUNT,Shape,ShapeVariant};

pub struct GameState<Rng>{
    pub map   : Map<map::cell::ShapeCell>,
    pub player: Player,
    pub rng   : Rng,
    pub paused: bool,
}

impl<Rng: rand::Rng> GameState<Rng>{
    pub fn new(mut rng: Rng) -> Self {
        GameState{
            map: Map::new(5,25),
            player: Player{
                x              : 0,
                y              : 0,
                shape          : ShapeVariant::new(<Shape as Rand>::rand(&mut rng),0),
                move_frequency : 1.0,
                move_time_count: 0.0,
            },
            rng   : rng,
            paused: false,
        }
    }

    pub fn update(&mut self, args: &event::UpdateArgs){if !self.paused{
        //Add the time since the last update to the time count
        self.player.move_time_count += args.dt;

        //If the time count is bigger than the shape move frequency, then repeat until it is smaller
        while self.player.move_time_count >= self.player.move_frequency{
            //Subtract one step of frequency
            self.player.move_time_count -= self.player.move_frequency;

            //If there are a collision below
            if self.map.shape_intersects(&self.player.shape, self.player.x as map::PosAxis, self.player.y as map::PosAxis + 1).is_some() {
                //Imprint the current shape onto the map
                self.map.imprint_shape(&self.player.shape,self.player.x,self.player.y,|variant| map::cell::ShapeCell(Some(variant.shape())));

                //Handles the filled rows
                let map_height = self.map.height();
                self.map.handle_full_rows(cmp::max(0,self.player.y) as map::SizeAxis .. cmp::min(self.player.y as map::SizeAxis + BLOCK_COUNT,map_height));

                //Select a new shape at random, setting its position to the starting position
                self.player.shape = ShapeVariant::new(<Shape as Rand>::rand(&mut self.rng),0);
                self.player.x = self.map.width() as map::PosAxis/2 - BLOCK_COUNT as map::PosAxis/2;//TODO: Top middle of map
                self.player.y = 0;
                //If the new shape at the starting position also collides with another shape
                if self.map.shape_intersects(&self.player.shape, self.player.x, self.player.y).is_some() {
                    //Reset the map
                    self.map.clear();
                    self.player.move_time_count = 0.0;
                }
            }
            else{
                //Move the current shape downwards
                self.player.y += 1;
            }
        }
    }}

    ///Moves the current shape if there are no collisions at the new position.
    ///Returns whether the movement was successful due to collisions.
    pub fn move_shape(&mut self, dx: map::PosAxis, dy: map::PosAxis) -> bool{
        //Collision check
        if self.map.shape_intersects(&self.player.shape, self.player.x + dx, self.player.y + dy).is_some(){
            //Collided => cannot move
            false
        }else{
            //No collision, able to move and does so
            self.player.x += dx;
            self.player.y += dy;
            true
        }
    }

    ///Try to rotate (forwards). If this results in a collision, try to resolve this collision by
    ///moving in the x axis. If the collision cannot resolve, amend the rotation and return false,
    ///otherwise return true.
    pub fn rotate_and_resolve(&mut self) -> bool {
        self.player.shape.next_rotation();
        if let Some((x,_)) = self.map.shape_intersects(&self.player.shape, self.player.x, self.player.y) {
            let center_x = self.player.x + 2;//TODO: Magic constants everywhere
            let sign = if x < center_x {1} else {-1};
            for i in 1..3 {
                if self.move_shape(i * sign, 0) {return true;}
            }
            self.player.shape.previous_rotation();
            return false;
        }
    true
    }
}
