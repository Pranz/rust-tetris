use collections::vec_map::VecMap;
use core::{cmp,mem};
use piston::event;
use rand::{self,Rand};

use super::controller::Controller;
use super::data::cell;
use super::data::grid::{self,Grid};
use super::data::map;
use super::data::map::Map as MapTrait;
use super::data::player::{self,Player};
use super::data::shapes::tetrimino::{Shape,RotatedShape};

pub type MapId    = u8;
pub type PlayerId = u8;

#[derive(Copy,Clone,Eq,PartialEq)]
pub enum Event{
    //MapStart(MapId),
    //MapUpdate(MapId),
    //MapEnd(MapId),
    //PlayerAdd(PlayerId,MapId),
    //PlayerRemove(PlayerId,MapId),
    //PlayerMapChange(PlayerId,MapId,MapId),
    //PlayerRotate(PlayerId),
    //PlayerRotateCollide(PlayerId,MapId),
    //PlayerMove(PlayerId,MapId,grid::PosAxis,grid::PosAxis),
    //PlayerMoveCollide(PlayerId,MapId,grid::PosAxis,grid::PosAxis),
    PlayerRowsClear{n: grid::SizeAxis},
    PlayerMoveGravity,//(PlayerId,MapId,grid::PosAxis),
    PlayerImprint,//(PlayerId,MapId,grid::PosAxis,grid::PosAxis),
    PlayerNewShape{old: RotatedShape,new: Shape},//(PlayerId,MapId,grid::PosAxis,grid::PosAxis),
}

fn imprint_cell(variant: &RotatedShape) -> cell::ShapeCell{
    cell::ShapeCell(Some(variant.shape()))
}

pub struct GameState<Map,Rng>{
    pub maps       : VecMap<Map>,
    pub players    : VecMap<Player>,
    pub controllers: VecMap<Box<Controller<Map>>>,
    pub rng        : Rng,//TODO: See http://doc.rust-lang.org/rand/src/rand/lib.rs.html#724-726 for getting a seed
    pub paused     : bool
}

impl<Map,Rng: rand::Rng> GameState<Map,Rng>{
    pub fn new(rng: Rng) -> Self{
        GameState{
            maps       : VecMap::new(),
            players    : VecMap::new(),
            controllers: VecMap::new(),
            rng        : rng,
            paused     : false,
        }
    }

