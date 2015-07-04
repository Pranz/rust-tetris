use collections::vec_map::VecMap;
use core::cmp;
use piston::event;
use rand::{self,Rand};

use super::super::ai::fill_one::Ai;
use super::grid::Grid;
use super::map;
use super::map::Map as MapTrait;
use super::player::Player;
use super::shapes::tetrimino::{Shape,ShapeVariant};

pub type MapId    = u8;
pub type PlayerId = u8;

pub enum Event{
    //MapStart(MapId),
    //MapUpdate(MapId),
    //MapEnd(MapId),
    //PlayerAdd(PlayerId,MapId),
    //PlayerRemove(PlayerId,MapId),
    //PlayerMapChange(PlayerId,MapId,MapId),
    //PlayerRotate(PlayerId),
    //PlayerRotateCollide(PlayerId,MapId),
    //PlayerMove(PlayerId,MapId,map::PosAxis,map::PosAxis),
    //PlayerMoveCollide(PlayerId,MapId,map::PosAxis,map::PosAxis),
    PlayerMoveGravity,//(PlayerId,MapId,map::PosAxis),
    PlayerImprint,//(PlayerId,MapId,map::PosAxis,map::PosAxis),
    PlayerNewShape,//(PlayerId,MapId,map::PosAxis,map::PosAxis),
}

pub struct GameState<Map,Rng>{//TODO: Move out of the `data` module
    pub maps   : VecMap<Map>,
    pub players: VecMap<Player>,
    pub ai     : VecMap<Ai>,
    pub rng    : Rng,
    pub paused : bool
}

impl<Map,Rng: rand::Rng> GameState<Map,Rng>{
    pub fn new(rng: Rng) -> Self{
        GameState{
            maps   : VecMap::new(),
            players: VecMap::new(),
            ai     : VecMap::new(),
            rng    : rng,
            paused : false,
        }
    }

    pub fn update(&mut self, args: &event::UpdateArgs)
        where Map: MapTrait<Cell = map::cell::ShapeCell>
    {if !self.paused{
        //Players
        for (player_id,player) in self.players.iter_mut(){
            if let Some(map) = self.maps.get_mut(&(player.map as usize)){
                //AI, if any
                let mut ai = self.ai.get_mut(&(player_id as usize));

                //Add the time since the last update to the time count
                player.move_time_count += args.dt;

                //If the time count is bigger than the shape move frequency, then repeat until it is smaller
                while player.move_time_count >= player.move_frequency{
                    //Subtract one step of frequency
                    player.move_time_count -= player.move_frequency;

                    //If there are a collision below
                    match map.shape_intersects(&player.shape, player.x, player.y + 1){
                        map::CellIntersection::Imprint(_,_) |
                        map::CellIntersection::OutOfBounds(_,_) => {
                            //Imprint the current shape onto the map
                            map.imprint_shape(&player.shape,player.x,player.y,|variant| map::cell::ShapeCell(Some(variant.shape())));

                            //Handles the filled rows
                            let min_y = cmp::max(0,player.y) as map::SizeAxis;
                            let max_y = cmp::min(min_y + player.shape.height(),map.height());
                            if min_y!=max_y{
                                map.handle_full_rows(min_y .. max_y);
                            }

                            //Select a new shape at random, setting its position to the starting position
                            player.shape = ShapeVariant::new(<Shape as Rand>::rand(&mut self.rng),0);
                            player.x = map.width() as map::PosAxis/2 - player.shape.center_x() as map::PosAxis;
                            player.y = 0;//TODO: Spawn above optionally: -(player.shape.height() as map::PosAxis);

                            //If the new shape at the starting position also collides with another shape
                            if let map::CellIntersection::Imprint(_,_) = map.shape_intersects(&player.shape, player.x, player.y){//TODO: This won't work if the block is spawning outside of the map. Should check when imprint_block is executed and see if any of those positions is out of range
                                //Reset the map
                                map.clear();
                                player.move_time_count = 0.0;
                            }

                            if let Some(ref mut ai) = ai{
                                ai.event(Event::PlayerImprint,player,map);
                                ai.event(Event::PlayerNewShape,player,map);
                            }
                        },
                        map::CellIntersection::None =>{
                            //Move the current shape downwards
                            player.y += 1;
                            if let Some(ref mut ai) = ai{ai.event(Event::PlayerMoveGravity,player,map);}
                        }
                    }
                }

                //AI update
                if let Some(ref mut ai) = ai{ai.update(args,player,map);}
            }
        }
    }}

    pub fn with_player<F: FnOnce(&mut Player)>(&mut self,player_id: PlayerId,f: F){
        if let Some(player) = self.players.get_mut(&(player_id as usize)){
            f(player);
        }
    }

    pub fn with_player_map<F: FnOnce(&mut Player,&mut Map)>(&mut self,player_id: PlayerId,f: F){
        if let Some(player) = self.players.get_mut(&(player_id as usize)){
            if let Some(map) = self.maps.get_mut(&(player.map as usize)){
                f(player,map);
            }
        }
    }
}

///Moves player if there are no collisions at the new position.
///Returns whether the movement was successful or not due to collisions.
pub fn move_player<M: MapTrait>(player: &mut Player,map: &M,dx: map::PosAxis, dy: map::PosAxis) -> bool{
    //Collision check
    match map.shape_intersects(&player.shape,player.x + dx,player.y + dy){
        //Collided => cannot move
        map::CellIntersection::Imprint(_,_) |
        map::CellIntersection::OutOfBounds(_,_) => false,

        //No collision, able to move and does so
        map::CellIntersection::None => {
            player.x += dx;
            player.y += dy;
            true
        }
    }
}

///Try to rotate (forwards). If this results in a collision, try to resolve this collision by
///moving in the x axis. If the collision cannot resolve, amend the rotation and return false,
///otherwise return true.
pub fn rotate_next_and_resolve_player<M: MapTrait>(player: &mut Player,map: &M) -> bool{
    player.shape.next_rotation();
    match map.shape_intersects(&player.shape,player.x,player.y){
        map::CellIntersection::Imprint(x,_) |
        map::CellIntersection::OutOfBounds(x,_) => {
            let center_x = player.x + 2;//TODO: Magic constants everywhere
            let sign = if x < center_x {1} else {-1};
            for i in 1..3 {
                if move_player(player,map,i * sign, 0){return true;}
            }
            player.shape.previous_rotation();

            false
        },
        _ => true
    }
}

///Try to rotate (backwards). If this results in a collision, try to resolve this collision by
///moving in the x axis. If the collision cannot resolve, amend the rotation and return false,
///otherwise return true.
pub fn rotate_previous_and_resolve_player<M: MapTrait>(player: &mut Player,map: &M) -> bool{
    player.shape.previous_rotation();
    match map.shape_intersects(&player.shape,player.x,player.y){
        map::CellIntersection::Imprint(x,_) |
        map::CellIntersection::OutOfBounds(x,_) => {
            let center_x = player.x + 2;//TODO: Magic constants everywhere
            let sign = if x < center_x {1} else {-1};
            for i in 1..3 {
                if move_player(player,map,i * sign, 0){return true;}
            }
            player.shape.next_rotation();

            false
        },
        _ => true
    }
}