    pub fn update(&mut self, args: &event::UpdateArgs)
        where Map: MapTrait<Cell = cell::ShapeCell>
    {if !self.paused{
        //After action
        enum Action{
            None,
            ResetMap(MapId)
        }let mut action = Action::None;

        //Players
        'player_loop: for (player_id,player) in self.players.iter_mut(){
            if let Some(map) = self.maps.get_mut(&(player.map as usize)){
                //AI, if any
                let mut controller = self.controllers.get_mut(&(player_id as usize));

                //Add the time since the last update to the time count
                player.move_time_count += args.dt;

                //If the time count is bigger than the shape move frequency, then repeat until it is smaller
                while player.move_time_count >= player.settings.move_frequency{
                    //Subtract one step of frequency
                    player.move_time_count -= player.settings.move_frequency;

                    //If there are a collision below
                    if move_player(player,map,grid::Pos{x: 0,y: 1}){
                        if let Some(ref mut controller) = controller{controller.event(Event::PlayerMoveGravity,player,map);}
                    }else{
                        //Imprint the current shape onto the map
                        map.imprint_shape(&player.shape,player.pos,&(imprint_cell as fn(&RotatedShape) -> cell::ShapeCell));

                        //Handles the filled rows
                        let min_y = cmp::max(0,player.pos.y) as grid::SizeAxis;
                        let max_y = cmp::min(min_y + player.shape.height(),map.height());
                        if min_y!=max_y{
                            let rows = map.handle_full_rows(min_y .. max_y);
                            if let Some(ref mut controller) = controller{controller.event(Event::PlayerRowsClear{n: rows},player,map);}
                        }

                        //Respawn player and check for collision at spawn position
                        let shape = <Shape as Rand>::rand(&mut self.rng);
                        if !respawn_player(player,map,shape){
                            action = Action::ResetMap(player.map);
                            break 'player_loop;
                        }

                        if let Some(ref mut controller) = controller{controller.event(Event::PlayerNewShape{old: player.shape,new: shape},player,map);}
                        if let Some(ref mut controller) = controller{controller.event(Event::PlayerImprint,player,map);}
                    }
                }

                //AI update
                if let Some(ref mut controller) = controller{controller.update(args,player,map);}
            }
        }

        match action{
            Action::None => (),
            Action::ResetMap(map_id) => self.reset_map(map_id),
        };
    }}

    pub fn with_player<F: FnOnce(&mut Player) -> R,R>(&mut self,player_id: PlayerId,f: F) -> Option<R>{
        if let Some(player) = self.players.get_mut(&(player_id as usize)){
            return Some(f(player))
        }
        None
    }

    pub fn with_player_map<F: FnOnce(&mut Player,&mut Map)-> R,R>(&mut self,player_id: PlayerId,f: F) -> Option<R>{
        if let Some(player) = self.players.get_mut(&(player_id as usize)){
            if let Some(map) = self.maps.get_mut(&(player.map as usize)){
                return Some(f(player,map))
            }
        }
        None
    }

    pub fn with_map<F: FnOnce(&mut Map) -> R,R>(&mut self,map_id: MapId,f: F) -> Option<R>{
        if let Some(map) = self.maps.get_mut(&(map_id as usize)){
            return Some(f(map))
        }
        None
    }

    pub fn with_map_players<F: FnMut(&mut Player)>(&mut self,map_id: MapId,mut f: F){
        for (_,player) in self.players.iter_mut(){
            if player.map == map_id{
                f(player);
            }
        }
    }

    pub fn add_player(&mut self,map_id: MapId,settings: player::Settings) -> Option<PlayerId>
        where Map: MapTrait
    {
        if let Some(map) = self.maps.get_mut(&(map_id as usize)){
            let new_id = self.players.len();
            let shape = RotatedShape::new(<Shape as rand::Rand>::rand(&mut self.rng));

            self.players.insert(new_id,Player{
                pos            : respawn_position(shape,map),
                shadow_pos     : None,
                shape          : shape,
                map            : map_id,
                move_time_count: 0.0,
                points         : 0,
                settings       : settings
            });
            let player = self.players.get_mut(&new_id).unwrap();

            let mut controller = self.controllers.get_mut(&(new_id as usize));
            if let Some(ref mut controller) = controller{controller.event(Event::PlayerNewShape{old: shape,new: shape.shape()},player,map);}

            Some(new_id as PlayerId)
        }else{
            None
        }
    }

    pub fn reset_map(&mut self,map_id: MapId)
        where Map: MapTrait,
              <Map as Grid>::Cell: cell::Cell
    {
        let self2 = unsafe{mem::transmute::<&mut Self,&mut Self>(self)};
        let self3 = unsafe{mem::transmute::<&mut Self,&mut Self>(self)};

        self.with_map(map_id,|map|{//`self.with_map` accesses `self.maps`
            //Clear map
            map.clear();

            self2.with_map_players(map_id,|player|{//`self2.with_map_players` accesses `self.players`
                //Reset all players in the map
                respawn_player(player,map,<Shape as Rand>::rand(&mut self3.rng));//`self3.rng` accesses `self.rng`
                player.move_time_count = 0.0;
            });
        });
    }
}

///Moves player if there are no collisions at the new position.
///Returns whether the movement was successful or not due to collisions.
pub fn move_player<M: MapTrait>(player: &mut Player,map: &M,delta: grid::Pos) -> bool{
    //Collision check
    match map.shape_intersects(&player.shape,grid::Pos{x: player.pos.x + delta.x,y: player.pos.y + delta.y}){
        //Collided => cannot move
        map::CellIntersection::Imprint(_) |
        map::CellIntersection::OutOfBounds(_) => false,

        //No collision, able to move and does so
        map::CellIntersection::None => {
            //Change position
            player.pos.x += delta.x;
            player.pos.y += delta.y;

            //Recalcuate fastfall shadow position when moving horizontally
            if player.settings.fastfall_shadow && delta.x!=0{
                player.shadow_pos = Some(fast_fallen_shape(&player.shape,map,player.pos));
            }

            true
        }
    }
}

///Try to rotate (forwards). If this results in a collision, try to resolve this collision by
///moving in the x axis. If the collision cannot resolve, amend the rotation and return false,
///otherwise return true.
pub fn rotate_anticlockwise_and_resolve_player<M: MapTrait>(player: &mut Player,map: &M) -> bool{
    let rotated_shape = player.shape.rotated_anticlockwise();

    'try_rotate: loop{
        match map.shape_intersects(&rotated_shape,player.pos){
            map::CellIntersection::Imprint(pos) |
            map::CellIntersection::OutOfBounds(pos) => {
                let center_x = player.pos.x + player.shape.center_x() as grid::PosAxis;
                let sign = if pos.x < center_x {1} else {-1};
                for i in 1..player.shape.width(){
                    if move_player(player,map,grid::Pos{x: i as grid::PosAxis * sign,y: 0}){
                        break 'try_rotate;
                    }
                }
            },
            _ => break 'try_rotate
        }

        return false;
    }

    {//Successfully rotated
        player.shape = rotated_shape;

        //Recalcuate fastfall shadow position when moving horizontally
        if player.settings.fastfall_shadow{
            player.shadow_pos = Some(fast_fallen_shape(&player.shape,map,player.pos));
        }
        return true;
    }
}

///Try to rotate (backwards). If this results in a collision, try to resolve this collision by
///moving in the x axis. If the collision cannot resolve, amend the rotation and return false,
///otherwise return true.
pub fn rotate_clockwise_and_resolve_player<M: MapTrait>(player: &mut Player,map: &M) -> bool{
    let rotated_shape = player.shape.rotated_clockwise();

    'try_rotate: loop{
        match map.shape_intersects(&rotated_shape,player.pos){
            map::CellIntersection::Imprint(pos) |
            map::CellIntersection::OutOfBounds(pos) => {
                let center_x = player.pos.x + player.shape.center_x() as grid::PosAxis;
                let sign = if pos.x < center_x {1} else {-1};
                for i in 1..player.shape.width(){
                    if move_player(player,map,grid::Pos{x: i as grid::PosAxis * sign,y: 0}){
                        break 'try_rotate;
                    }
                }
            },
            _ => break 'try_rotate
        }

        return false;
    }

    {//Successfully rotated
        player.shape = rotated_shape;

        //Recalcuate fastfall shadow position when moving horizontally
        if player.settings.fastfall_shadow{
            player.shadow_pos = Some(fast_fallen_shape(&player.shape,map,player.pos));
        }
        return true;
    }
}

///Returns the origin position based on the player and map
pub fn respawn_position<M: MapTrait>(shape: RotatedShape,map: &M) -> grid::Pos{
    grid::Pos{
        x: map.width() as grid::PosAxis/2 - shape.center_x() as grid::PosAxis,
        y: 0//TODO: Spawn above optionally: -(player.shape.height() as grid::PosAxis);
    }
}

///Respawns player to its origin position
///Returns whether the respawning was successful or not due to collisions.
pub fn respawn_player<M: MapTrait>(player: &mut Player,map: &M,new_shape: Shape) -> bool{
    //Select a new shape at random, setting its position to the starting position
    player.shape = RotatedShape::new(new_shape);
    player.pos = respawn_position(player.shape,map);
    if player.settings.fastfall_shadow{
        player.shadow_pos = Some(fast_fallen_shape(&player.shape,map,player.pos));
    }

    //If the new shape at the starting position also collides with another shape
    match map.shape_intersects(&player.shape,player.pos){
        map::CellIntersection::Imprint(_) => false,
        _ => true
    }
}

pub fn fast_fallen_shape<M: MapTrait>(shape: &RotatedShape,map: &M,shape_pos: grid::Pos) -> grid::Pos{
    for y in shape_pos.y .. map.height() as grid::PosAxis{
        match map.shape_intersects(&shape,grid::Pos{x: shape_pos.x,y: y+1}){
            map::CellIntersection::Imprint(_)     |
            map::CellIntersection::OutOfBounds(_) => return grid::Pos{x: shape_pos.x,y: y},
            _ => ()
        };
    }

    unreachable!()
}
